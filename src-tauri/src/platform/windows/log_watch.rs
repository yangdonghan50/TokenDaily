//! Native wait handles: no recurring disk reads while waiting for changes.
use crate::platform::log_watch::Signal;
use std::os::windows::ffi::OsStrExt;
use std::{path::Path, sync::Arc, time::Duration};
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, WAIT_OBJECT_0},
    Storage::FileSystem::{
        CreateFileW, ReadDirectoryChangesW, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OVERLAPPED,
        FILE_LIST_DIRECTORY, FILE_NOTIFY_CHANGE_DIR_NAME, FILE_NOTIFY_CHANGE_FILE_NAME,
        FILE_NOTIFY_CHANGE_LAST_WRITE, FILE_NOTIFY_CHANGE_SIZE, FILE_SHARE_DELETE, FILE_SHARE_READ,
        FILE_SHARE_WRITE, OPEN_EXISTING,
    },
    System::{
        Threading::{CreateEventW, ResetEvent, SetEvent, WaitForMultipleObjects},
        IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
    },
};

pub(crate) struct Wake(HANDLE);
// The owned kernel event is safe to signal across threads; Arc keeps it alive.
unsafe impl Send for Wake {}
unsafe impl Sync for Wake {}
impl Wake {
    pub(crate) fn new() -> Option<Arc<Self>> {
        let handle = unsafe { CreateEventW(std::ptr::null(), 0, 0, std::ptr::null()) };
        (!handle.is_null()).then(|| Arc::new(Self(handle)))
    }
    pub(crate) fn unpark(&self) {
        unsafe {
            SetEvent(self.0);
        }
    }
}
impl Drop for Wake {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

struct Change {
    handle: HANDLE,
    overlap: Box<OVERLAPPED>,
    buffer: Box<[u32; 8192]>,
    recursive: bool,
    flags: u32,
    pending: bool,
}
impl Change {
    fn start(&mut self) -> bool {
        unsafe {
            ResetEvent(self.overlap.hEvent);
            self.pending = ReadDirectoryChangesW(
                self.handle,
                self.buffer.as_mut_ptr().cast(),
                32768,
                self.recursive as i32,
                self.flags,
                std::ptr::null_mut(),
                &mut *self.overlap,
                None,
            ) != 0;
        }
        self.pending
    }
    fn complete(&mut self) {
        let mut bytes = 0;
        unsafe {
            GetOverlappedResult(self.handle, &*self.overlap, &mut bytes, 1);
        }
        self.pending = false;
    }
}
impl Drop for Change {
    fn drop(&mut self) {
        unsafe {
            if self.pending {
                CancelIoEx(self.handle, &*self.overlap);
                self.complete();
            }
            CloseHandle(self.overlap.hEvent);
            CloseHandle(self.handle);
        }
    }
}
pub(crate) struct Watch {
    wake: Arc<Wake>,
    changes: Vec<Change>,
}
impl Watch {
    pub(crate) fn new(wake: Arc<Wake>) -> Self {
        Self {
            wake,
            changes: Vec::new(),
        }
    }
    pub(crate) fn stop(&mut self) {
        self.changes.clear();
    }
    pub(crate) fn reset(&mut self, home: &Path) {
        self.stop();
        // Home itself is non-recursive, to notice a missing log directory being
        // created without watching unrelated config/cache subtrees.
        for (path, recursive) in [
            (home.to_owned(), false),
            (home.join("sessions"), true),
            (home.join("archived_sessions"), true),
        ] {
            let Ok(path) = path.canonicalize() else {
                continue;
            };
            let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
            let flags = FILE_NOTIFY_CHANGE_DIR_NAME
                | if recursive {
                    FILE_NOTIFY_CHANGE_FILE_NAME
                        | FILE_NOTIFY_CHANGE_SIZE
                        | FILE_NOTIFY_CHANGE_LAST_WRITE
                } else {
                    0
                };
            let handle = unsafe {
                CreateFileW(
                    wide.as_ptr(),
                    FILE_LIST_DIRECTORY,
                    FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                    std::ptr::null(),
                    OPEN_EXISTING,
                    FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OVERLAPPED,
                    std::ptr::null_mut(),
                )
            };
            if handle != INVALID_HANDLE_VALUE {
                let event = unsafe { CreateEventW(std::ptr::null(), 1, 0, std::ptr::null()) };
                if event.is_null() {
                    unsafe {
                        CloseHandle(handle);
                    }
                    continue;
                }
                let mut overlap: Box<OVERLAPPED> = Box::new(unsafe { std::mem::zeroed() });
                overlap.hEvent = event;
                let mut change = Change {
                    handle,
                    overlap,
                    buffer: Box::new([0; 8192]),
                    recursive,
                    flags,
                    pending: false,
                };
                if change.start() {
                    self.changes.push(change);
                }
            }
        }
    }
    pub(crate) fn wait(&mut self, duration: Duration) -> Signal {
        let handles: Vec<HANDLE> = std::iter::once(self.wake.0)
            .chain(self.changes.iter().map(|c| c.overlap.hEvent))
            .collect();
        let result = unsafe {
            WaitForMultipleObjects(
                handles.len() as u32,
                handles.as_ptr(),
                0,
                duration.as_millis().min(u32::MAX as u128 - 1) as u32,
            )
        };
        if result == WAIT_OBJECT_0 {
            return Signal::Manual;
        }
        let index = result.wrapping_sub(WAIT_OBJECT_0 + 1) as usize;
        if index < self.changes.len() {
            // Treat overflow as a change too: the collector reconciles all file
            // metadata. Rearm before scanning so concurrent writes remain queued.
            self.changes[index].complete();
            if !self.changes[index].start() {
                self.changes.remove(index);
            }
            return Signal::Changed;
        }
        if result == u32::MAX {
            std::thread::sleep(duration.min(Duration::from_secs(30)));
        }
        Signal::Timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_notifications_wake_for_flushed_writer_and_manual_refresh() {
        use std::io::Write;
        let root = std::env::temp_dir().join(format!("tokendaily-watch-{}", crate::now_ms()));
        std::fs::create_dir_all(root.join("sessions")).unwrap();
        let path = root.join("sessions/test.jsonl");
        let mut file = std::fs::File::create(path).unwrap();
        let wake = Wake::new().unwrap();
        let mut watch = Watch::new(wake.clone());
        watch.reset(&root);
        assert_eq!(
            watch.changes.len(),
            2,
            "native directory subscriptions must be available"
        );
        assert_eq!(watch.wait(Duration::from_millis(20)), Signal::Timeout);
        file.write_all(b"test\n").unwrap();
        file.flush().unwrap();
        file.sync_data().unwrap();
        assert_eq!(watch.wait(Duration::from_secs(3)), Signal::Changed);
        wake.unpark();
        assert_eq!(watch.wait(Duration::from_secs(1)), Signal::Manual);
        drop(file);
        watch.stop();
        std::fs::remove_dir_all(root).unwrap();
    }
}

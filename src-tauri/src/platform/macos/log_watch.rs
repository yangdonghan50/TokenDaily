//! FSEvents wakes the shared incremental collector; it never reads log contents.
//! Timed reconciliation remains authoritative when events are lost or unavailable.
use std::{
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

use crate::platform::log_watch::Signal;
use notify::{Event, FsEventWatcher, RecursiveMode, Watcher};

#[derive(Default)]
struct Pending {
    generation: u64,
    manual: bool,
    changed: bool,
    failed: bool,
}

pub(crate) struct Wake {
    pending: Mutex<Pending>,
    ready: Condvar,
}

impl Wake {
    pub(crate) fn new() -> Option<Arc<Self>> {
        Some(Arc::new(Self {
            pending: Mutex::new(Pending::default()),
            ready: Condvar::new(),
        }))
    }

    pub(crate) fn unpark(&self) {
        self.pending.lock().expect("wake lock").manual = true;
        self.ready.notify_one();
    }

    fn receive(&self, generation: u64, home: &Path, event: notify::Result<Event>) {
        // Keep just two wake bits: bursts cannot build an unbounded event queue.
        // Never retain paths or OS error strings (which may contain private paths).
        let (changed, failed) = match event {
            Ok(event) => (
                event.need_rescan()
                    || event.paths.iter().any(|path| {
                        path == home
                            || path.starts_with(home.join("sessions"))
                            || path.starts_with(home.join("archived_sessions"))
                    }),
                false,
            ),
            Err(_) => (true, true),
        };
        let mut pending = self.pending.lock().expect("wake lock");
        if pending.generation == generation && !pending.failed && changed {
            pending.changed = true;
            pending.failed |= failed;
            self.ready.notify_one();
        }
    }
}

// Include inode identity so removal/recreation at the same path rebinds the stream.
#[derive(PartialEq)]
struct Root {
    path: PathBuf,
    device: u64,
    inode: u64,
}
fn roots(home: &Path) -> std::io::Result<Vec<Root>> {
    let mut result = Vec::new();
    for path in [
        home.to_path_buf(),
        home.join("sessions"),
        home.join("archived_sessions"),
    ] {
        match fs::symlink_metadata(&path) {
            Ok(meta) if meta.is_dir() => result.push(Root {
                path,
                device: meta.dev(),
                inode: meta.ino(),
            }),
            // Match the collector: do not follow log directory symlinks.
            Ok(_) if path != home => (),
            Err(error) if path != home && error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(error),
            Ok(_) => return Err(std::io::Error::other("not a directory")),
        }
    }
    Ok(result)
}

pub(crate) struct Watch {
    wake: Arc<Wake>,
    native: Option<FsEventWatcher>,
    home: Option<PathBuf>,
    roots: Vec<Root>,
}

impl Watch {
    pub(crate) fn new(wake: Arc<Wake>) -> Self {
        Self {
            wake,
            native: None,
            home: None,
            roots: Vec::new(),
        }
    }

    pub(crate) fn warning(&self) -> Option<&'static str> {
        self.wake
            .pending
            .lock()
            .expect("wake lock")
            .failed
            .then_some("目录变化监听暂不可用，正在使用定时补查")
    }

    pub(crate) fn reset(&mut self, home: &Path) {
        let binding = home.canonicalize().ok().and_then(|home| {
            // notify's macOS path API requires UTF-8. Reject instead of panicking.
            home.to_str()?;
            roots(&home).ok().map(|roots| (home, roots))
        });
        if let Some((home, roots)) = &binding {
            if self.home.as_ref() == Some(home)
                && self.roots == *roots
                && self.native.is_some()
                && self.warning().is_none()
            {
                return;
            }
        }
        self.stop();
        let Some((home, roots)) = binding else {
            self.wake.pending.lock().expect("wake lock").failed = true;
            return;
        };
        let generation = self.wake.pending.lock().expect("wake lock").generation;
        let wake = self.wake.clone();
        let callback_home = home.clone();
        let native = FsEventWatcher::new(
            move |event| {
                wake.receive(generation, &callback_home, event);
            },
            notify::Config::default(),
        )
        .and_then(|mut native| {
            for root in &roots {
                native.watch(
                    &root.path,
                    if root.path == home {
                        RecursiveMode::NonRecursive
                    } else {
                        RecursiveMode::Recursive
                    },
                )?;
            }
            Ok(native)
        });
        self.home = Some(home);
        match native {
            Ok(native) => {
                self.native = Some(native);
                self.roots = roots;
            }
            Err(_) => self.wake.pending.lock().expect("wake lock").failed = true,
        }
    }

    pub(crate) fn stop(&mut self) {
        {
            let mut pending = self.wake.pending.lock().expect("wake lock");
            pending.generation = pending.generation.wrapping_add(1);
            pending.changed = false;
            pending.failed = false;
            // A manual connect/disconnect wake must survive a stream replacement.
        }
        // Drop joins the FSEvents run loop. Never hold its callback's mutex here.
        self.native = None;
        self.home = None;
        self.roots.clear();
    }

    pub(crate) fn wait(&mut self, duration: Duration) -> Signal {
        let pending = self.wake.pending.lock().expect("wake lock");
        let (mut pending, _) = self
            .wake
            .ready
            .wait_timeout_while(pending, duration, |value| !value.manual && !value.changed)
            .expect("wake wait");
        let signal = if pending.manual {
            pending.manual = false;
            Signal::Manual
        } else if pending.changed {
            pending.changed = false;
            Signal::Changed
        } else {
            Signal::Timeout
        };
        let failed = pending.failed;
        drop(pending);
        if signal == Signal::Changed && !failed {
            // A missing log directory may have just appeared. Subscribe before the
            // next scan; otherwise subsequent appends would wait for the timer.
            if let Some(home) = self.home.clone() {
                self.reset(&home);
            }
        }
        signal
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
#[path = "log_watch_tests.rs"]
mod tests;

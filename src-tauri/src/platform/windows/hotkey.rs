//! RegisterHotKey owns only the selected accelerator; no keyboard hook.
use crate::floating_hotkey::{Registry, Shortcut};
use std::{ptr::null_mut, sync::mpsc};
use windows_sys::Win32::{
    System::Threading::GetCurrentThreadId,
    UI::{
        Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_NOREPEAT},
        WindowsAndMessaging::{
            GetMessageW, PeekMessageW, PostThreadMessageW, MSG, PM_NOREMOVE, WM_APP, WM_HOTKEY,
            WM_QUIT,
        },
    },
};
const UPDATE: u32 = WM_APP + 73;

type Request = (Option<Shortcut>, mpsc::Sender<Result<(), String>>);
pub(crate) struct Driver {
    sender: mpsc::Sender<Request>,
    thread: u32,
}
impl Driver {
    pub(crate) fn new(toggle: impl Fn() + Send + 'static) -> Result<Self, String> {
        let (sender, receiver) = mpsc::channel::<Request>();
        let (ready, started) = mpsc::channel();
        std::thread::Builder::new()
            .name("floating-hotkey".into())
            .spawn(move || unsafe {
                let mut message: MSG = std::mem::zeroed();
                PeekMessageW(&mut message, null_mut(), 0, 0, PM_NOREMOVE);
                let _ = ready.send(GetCurrentThreadId());
                let mut current: Option<Shortcut> = None;
                let mut id = 1;
                while GetMessageW(&mut message, null_mut(), 0, 0) > 0 {
                    if message.message == UPDATE {
                        while let Ok((next, reply)) = receiver.try_recv() {
                            if current == next {
                                let _ = reply.send(Ok(()));
                                continue;
                            }
                            let next_id = if id == 1 { 2 } else { 1 };
                            if let Some(key) = next {
                                if RegisterHotKey(
                                    null_mut(),
                                    next_id,
                                    key.modifiers | MOD_NOREPEAT,
                                    key.key,
                                ) == 0
                                {
                                    let _ = reply
                                        .send(Err("快捷键被系统或其他应用占用，请更换按键".into()));
                                    continue;
                                }
                            }
                            if current.is_some() {
                                UnregisterHotKey(null_mut(), id);
                            }
                            current = next;
                            id = next_id;
                            let _ = reply.send(Ok(()));
                        }
                    } else if message.message == WM_HOTKEY
                        && current.is_some()
                        && message.wParam == id as usize
                    {
                        toggle();
                    }
                }
                if current.is_some() {
                    UnregisterHotKey(null_mut(), id);
                }
            })
            .map_err(|_| "无法启动快捷键服务".to_string())?;
        Ok(Self {
            sender,
            thread: started
                .recv()
                .map_err(|_| "快捷键服务启动失败".to_string())?,
        })
    }
}
impl Registry for Driver {
    fn replace(&mut self, value: Option<Shortcut>) -> Result<(), String> {
        let (reply, result) = mpsc::channel();
        self.sender
            .send((value, reply))
            .map_err(|_| "快捷键服务已停止".to_string())?;
        if unsafe { PostThreadMessageW(self.thread, UPDATE, 0, 0) } == 0 {
            return Err("快捷键服务无法响应".into());
        }
        result.recv().map_err(|_| "快捷键注册中断".to_string())?
    }
}
impl Drop for Driver {
    fn drop(&mut self) {
        unsafe {
            PostThreadMessageW(self.thread, WM_QUIT, 0, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "explicit Windows registration check; temporarily reserves Ctrl+Alt+Shift+F23/F24"]
    fn native_conflict_keeps_original_and_unregister_releases() {
        let mut first = Driver::new(|| {}).unwrap();
        let mut second = Driver::new(|| {}).unwrap();
        let a = Some(Shortcut {
            key: 134,
            modifiers: 7,
        });
        let b = Some(Shortcut {
            key: 135,
            modifiers: 7,
        });
        first.replace(a).unwrap();
        second.replace(b).unwrap();
        assert!(first.replace(b).is_err());
        assert!(
            second.replace(a).is_err(),
            "failed replacement must keep original registered"
        );
        first.replace(None).unwrap();
        second.replace(a).unwrap();
        second.replace(None).unwrap();
        first.replace(a).unwrap();
        first.replace(None).unwrap();
    }
}

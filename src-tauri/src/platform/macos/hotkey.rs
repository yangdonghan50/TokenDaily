//! Exclusive Carbon hotkeys, dispatched only on the main thread. No input hook.
use crate::floating_hotkey::{Registry, Shortcut};
use std::{ffi::c_void, ptr::null_mut, sync::mpsc};

type Ref = *mut c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct HotKeyId {
    signature: u32,
    id: u32,
}
#[repr(C)]
struct EventType {
    class: u32,
    kind: u32,
}
const KEYBOARD: u32 = u32::from_be_bytes(*b"keyb");
const SIGNATURE: u32 = u32::from_be_bytes(*b"TDhk");
#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn GetEventDispatcherTarget() -> Ref;
    fn InstallEventHandler(
        target: Ref,
        handler: unsafe extern "C" fn(Ref, Ref, Ref) -> i32,
        count: usize,
        types: *const EventType,
        data: Ref,
        result: *mut Ref,
    ) -> i32;
    fn RemoveEventHandler(handler: Ref) -> i32;
    fn RegisterEventHotKey(
        key: u32,
        modifiers: u32,
        id: HotKeyId,
        target: Ref,
        options: u32,
        result: *mut Ref,
    ) -> i32;
    fn UnregisterEventHotKey(key: Ref) -> i32;
    fn GetEventKind(event: Ref) -> u32;
    fn GetEventParameter(
        event: Ref,
        name: u32,
        kind: u32,
        actual_kind: *mut u32,
        size: usize,
        actual_size: *mut usize,
        result: Ref,
    ) -> i32;
    fn pthread_main_np() -> i32;
}

// Stored key IDs retain the existing cross-platform contract. Convert physical
// KeyboardEvent.code keys to Apple virtual codes, never interpret typed text.
fn native_key(key: u32) -> Result<u32, String> {
    let code = match key {
        65 => 0,
        83 => 1,
        68 => 2,
        70 => 3,
        72 => 4,
        71 => 5,
        90 => 6,
        88 => 7,
        67 => 8,
        86 => 9,
        66 => 11,
        81 => 12,
        87 => 13,
        69 => 14,
        82 => 15,
        89 => 16,
        84 => 17,
        49 => 18,
        50 => 19,
        51 => 20,
        52 => 21,
        54 => 22,
        53 => 23,
        187 => 24,
        57 => 25,
        55 => 26,
        189 => 27,
        56 => 28,
        48 => 29,
        221 => 30,
        79 => 31,
        85 => 32,
        219 => 33,
        73 => 34,
        80 => 35,
        13 => 36,
        76 => 37,
        74 => 38,
        222 => 39,
        75 => 40,
        186 => 41,
        220 => 42,
        188 => 43,
        191 => 44,
        78 => 45,
        77 => 46,
        190 => 47,
        9 => 48,
        32 => 49,
        192 => 50,
        8 => 51,
        27 => 53,
        110 => 65,
        106 => 67,
        107 => 69,
        111 => 75,
        109 => 78,
        96 => 82,
        97 => 83,
        98 => 84,
        99 => 85,
        100 => 86,
        101 => 87,
        102 => 88,
        103 => 89,
        104 => 91,
        105 => 92,
        116 => 96,
        117 => 97,
        118 => 98,
        114 => 99,
        119 => 100,
        120 => 101,
        122 => 103,
        124 => 105,
        127 => 106,
        125 => 107,
        121 => 109,
        126 => 113,
        45 => 114,
        36 => 115,
        33 => 116,
        46 => 117,
        115 => 118,
        35 => 119,
        113 => 120,
        34 => 121,
        112 => 122,
        37 => 123,
        39 => 124,
        40 => 125,
        38 => 126,
        128 => 64,
        129 => 79,
        130 => 80,
        131 => 90,
        _ => return Err("Mac 不支持这个按键，请使用字母、数字或 F1–F11 等按键".into()),
    };
    Ok(code)
}
fn native_modifiers(modifiers: u32) -> u32 {
    (if modifiers & 8 != 0 { 1 << 8 } else { 0 })
        | (if modifiers & 4 != 0 { 1 << 9 } else { 0 })
        | (if modifiers & 1 != 0 { 1 << 11 } else { 0 })
        | (if modifiers & 2 != 0 { 1 << 12 } else { 0 })
}
struct Native {
    handler: Ref,
    key: Ref,
    current: Option<Shortcut>,
    id: u32,
    held: bool,
    toggle: Box<dyn Fn() + Send>,
}
unsafe extern "C" fn event_handler(_: Ref, event: Ref, data: Ref) -> i32 {
    // Carbon invokes this on the main dispatcher; removal precedes freeing data.
    let state = unsafe { &mut *(data as *mut Native) };
    let mut id = HotKeyId::default();
    let result = unsafe {
        GetEventParameter(
            event,
            u32::from_be_bytes(*b"----"),
            u32::from_be_bytes(*b"hkid"),
            null_mut(),
            8,
            null_mut(),
            &mut id as *mut _ as Ref,
        )
    };
    if result != 0 || id.signature != SIGNATURE || id.id != state.id || state.current.is_none() {
        return -9874;
    }
    match unsafe { GetEventKind(event) } {
        5 if !state.held => {
            state.held = true;
            (state.toggle)();
        }
        6 => state.held = false,
        _ => {}
    }
    0
}
impl Native {
    fn install(toggle: Box<dyn Fn() + Send>) -> Result<Box<Self>, String> {
        let mut state = Box::new(Self {
            handler: null_mut(),
            key: null_mut(),
            current: None,
            id: 0,
            held: false,
            toggle,
        });
        let types = [
            EventType {
                class: KEYBOARD,
                kind: 5,
            },
            EventType {
                class: KEYBOARD,
                kind: 6,
            },
        ];
        let result = unsafe {
            InstallEventHandler(
                GetEventDispatcherTarget(),
                event_handler,
                2,
                types.as_ptr(),
                &mut *state as *mut _ as Ref,
                &mut state.handler,
            )
        };
        if result != 0 {
            return Err("无法启动 Mac 快捷键服务".into());
        }
        Ok(state)
    }
    fn replace(&mut self, next: Option<Shortcut>) -> Result<(), String> {
        if next == self.current {
            return Ok(());
        }
        let id = self
            .id
            .checked_add(1)
            .ok_or("快捷键标识已用尽，请重启应用")?;
        let mut replacement = null_mut();
        if let Some(shortcut) = next {
            let key = native_key(shortcut.key)?;
            let result = unsafe {
                RegisterEventHotKey(
                    key,
                    native_modifiers(shortcut.modifiers),
                    HotKeyId {
                        signature: SIGNATURE,
                        id,
                    },
                    GetEventDispatcherTarget(),
                    1,
                    &mut replacement,
                )
            };
            if result != 0 {
                return Err("快捷键被系统或其他应用占用，请更换按键".into());
            }
        }
        if !self.key.is_null() && unsafe { UnregisterEventHotKey(self.key) } != 0 {
            if !replacement.is_null() {
                unsafe {
                    UnregisterEventHotKey(replacement);
                }
            }
            return Err("原快捷键释放失败，已保留原绑定".into());
        }
        self.key = replacement;
        self.current = next;
        self.id = id;
        self.held = false;
        Ok(())
    }
}
impl Drop for Native {
    fn drop(&mut self) {
        unsafe {
            if !self.key.is_null() {
                UnregisterEventHotKey(self.key);
            }
            if !self.handler.is_null() {
                RemoveEventHandler(self.handler);
            }
        }
    }
}
pub(crate) fn on_main<T: Send + 'static>(
    app: &tauri::AppHandle,
    task: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    if unsafe { pthread_main_np() } != 0 {
        return task();
    }
    let (send, receive) = mpsc::channel();
    app.run_on_main_thread(move || {
        let _ = send.send(task());
    })
    .map_err(|_| "无法调用 Mac 快捷键服务")?;
    receive.recv().map_err(|_| "Mac 快捷键服务已停止")?
}
pub(crate) struct Driver {
    app: tauri::AppHandle,
    address: usize,
}
impl Driver {
    pub(crate) fn new(
        app: tauri::AppHandle,
        toggle: impl Fn() + Send + 'static,
    ) -> Result<Self, String> {
        let address = on_main(&app, move || {
            Native::install(Box::new(toggle)).map(|state| Box::into_raw(state) as usize)
        })?;
        Ok(Self { app, address })
    }
}
impl Registry for Driver {
    fn replace(&mut self, value: Option<Shortcut>) -> Result<(), String> {
        let address = self.address;
        on_main(&self.app, move || unsafe {
            (&mut *(address as *mut Native)).replace(value)
        })
    }
}
impl Drop for Driver {
    fn drop(&mut self) {
        let address = self.address;
        let remove = move || unsafe {
            drop(Box::from_raw(address as *mut Native));
        };
        if unsafe { pthread_main_np() } != 0 {
            remove();
        } else {
            let _ = self.app.run_on_main_thread(remove);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_physical_keys_and_command_option_without_guessing() {
        assert_eq!(native_key(75).unwrap(), 40);
        assert_eq!(native_key(119).unwrap(), 100);
        assert_eq!(native_modifiers(15), 0x1b00);
        assert_eq!(native_modifiers(8), 0x100);
        for key in [0, 91, 123, 135] {
            assert!(native_key(key).is_err());
        }
    }
    #[test]
    #[ignore = "explicit isolated Carbon registration/conflict/release test"]
    fn native_conflict_keeps_original_and_drop_releases() {
        let mut a = Native::install(Box::new(|| {})).unwrap();
        let mut b = Native::install(Box::new(|| {})).unwrap();
        let first = Some(Shortcut {
            key: 130,
            modifiers: 7,
        });
        let second = Some(Shortcut {
            key: 131,
            modifiers: 7,
        });
        a.replace(first).unwrap();
        b.replace(second).unwrap();
        assert!(a.replace(second).is_err());
        assert!(b.replace(first).is_err());
        a.replace(None).unwrap();
        b.replace(first).unwrap();
        drop(b);
        a.replace(first).unwrap();
    }
}

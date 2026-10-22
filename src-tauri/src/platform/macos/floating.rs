//! Local AppKit event monitor: only an explicitly initiated floating-window drag.
//! No global input hook, accessibility permission, or stored pointer history.
use super::floating_geometry::{drag_frame, escape_collapses, logical_point, Rect, Side};
use block2::RcBlock;
use objc2::{class, msg_send, rc::Retained, runtime::AnyObject};
use serde::Serialize;
use std::{cell::RefCell, ptr::NonNull, sync::atomic::Ordering};
use tauri::{Emitter, LogicalPosition, Manager};
struct Drag {
    origin: Rect,
    start: (f64, f64),
    side: Option<Side>,
    moved: bool,
}
struct Monitor {
    token: Retained<AnyObject>,
    drag: Option<Drag>,
}
impl Drop for Monitor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![class!(NSEvent), removeMonitor: &*self.token];
        }
    }
}
thread_local! {static MONITOR:RefCell<Option<Monitor>>=const {RefCell::new(None)};}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct End {
    moved: bool,
    cancelled: bool,
    snap_side: Option<Side>,
}
pub(crate) fn primary_down() -> bool {
    unsafe {
        let buttons: usize = msg_send![class!(NSEvent), pressedMouseButtons];
        buttons & 1 != 0
    }
}

pub(crate) fn install(window: tauri::WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
    let callback = RcBlock::new(move |event: NonNull<AnyObject>| -> *mut AnyObject {
        let kind: usize = unsafe { msg_send![event.as_ref(), type] };
        MONITOR.with(|slot| {
            let mut guard = slot.borrow_mut();
            let Some(monitor) = guard.as_mut() else {
                return event.as_ptr();
            };
            let escape = kind == 10
                && unsafe {
                    let key: u16 = msg_send![event.as_ref(), keyCode];
                    key == 53
                };
            let Some(drag) = monitor.drag.as_mut() else {
                if escape && panel_escape(&window) {
                    let _ = window.emit("floating-escape", ());
                    return std::ptr::null_mut();
                }
                return event.as_ptr();
            };
            let cancelled = escape || !window.is_visible().unwrap_or(false);
            if kind == 2 || cancelled {
                if cancelled {
                    let _ = window.set_position(LogicalPosition::new(drag.origin.x, drag.origin.y));
                }
                let result = End {
                    moved: drag.moved,
                    cancelled,
                    snap_side: if cancelled { None } else { drag.side },
                };
                monitor.drag = None;
                let _ = window.emit("floating-drag-ended", result);
                return if escape {
                    std::ptr::null_mut()
                } else {
                    event.as_ptr()
                };
            }
            if kind != 6 {
                return event.as_ptr();
            }
            let Ok(cursor) = cursor_logical(&window) else {
                return event.as_ptr();
            };
            let Ok(Some(screen)) = window.monitor_from_point(cursor.0, cursor.1) else {
                return event.as_ptr();
            };
            let scale = screen.scale_factor();
            let area = screen.work_area();
            let area = Rect {
                x: area.position.x as f64 / scale,
                y: area.position.y as f64 / scale,
                width: area.size.width as f64 / scale,
                height: area.size.height as f64 / scale,
            };
            let enabled = window
                .state::<crate::AppState>()
                .floating_edge
                .load(Ordering::SeqCst);
            let (next, side) = drag_frame(drag.origin, drag.start, cursor, area, enabled);
            if window
                .set_position(LogicalPosition::new(next.x, next.y))
                .is_err()
            {
                let _ = window.emit(
                    "floating-drag-ended",
                    End {
                        moved: drag.moved,
                        cancelled: true,
                        snap_side: None,
                    },
                );
                monitor.drag = None;
                return event.as_ptr();
            }
            drag.moved |= next.x != drag.origin.x || next.y != drag.origin.y;
            if drag.side != side && drag.origin.width == drag.origin.height {
                let _ = window.emit("floating-drag-snap", side);
            }
            drag.side = side;
            // Let WebKit receive the button sequence so its native mouse state
            // is released. The shared press controller already suppresses drag clicks.
            event.as_ptr()
        })
    });
    // NSEventMaskLeftMouseDragged | LeftMouseUp | KeyDown, AppKit retains the block.
    let token: Option<Retained<AnyObject>> = unsafe {
        msg_send![class!(NSEvent),addLocalMonitorForEventsMatchingMask:(1usize<<6 | 1usize<<2 | 1usize<<10),handler:&*callback]
    };
    let token = token.ok_or("无法安装悬浮窗本地拖动监听")?;
    MONITOR.with(|slot| *slot.borrow_mut() = Some(Monitor { token, drag: None }));
    Ok(())
}

pub(crate) async fn start(window: tauri::WebviewWindow) -> Result<(), String> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    let handle = window.clone();
    window
        .run_on_main_thread(move || {
            let result = (|| {
                if !primary_down() || !handle.is_visible().map_err(|_| "无法读取悬浮窗状态")?
                {
                    return Err("拖动已结束，请按住后重试");
                }
                let position = handle.outer_position().map_err(|_| "无法读取悬浮窗位置")?;
                let size = handle.outer_size().map_err(|_| "无法读取悬浮窗大小")?;
                let pointer = cursor_logical(&handle)?;
                let scale = handle.scale_factor().map_err(|_| "无法读取窗口缩放")?;
                MONITOR.with(|slot| {
                    let mut guard = slot.borrow_mut();
                    let monitor = guard.as_mut().ok_or("悬浮窗拖动尚未初始化")?;
                    monitor.drag = Some(Drag {
                        origin: Rect {
                            x: position.x as f64 / scale,
                            y: position.y as f64 / scale,
                            width: size.width as f64 / scale,
                            height: size.height as f64 / scale,
                        },
                        start: pointer,
                        side: None,
                        moved: false,
                    });
                    Ok(())
                })
            })();
            let _ = tx.send(result.map_err(str::to_owned));
        })
        .map_err(|_| "无法调度窗口拖动")?;
    tauri::async_runtime::spawn_blocking(move || {
        rx.recv().map_err(|_| "窗口拖动任务已结束".to_owned())?
    })
    .await
    .map_err(|_| "窗口拖动任务失败".to_owned())?
}

fn panel_escape(window: &tauri::WebviewWindow) -> bool {
    let frame = (|| {
        let scale = window.scale_factor().ok()?;
        let position = window.outer_position().ok()?;
        let size = window.outer_size().ok()?;
        Some(Rect {
            x: position.x as f64 / scale,
            y: position.y as f64 / scale,
            width: size.width as f64 / scale,
            height: size.height as f64 / scale,
        })
    })();
    match (frame, cursor_logical(window)) {
        (Some(frame), Ok(cursor)) => {
            escape_collapses(window.is_visible().unwrap_or(false), frame, cursor, false)
        }
        _ => false,
    }
}

fn cursor_logical(window: &tauri::WebviewWindow) -> Result<(f64, f64), &'static str> {
    let pointer = window.cursor_position().map_err(|_| "无法读取鼠标位置")?;
    let scale = window
        .primary_monitor()
        .map_err(|_| "无法读取主显示器")?
        .ok_or("主显示器不可用")?
        .scale_factor();
    Ok(logical_point((pointer.x, pointer.y), scale))
}

pub(crate) fn pointer_for_window(
    window: &tauri::WebviewWindow,
) -> Result<tauri::PhysicalPosition<f64>, String> {
    let (x, y) = cursor_logical(window).map_err(str::to_owned)?;
    let scale = window.scale_factor().map_err(|_| "无法读取窗口缩放")?;
    Ok(tauri::PhysicalPosition::new(x * scale, y * scale))
}

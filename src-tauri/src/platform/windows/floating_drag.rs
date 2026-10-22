//! Window-local live snapping and drag completion. No global hooks or stored coordinates.
#[cfg(target_os = "windows")]
mod windows {
    use serde::Serialize;
    use tauri::{Emitter, Manager};
    use windows_sys::Win32::{
        Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
        Graphics::Gdi::{GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST},
        UI::{
            Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE},
            Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
            WindowsAndMessaging::{
                GetCursorPos, GetSystemMetrics, GetWindowRect, WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE,
                WM_MOVING, WM_NCDESTROY,
            },
        },
    };

    const SUBCLASS_ID: usize = 0x54444c59;

    fn drag_rectangle(origin: RECT, start: POINT, cursor: POINT, proposed: RECT) -> RECT {
        // WM_MOVING can rebase its rectangle after we snap it. Keep the original
        // grab point throughout this drag so a correction cannot change contact.
        let left = origin.left + cursor.x - start.x;
        let top = origin.top + cursor.y - start.y;
        RECT {
            left,
            top,
            right: left + proposed.right - proposed.left,
            bottom: top + proposed.bottom - proposed.top,
        }
    }

    fn snap_during_drag(cursor: POINT, area: RECT, proposed: RECT, enabled: bool) -> Option<RECT> {
        if !enabled || cursor.y < area.top || cursor.y >= area.bottom {
            return None;
        }
        let width = proposed.right - proposed.left;
        let height = proposed.bottom - proposed.top;
        // The 56px native ball contains a 48px visible circle with 4px padding.
        // Use its scaled visible bounds, not the pointer or transparent window edge.
        let inset = if width == height { (width + 7) / 14 } else { 0 };
        let left = if proposed.left + inset <= area.left {
            area.left
        } else if proposed.right - inset >= area.right {
            (area.right - width).max(area.left)
        } else {
            return None;
        };
        let top = proposed
            .top
            .clamp(area.top, (area.bottom - height).max(area.top));
        Some(RECT {
            left,
            top,
            right: left + width,
            bottom: top + height,
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn rect(left: i32, top: i32, right: i32, bottom: i32) -> RECT {
            RECT {
                left,
                top,
                right,
                bottom,
            }
        }

        #[test]
        fn held_drag_snaps_on_contact_without_waiting_for_release() {
            let area = rect(0, 0, 1920, 1020);
            let proposed = rect(1855, 400, 1925, 470); // 5px transparent inset, circle touches right.
            let snapped =
                snap_during_drag(POINT { x: 1890, y: 435 }, area, proposed, true).unwrap();
            assert_eq!(
                (snapped.left, snapped.right, snapped.top, snapped.bottom),
                (1850, 1920, 400, 470)
            );
            let snapped =
                snap_during_drag(POINT { x: 30, y: 435 }, area, rect(-5, 400, 65, 470), true)
                    .unwrap();
            assert_eq!((snapped.left, snapped.right), (0, 70));
        }

        #[test]
        fn nearby_retreat_disabled_and_outside_work_area_do_not_snap() {
            let area = rect(0, 0, 1920, 1020);
            let proposed = rect(1840, 400, 1910, 470);
            for x in [35, 1885, 1918, 1] {
                assert!(snap_during_drag(POINT { x, y: 435 }, area, proposed, true).is_none());
            }
            assert!(snap_during_drag(POINT { x: 1919, y: 435 }, area, proposed, false).is_none());
            assert!(snap_during_drag(POINT { x: 1919, y: 1020 }, area, proposed, true).is_none());
            // A one-pixel visible gap must not revive proximity snapping.
            assert!(snap_during_drag(
                POINT { x: 1889, y: 435 },
                area,
                rect(1854, 400, 1924, 470),
                true
            )
            .is_none());
            assert!(
                snap_during_drag(POINT { x: 31, y: 435 }, area, rect(-4, 400, 66, 470), true)
                    .is_none()
            );
        }

        #[test]
        fn held_snap_preserves_size_and_fits_negative_monitor_work_area() {
            let area = rect(-1920, -200, 0, 820);
            let snapped =
                snap_during_drag(POINT { x: -30, y: 819 }, area, rect(-65, 780, 5, 850), true)
                    .unwrap();
            assert_eq!(
                (snapped.left, snapped.top, snapped.right, snapped.bottom),
                (-70, 750, 0, 820)
            );
        }

        #[test]
        fn visible_ball_contact_scales_and_does_not_require_pointer_at_boundary() {
            let area = rect(0, 0, 1920, 1020);
            for (size, inset) in [(56, 4), (70, 5), (84, 6), (112, 8)] {
                let proposed = rect(1920 + inset - size, 400, 1920 + inset, 400 + size);
                let cursor = POINT {
                    x: proposed.left + size / 2,
                    y: 410,
                };
                assert!(cursor.x < 1919);
                assert_eq!(
                    snap_during_drag(cursor, area, proposed, true)
                        .unwrap()
                        .right,
                    1920
                );
            }
        }

        #[test]
        fn repeated_contact_uses_original_grab_point_after_windows_adjusts_rectangle() {
            let area = rect(0, 0, 1920, 1020);
            let origin = rect(800, 400, 870, 470);
            let start = POINT { x: 847, y: 435 };
            for (x, y, expected_side) in [
                (1885, 435, None),
                (1905, 435, Some(1850)),
                (1905, 455, Some(1850)),
                (1600, 435, None),
                (1905, 435, Some(1850)),
                (20, 435, Some(0)),
                (300, 435, None),
                (20, 435, Some(0)),
            ] {
                let cursor = POINT { x, y };
                // Windows may propose a previously corrected rectangle. It must
                // not change the grab offset or erase a still-held contact.
                let proposed = rect(1850, 400, 1920, 470);
                let desired = drag_rectangle(origin, start, cursor, proposed);
                let snapped = snap_during_drag(cursor, area, desired, true);
                assert_eq!(snapped.map(|r| r.left), expected_side);
                assert_eq!(desired.top, y - 35);
            }
        }
    }
    struct DragState {
        window: tauri::WebviewWindow,
        origin: Option<(RECT, POINT)>,
        snapped: Option<(DockSide, POINT, RECT)>,
    }

    #[derive(Clone, Copy, PartialEq, Serialize)]
    #[serde(rename_all = "lowercase")]
    enum DockSide {
        Left,
        Right,
    }

    #[derive(Clone, Serialize)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[derive(Clone, Serialize)]
    struct Area {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    }
    #[derive(Clone, Serialize)]
    struct DragEnd {
        pointer: Point,
        area: Area,
        moved: bool,
        cancelled: bool,
        #[serde(rename = "snapSide")]
        snap_side: Option<DockSide>,
    }

    unsafe extern "system" fn callback(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        id: usize,
        data: usize,
    ) -> LRESULT {
        if message == WM_NCDESTROY {
            RemoveWindowSubclass(hwnd, Some(callback), id);
            drop(Box::from_raw(data as *mut DragState));
            return DefSubclassProc(hwnd, message, wparam, lparam);
        }
        // Access the state only before DefSubclassProc: it may dispatch nested messages.
        let state = &mut *(data as *mut DragState);
        if message == WM_ENTERSIZEMOVE {
            let mut rect: RECT = std::mem::zeroed();
            let mut cursor: POINT = std::mem::zeroed();
            state.origin = (GetWindowRect(hwnd, &mut rect) != 0 && GetCursorPos(&mut cursor) != 0)
                .then_some((rect, cursor));
            state.snapped = None;
        } else if message == WM_MOVING && state.origin.is_some() && lparam != 0 {
            // A queued move delivered after mouseup must not erase the last held
            // snap. Respect Windows' swapped primary/secondary mouse buttons.
            if GetAsyncKeyState(if GetSystemMetrics(23) != 0 { 2 } else { 1 }) >= 0 {
                return DefSubclassProc(hwnd, message, wparam, lparam);
            }
            let previous_side = state.snapped.map(|(side, _, _)| side);
            state.snapped = None;
            let enabled = state
                .window
                .try_state::<crate::AppState>()
                .is_some_and(|app| app.floating_edge.load(std::sync::atomic::Ordering::SeqCst));
            let mut cursor: POINT = std::mem::zeroed();
            let mut monitor: MONITORINFO = std::mem::zeroed();
            monitor.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if enabled
                && GetCursorPos(&mut cursor) != 0
                && GetMonitorInfoW(
                    MonitorFromPoint(cursor, MONITOR_DEFAULTTONEAREST),
                    &mut monitor,
                ) != 0
            {
                // Windows applies this drag rectangle while the button is still held.
                // Preserve the grab point even when leaving and re-entering an edge.
                let proposed = lparam as *mut RECT;
                let (origin, start) = state.origin.unwrap();
                *proposed = drag_rectangle(origin, start, cursor, *proposed);
                if let Some(snapped) = snap_during_drag(cursor, monitor.rcWork, *proposed, enabled)
                {
                    *proposed = snapped;
                    let side = if snapped.left == monitor.rcWork.left {
                        DockSide::Left
                    } else {
                        DockSide::Right
                    };
                    state.snapped = Some((side, cursor, monitor.rcWork));
                    if origin.right - origin.left == origin.bottom - origin.top
                        && previous_side != Some(side)
                    {
                        let _ = state.window.emit("floating-drag-snap", Some(side));
                    }
                    return 1;
                }
                if previous_side.is_some() {
                    let _ = state
                        .window
                        .emit("floating-drag-snap", Option::<DockSide>::None);
                }
                return 1;
            }
            if previous_side.is_some() {
                let _ = state
                    .window
                    .emit("floating-drag-snap", Option::<DockSide>::None);
            }
        } else if message == WM_EXITSIZEMOVE {
            if let Some((origin, _)) = state.origin.take() {
                let mut cursor: POINT = std::mem::zeroed();
                let mut rect: RECT = std::mem::zeroed();
                let mut monitor: MONITORINFO = std::mem::zeroed();
                monitor.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
                if GetCursorPos(&mut cursor) != 0
                    && GetWindowRect(hwnd, &mut rect) != 0
                    && GetMonitorInfoW(
                        MonitorFromPoint(cursor, MONITOR_DEFAULTTONEAREST),
                        &mut monitor,
                    ) != 0
                {
                    // Commit the last live snap even if the pointer has already
                    // moved away after mouseup. Retreat during dragging clears it.
                    let (snap_side, cursor, area) = match state.snapped.take() {
                        Some((side, pointer, area)) => (Some(side), pointer, area),
                        None => (None, cursor, monitor.rcWork),
                    };
                    let _ = state.window.emit(
                        "floating-drag-ended",
                        DragEnd {
                            pointer: Point {
                                x: cursor.x,
                                y: cursor.y,
                            },
                            area: Area {
                                x: area.left,
                                y: area.top,
                                width: area.right - area.left,
                                height: area.bottom - area.top,
                            },
                            moved: snap_side.is_some()
                                || rect.left != origin.left
                                || rect.top != origin.top,
                            cancelled: GetAsyncKeyState(VK_ESCAPE as i32) < 0,
                            snap_side,
                        },
                    );
                }
            }
        }
        DefSubclassProc(hwnd, message, wparam, lparam)
    }

    // Called once from Tauri setup on the owning UI thread, before showing the ball.
    pub fn install(window: tauri::WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
        let hwnd = window.hwnd()?.0 as HWND;
        let state = Box::into_raw(Box::new(DragState {
            window,
            origin: None,
            snapped: None,
        }));
        if unsafe { SetWindowSubclass(hwnd, Some(callback), SUBCLASS_ID, state as usize) } == 0 {
            unsafe {
                drop(Box::from_raw(state));
            }
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub use windows::install;

// Shared commands delegate here; all Win32 calls remain in the Windows module.
pub(crate) fn primary_down() -> bool {
    use windows_sys::Win32::UI::{
        Input::KeyboardAndMouse::GetAsyncKeyState, WindowsAndMessaging::GetSystemMetrics,
    };
    unsafe { GetAsyncKeyState(if GetSystemMetrics(23) != 0 { 2 } else { 1 }) < 0 }
}
pub(crate) async fn start(window: tauri::WebviewWindow) -> Result<(), String> {
    window
        .start_dragging()
        .map_err(|_| "暂时无法拖动窗口".into())
}

pub(crate) fn pointer_for_window(
    window: &tauri::WebviewWindow,
) -> Result<tauri::PhysicalPosition<f64>, String> {
    window
        .cursor_position()
        .map_err(|_| "无法读取鼠标位置".into())
}

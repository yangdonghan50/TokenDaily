//! Normalized desktop floating-window capabilities.
#[cfg(target_os = "macos")]
pub(crate) use super::macos::floating::{install, pointer_for_window, primary_down, start};
#[cfg(target_os = "windows")]
pub(crate) use super::windows::floating_drag::{install, pointer_for_window, primary_down, start};

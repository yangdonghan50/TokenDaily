//! Normalized platform boundary for system notifications.
#[cfg(target_os = "macos")]
pub(crate) use super::macos::notification::{
    check_test, install, permission, request_permission, send,
};
#[cfg(target_os = "windows")]
pub(crate) use super::windows::notification::{
    check_test, install, permission, request_permission, send,
};

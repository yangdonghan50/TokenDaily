//! OS integration boundary. Data contracts and product rules stay outside this module.

pub(crate) mod floating;
pub(crate) mod floating_placement;
pub(crate) mod leaderboard_credentials;
pub(crate) mod log_watch;
pub(crate) mod notification;
pub(crate) mod workbuddy_credentials;
#[cfg(target_os = "macos")]
pub(crate) use macos::app_update;
#[cfg(target_os = "windows")]
pub(crate) use windows::app_update;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub(crate) use macos::{
    claude_account_file, claude_credentials, claude_desktop_history, claude_home, codex_executable,
    codex_home, configure_codex_command, hotkey, open_official_url, trae_database, workbuddy_home,
};
#[cfg(target_os = "windows")]
pub(crate) use windows::{
    claude_account_file, claude_credentials, claude_desktop_history, claude_home, codex_home,
    hotkey, open_official_url, protect_secret, trae_database, unprotect_secret, workbuddy_home,
};

#[cfg(target_os = "windows")]
pub(crate) fn codex_executable(
    _selected: Option<&std::path::Path>,
) -> Result<std::path::PathBuf, &'static str> {
    windows::codex_executable().ok_or("未找到本机 Codex CLI")
}

#[cfg(target_os = "windows")]
pub(crate) fn configure_codex_command(
    _command: &mut std::process::Command,
    _executable: &std::path::Path,
) {
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
compile_error!("TokenDaily currently supports Windows and macOS desktop targets only");

use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Phase {
    Unsupported,
    Idle,
    Checking,
    Current,
    Available,
    Downloading,
    Installing,
    CheckError,
    InstallError,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Snapshot {
    pub phase: Phase,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub downloaded: u64,
    pub total: Option<u64>,
    pub message: Option<String>,
}

impl Snapshot {
    pub(crate) fn new(version: &str, supported: bool) -> Self {
        Self {
            phase: if supported {
                Phase::Idle
            } else {
                Phase::Unsupported
            },
            current_version: version.into(),
            latest_version: None,
            downloaded: 0,
            total: None,
            message: None,
        }
    }
}

pub(crate) struct UpdateState {
    pub snapshot: Snapshot,
    last_check: Option<Instant>,
}

impl UpdateState {
    pub fn new(version: &str) -> Self {
        Self {
            snapshot: Snapshot::new(version, true),
            last_check: None,
        }
    }
    pub fn begin_check(&mut self, now: Instant, automatic: bool) -> bool {
        if matches!(
            self.snapshot.phase,
            Phase::Checking | Phase::Downloading | Phase::Installing
        ) || self.last_check.is_some_and(|at| {
            now.saturating_duration_since(at)
                < Duration::from_secs(if automatic { 6 * 3600 } else { 60 })
        }) {
            return false;
        }
        self.last_check = Some(now);
        self.snapshot.phase = Phase::Checking;
        self.snapshot.message = None;
        true
    }
    pub fn finish_check(&mut self, version: Option<&str>, error: Option<&str>) {
        self.snapshot.message = error.map(Into::into);
        if error.is_some() {
            self.snapshot.phase = if self.snapshot.latest_version.is_some() {
                Phase::Available
            } else {
                Phase::CheckError
            };
        } else {
            self.snapshot.latest_version = version.map(Into::into);
            self.snapshot.phase = if version.is_some() {
                Phase::Available
            } else {
                Phase::Current
            };
        }
    }
    pub fn begin_install(&mut self) -> bool {
        if !matches!(self.snapshot.phase, Phase::Available | Phase::InstallError)
            || self.snapshot.latest_version.is_none()
        {
            return false;
        }
        self.snapshot.phase = Phase::Downloading;
        self.snapshot.downloaded = 0;
        self.snapshot.total = None;
        self.snapshot.message = None;
        true
    }
    pub fn progress(&mut self, chunk: usize, total: Option<u64>) {
        self.snapshot.downloaded = self.snapshot.downloaded.saturating_add(chunk as u64);
        self.snapshot.total = total.filter(|value| *value > 0);
    }
    pub fn ready_to_install(&mut self) {
        self.snapshot.phase = Phase::Installing;
    }
    pub fn fail_install(&mut self, message: &str) {
        self.snapshot.phase = Phase::InstallError;
        self.snapshot.message = Some(message.into());
    }
}

fn local_window(window: &tauri::WebviewWindow) -> Result<(), String> {
    if matches!(window.label(), "main" | "floating") {
        Ok(())
    } else {
        Err("仅允许 TokenDaily 主界面执行更新".into())
    }
}

#[tauri::command]
pub(crate) fn get_app_update(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
) -> Result<Snapshot, String> {
    local_window(&window)?;
    Ok(crate::platform::app_update::snapshot(&app))
}
#[tauri::command]
pub(crate) async fn check_app_update(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
) -> Result<Snapshot, String> {
    local_window(&window)?;
    Ok(crate::platform::app_update::check(app, false).await)
}
#[tauri::command]
pub(crate) async fn install_app_update(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
) -> Result<Snapshot, String> {
    local_window(&window)?;
    crate::platform::app_update::install(app).await
}

#[cfg(test)]
#[path = "app_update_tests.rs"]
mod tests;

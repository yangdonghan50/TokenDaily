//! Independent WorkBuddy local logs. Official credits live in workbuddy_credits.
use crate::{codex::DesktopState, usage::UsageState, workbuddy_usage::Collector};
use std::{fs, path::PathBuf, sync::Mutex, thread, time::Duration};
use tauri::{Emitter, Manager, State};
struct Inner {
    enabled: bool,
    generation: u64,
    refreshing: bool,
    current: DesktopState,
    usage: UsageState,
}
pub(crate) struct Service {
    inner: Mutex<Inner>,
    collector: Mutex<(u64, Collector)>,
    path: PathBuf,
}
impl Service {
    pub(crate) fn new(dir: &std::path::Path) -> Self {
        let path = dir.join("workbuddy.json");
        let enabled = fs::read(&path)
            .ok()
            .and_then(|b| serde_json::from_slice::<bool>(&b).ok())
            .unwrap_or(false);
        let mut current = DesktopState::disconnected();
        current.connected = enabled;
        if enabled {
            current.status = "loading".into();
        }
        Self {
            inner: Mutex::new(Inner {
                enabled,
                generation: 0,
                refreshing: false,
                current,
                usage: UsageState::default(),
            }),
            collector: Mutex::new((0, Collector::new(crate::platform::workbuddy_home()))),
            path,
        }
    }
}
fn emit(app: &tauri::AppHandle, inner: &Inner) {
    let _ = app.emit("workbuddy-state-updated", &inner.current);
    let _ = app.emit("workbuddy-local-usage-updated", &inner.usage);
    let _ = app.emit("workbuddy-refresh-status", inner.refreshing);
}
#[tauri::command]
pub(crate) fn get_workbuddy_state(s: State<'_, Service>) -> DesktopState {
    s.inner.lock().unwrap().current.clone()
}
#[tauri::command]
pub(crate) fn get_workbuddy_local_usage(s: State<'_, Service>) -> UsageState {
    s.inner.lock().unwrap().usage.clone()
}
#[tauri::command]
pub(crate) fn get_workbuddy_refreshing(s: State<'_, Service>) -> bool {
    s.inner.lock().unwrap().refreshing
}
#[tauri::command]
pub(crate) async fn connect_workbuddy(app: tauri::AppHandle) -> Result<DesktopState, String> {
    {
        let s = app.state::<Service>();
        let mut inner = s.inner.lock().unwrap();
        fs::write(&s.path, b"true").map_err(|_| "WorkBuddy 连接设置保存失败")?;
        if !inner.enabled {
            inner.generation += 1;
        }
        inner.enabled = true;
        inner.current.connected = true;
    }
    Ok(refresh_workbuddy(app).await)
}
#[tauri::command]
pub(crate) fn disconnect_workbuddy(app: tauri::AppHandle) -> Result<DesktopState, String> {
    crate::workbuddy_credits::clear(&app)?;
    let s = app.state::<Service>();
    let mut inner = s.inner.lock().unwrap();
    fs::write(&s.path, b"false").map_err(|_| "WorkBuddy 连接设置保存失败")?;
    reset(&mut inner);
    emit(&app, &inner);
    let state = inner.current.clone();
    drop(inner);
    crate::providers::update_selected_tray(&app);
    Ok(state)
}
fn reset(inner: &mut Inner) {
    inner.enabled = false;
    inner.generation += 1;
    inner.refreshing = false;
    inner.current = DesktopState::disconnected();
    inner.usage = UsageState::default();
}
pub(crate) fn clear(app: &tauri::AppHandle) -> Result<(), String> {
    crate::workbuddy_credits::clear(app)?;
    let s = app.state::<Service>();
    let mut inner = s.inner.lock().unwrap();
    reset(&mut inner);
    let _ = fs::remove_file(&s.path);
    emit(app, &inner);
    Ok(())
}
#[tauri::command]
pub(crate) async fn refresh_workbuddy(app: tauri::AppHandle) -> DesktopState {
    let credits_app = app.clone();
    tauri::async_runtime::spawn(async move {
        crate::workbuddy_credits::refresh_workbuddy_credits(credits_app).await;
    });
    let fallback = app.clone();
    tauri::async_runtime::spawn_blocking(move || refresh(&app, true))
        .await
        .unwrap_or_else(|_| {
            let s = fallback.state::<Service>();
            let mut inner = s.inner.lock().unwrap();
            inner.refreshing = false;
            if inner.enabled {
                inner.usage = UsageState::with_error(&inner.usage, "WorkBuddy 采集任务意外结束");
            }
            emit(&fallback, &inner);
            inner.current.clone()
        })
}
fn refresh(app: &tauri::AppHandle, force: bool) -> DesktopState {
    let s = app.state::<Service>();
    let generation = {
        let mut inner = s.inner.lock().unwrap();
        if !inner.enabled || inner.refreshing {
            return inner.current.clone();
        }
        inner.refreshing = true;
        if force {
            let _ = app.emit("workbuddy-refresh-status", true);
        }
        inner.generation
    };
    let usage = {
        let mut pair = s.collector.lock().unwrap();
        if pair.0 != generation {
            *pair = (
                generation,
                Collector::new(crate::platform::workbuddy_home()),
            );
        }
        pair.1.poll(crate::now_ms(), force)
    };
    let mut inner = s.inner.lock().unwrap();
    if !accept_result(inner.enabled, inner.generation, generation) {
        return inner.current.clone();
    }
    inner.usage = usage;
    inner.current = DesktopState {
        connected: true,
        status: "unsupported".into(),
        identity_label: Some("本机记录 · 不区分账户".into()),
        message: Some("本机 Token 来自日志；账户积分独立授权与刷新，可在下方设置中管理。".into()),
        ..DesktopState::disconnected()
    };
    inner.refreshing = false;
    emit(app, &inner);
    let current = inner.current.clone();
    drop(inner);
    crate::providers::update_selected_tray(app);
    current
}
fn accept_result(enabled: bool, current: u64, started: u64) -> bool {
    enabled && current == started
}
pub(crate) fn start(app: tauri::AppHandle) {
    thread::spawn(move || loop {
        refresh(&app, false);
        thread::sleep(Duration::from_secs(2));
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disconnect_and_reconnect_discard_inflight_results() {
        assert!(!accept_result(false, 1, 1));
        assert!(!accept_result(true, 2, 1));
        assert!(accept_result(true, 2, 2));
    }
}

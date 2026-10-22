//! Provider selection and independent Claude runtime; Codex retains its existing pipeline.
use crate::{
    alerts, claude, claude_statusline, claude_usage::Collector, codex::DesktopState,
    usage::UsageState,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Mutex, thread, time::Duration};
use tauri::{Emitter, Manager, State};

#[derive(Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Provider {
    #[default]
    All,
    Codex,
    Claude,
    Workbuddy,
    Trae,
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    provider: Provider,
    claude_enabled: bool,
}
struct Inner {
    settings: Settings,
    generation: u64,
    current: DesktopState,
    usage: UsageState,
    refreshing: bool,
    next_allowed: i64,
    next_auto: i64,
    failures: u32,
    credential: Option<String>,
}
pub(crate) struct Providers {
    ledger: Mutex<alerts::Delivery>,
    inner: Mutex<Inner>,
    settings_path: PathBuf,
    alerts_path: PathBuf,
    data_dir: PathBuf,
}
impl Providers {
    pub(crate) fn new(directory: &std::path::Path) -> Self {
        let settings_path = directory.join("providers.json");
        let settings: Settings = fs::read(&settings_path)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default();
        let alerts_path = directory.join(if cfg!(debug_assertions) {
            "claude-alerts-dev.json"
        } else {
            "claude-alerts.json"
        });
        let ledger = alerts::load_delivery(&alerts_path);
        let mut current = DesktopState::disconnected();
        current.connected = settings.claude_enabled;
        if current.connected {
            current.status = "loading".into();
        }
        Self {
            inner: Mutex::new(Inner {
                settings,
                generation: 0,
                current,
                usage: UsageState::default(),
                refreshing: false,
                next_allowed: 0,
                next_auto: 0,
                failures: 0,
                credential: None,
            }),
            settings_path,
            alerts_path,
            data_dir: directory.to_path_buf(),
            ledger: Mutex::new(ledger),
        }
    }
    fn save(&self, settings: &Settings) -> Result<(), String> {
        fs::write(
            &self.settings_path,
            serde_json::to_vec(settings).map_err(|_| "无法编码连接设置")?,
        )
        .map_err(|_| "连接设置保存失败，请检查磁盘权限与空间".into())
    }
}
#[tauri::command]
pub(crate) fn get_provider(state: State<'_, Providers>) -> Provider {
    state.inner.lock().unwrap().settings.provider
}
#[tauri::command]
pub(crate) fn set_provider(app: tauri::AppHandle, provider: Provider) -> Result<Provider, String> {
    let service = app.state::<Providers>();
    let mut inner = service.inner.lock().unwrap();
    let mut settings = inner.settings.clone();
    settings.provider = provider;
    service.save(&settings)?;
    inner.settings = settings;
    drop(inner);
    let _ = app.emit("provider-updated", provider);
    update_selected_tray(&app);
    Ok(provider)
}
pub(crate) fn is_claude(app: &tauri::AppHandle) -> bool {
    app.try_state::<Providers>()
        .is_some_and(|s| s.inner.lock().unwrap().settings.provider == Provider::Claude)
}
pub(crate) fn is_all(app: &tauri::AppHandle) -> bool {
    app.try_state::<Providers>()
        .is_some_and(|s| s.inner.lock().unwrap().settings.provider == Provider::All)
}
pub(crate) fn is_trae(app: &tauri::AppHandle) -> bool {
    app.try_state::<Providers>()
        .is_some_and(|s| s.inner.lock().unwrap().settings.provider == Provider::Trae)
}
pub(crate) fn is_workbuddy(app: &tauri::AppHandle) -> bool {
    app.try_state::<Providers>()
        .is_some_and(|s| s.inner.lock().unwrap().settings.provider == Provider::Workbuddy)
}
pub(crate) fn overview_tooltip(app: &tauri::AppHandle, codex: &DesktopState) -> String {
    let claude = app
        .state::<Providers>()
        .inner
        .lock()
        .unwrap()
        .current
        .clone();
    let brief = |label: &str, state: &DesktopState| {
        let prefix = if !state.connected {
            "未连接"
        } else if state.status == "auth-required" {
            if label == "Claude" {
                "额度未接入"
            } else {
                "需要登录"
            }
        } else if state.snapshot.is_none() {
            "额度未知"
        } else {
            ""
        };
        let windows = state
            .snapshot
            .as_ref()
            .map(|s| {
                s.windows
                    .iter()
                    .map(|w| {
                        format!(
                            "{} {}",
                            w.label,
                            w.remaining_percent
                                .map(|p| format!("{p:.0}%"))
                                .unwrap_or_else(|| "未知".into())
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" / ")
            })
            .unwrap_or_default();
        let old = state.snapshot.as_ref().is_some_and(|s| {
            state.status == "stale" || crate::now_ms() - s.collected_at_ms > 600_000
        });
        format!(
            "{label} · {prefix}{}{windows}",
            if old { "上次剩余 " } else { "" }
        )
        .chars()
        .take(58)
        .collect::<String>()
    };
    format!(
        "TokenDaily\n{}\n{}\nTRAE · {}\nWorkBuddy · {}",
        brief("Codex", codex),
        brief("Claude", &claude),
        if crate::trae::get_trae_state(app.state::<crate::trae::Service>()).connected {
            crate::trae_credits::tooltip(app)
        } else {
            "未连接".into()
        },
        if crate::workbuddy::get_workbuddy_state(app.state::<crate::workbuddy::Service>()).connected
        {
            crate::workbuddy_credits::tooltip(app)
        } else {
            "未连接".into()
        }
    )
}
pub(crate) fn update_selected_tray(app: &tauri::AppHandle) {
    if is_trae(app) {
        if let Some(tray) = app.tray_by_id("main") {
            let _ = tray.set_tooltip(Some(format!(
                "TokenDaily · TRAE · {}",
                crate::trae_credits::tooltip(app)
            )));
        }
    } else if is_workbuddy(app) {
        let state = crate::workbuddy::get_workbuddy_state(app.state::<crate::workbuddy::Service>());
        if let Some(tray) = app.tray_by_id("main") {
            let _ = tray.set_tooltip(Some(if state.connected {
                format!(
                    "TokenDaily · WorkBuddy · {}",
                    crate::workbuddy_credits::tooltip(app)
                )
            } else {
                "TokenDaily · WorkBuddy 未连接".into()
            }));
        }
    } else if is_claude(app) {
        let state = app
            .state::<Providers>()
            .inner
            .lock()
            .unwrap()
            .current
            .clone();
        if let Some(tray) = app.tray_by_id("main") {
            let tooltip = if state.status == "auth-required" {
                "TokenDaily · Claude 额度来源未接入".into()
            } else {
                crate::tray_tooltip(&state).replace("Codex", "Claude")
            };
            let _ = tray.set_tooltip(Some(tooltip));
        }
    } else {
        let state = app
            .state::<crate::AppState>()
            .current
            .lock()
            .unwrap()
            .clone();
        crate::update_tray(app, &state);
    }
}
#[tauri::command]
pub(crate) fn get_claude_state(state: State<'_, Providers>) -> DesktopState {
    state.inner.lock().unwrap().current.clone()
}
#[tauri::command]
pub(crate) fn get_claude_local_usage(state: State<'_, Providers>) -> UsageState {
    state.inner.lock().unwrap().usage.clone()
}
#[tauri::command]
pub(crate) fn get_claude_refreshing(state: State<'_, Providers>) -> bool {
    state.inner.lock().unwrap().refreshing
}

#[tauri::command]
pub(crate) fn get_claude_statusline(
    state: State<'_, Providers>,
) -> claude_statusline::BridgeStatus {
    claude_statusline::status(&state.data_dir, None)
}
#[tauri::command]
pub(crate) async fn set_claude_statusline(
    app: tauri::AppHandle,
    enabled: bool,
) -> Result<claude_statusline::BridgeStatus, String> {
    let dir = app.state::<Providers>().data_dir.clone();
    let notice = if enabled {
        let executable = std::env::current_exe().map_err(|_| "无法定位 TokenDaily 程序")?;
        claude_statusline::enable(&dir, &executable)?;
        None
    } else {
        claude_statusline::disable(&dir, false)?
    };
    app.state::<Providers>().inner.lock().unwrap().next_allowed = 0;
    let refresh_app = app.clone();
    tauri::async_runtime::spawn(async move {
        refresh_claude(refresh_app).await;
    });
    Ok(claude_statusline::status(&dir, notice))
}

/// Adds reset times from an opted-in Claude Code statusLine capture to Claude
/// Desktop's local record. A verified Claude Code quota read already carries reset
/// times and is never mixed with other sources.
#[cfg(target_os = "macos")]
fn with_statusline(
    dir: &std::path::Path,
    result: Result<(DesktopState, String), claude::Failure>,
) -> Result<(DesktopState, String), claude::Failure> {
    let Ok((state, credential)) = &result else {
        return result;
    };
    if !credential.starts_with("desktop:") {
        return result;
    }
    let Some(capture) = claude_statusline::load_capture(dir) else {
        return result;
    };
    match claude_statusline::merge(Some(state), &capture, crate::now_ms()) {
        Some(merged) => Ok((merged, credential.clone())),
        None => result,
    }
}

#[tauri::command]
pub(crate) async fn connect_claude(app: tauri::AppHandle) -> Result<DesktopState, String> {
    {
        let service = app.state::<Providers>();
        let mut inner = service.inner.lock().unwrap();
        let mut settings = inner.settings.clone();
        settings.claude_enabled = true;
        service.save(&settings)?;
        if !inner.settings.claude_enabled {
            inner.generation += 1;
            inner.next_allowed = 0;
            inner.next_auto = 0;
        }
        inner.settings = settings;
        inner.current.connected = true;
    }
    Ok(refresh_claude(app).await)
}
#[tauri::command]
pub(crate) fn disconnect_claude(app: tauri::AppHandle) -> Result<DesktopState, String> {
    let service = app.state::<Providers>();
    let mut inner = service.inner.lock().unwrap();
    let mut settings = inner.settings.clone();
    settings.claude_enabled = false;
    service.save(&settings)?;
    inner.settings = settings;
    reset(&mut inner);
    emit(&app, &inner);
    let current = inner.current.clone();
    drop(inner);
    update_selected_tray(&app);
    Ok(current)
}
fn reset(inner: &mut Inner) {
    inner.generation += 1;
    inner.current = DesktopState::disconnected();
    inner.usage = UsageState::default();
    inner.credential = None;
    inner.refreshing = false;
    inner.next_allowed = 0;
    inner.next_auto = 0;
    inner.failures = 0;
}
pub(crate) fn clear(app: &tauri::AppHandle) {
    let service = app.state::<Providers>();
    // Restore Claude Code's statusLine before deleting the bridge record it depends on.
    let _ = claude_statusline::disable(&service.data_dir, true);
    let mut inner = service.inner.lock().unwrap();
    inner.settings = Settings::default();
    reset(&mut inner);
    let _ = fs::remove_file(&service.settings_path);
    emit(app, &inner);
    let _ = app.emit("provider-updated", Provider::All);
    drop(inner);
    // Same lock as submission: an invalidated worker cannot recreate deleted slots.
    let mut ledger = service.ledger.lock().unwrap();
    *ledger = alerts::Delivery::default();
    for path in alerts::ledger_paths(&service.alerts_path)
        .into_iter()
        .chain([service.alerts_path.clone()])
    {
        if let Err(error) = fs::remove_file(path) {
            if error.kind() != std::io::ErrorKind::NotFound {
                *ledger = alerts::Delivery::blocked();
            }
        }
    }
    let _ = app.emit("claude-notification-status", &ledger.status);
}
fn emit(app: &tauri::AppHandle, inner: &Inner) {
    let _ = app.emit("claude-state-updated", &inner.current);
    let _ = app.emit("claude-local-usage-updated", &inner.usage);
    let _ = app.emit("claude-refresh-status", inner.refreshing);
}
#[tauri::command]
pub(crate) async fn refresh_claude(app: tauri::AppHandle) -> DesktopState {
    let fallback = app.clone();
    tauri::async_runtime::spawn_blocking(move || refresh(&app))
        .await
        .unwrap_or_else(|_| {
            let service = fallback.state::<Providers>();
            let mut inner = service.inner.lock().unwrap();
            inner.refreshing = false;
            if inner.settings.claude_enabled {
                inner.current = DesktopState::with_error(&inner.current, "Claude 采集任务意外结束");
            }
            emit(&fallback, &inner);
            inner.current.clone()
        })
}
fn refresh(app: &tauri::AppHandle) -> DesktopState {
    let service = app.state::<Providers>();
    let generation = {
        let mut inner = service.inner.lock().unwrap();
        if !inner.settings.claude_enabled
            || inner.refreshing
            || crate::now_ms() < inner.next_allowed
        {
            return inner.current.clone();
        }
        inner.refreshing = true;
        let _ = app.emit("claude-refresh-status", true);
        inner.generation
    };
    let observed = claude::source_fingerprint().ok();
    let result = claude::read_once();
    #[cfg(target_os = "macos")]
    let result = with_statusline(&service.data_dir, result);
    let mut inner = service.inner.lock().unwrap();
    if inner.generation != generation || !inner.settings.claude_enabled {
        return inner.current.clone();
    }
    let now = crate::now_ms();
    match result {
        Ok((current, credential)) => {
            if claude::source_fingerprint().ok().as_deref() == Some(credential.as_str()) {
                inner.current = current;
                inner.credential = Some(credential);
                inner.failures = 0;
                inner.next_allowed = now + 10_000;
            } else {
                inner.current = DesktopState::with_error(
                    &DesktopState::disconnected(),
                    "Claude 登录来源已变化，请重新刷新",
                );
                inner.credential = None;
                inner.failures = inner.failures.saturating_add(1);
                inner.next_allowed = now + 60_000;
            }
        }
        Err(failure) => {
            let same = observed.is_some()
                && inner.credential == observed
                && claude::source_fingerprint().ok() == observed;
            inner.current = failed_state(&inner.current, same, failure.auth, failure.message);
            if !same || failure.auth {
                inner.credential = None;
            }
            inner.failures = inner.failures.saturating_add(1);
            inner.next_allowed = now + 60_000;
        }
    }
    inner.next_auto = now + (300_000i64 * (1i64 << inner.failures.min(3))).min(3_600_000);
    inner.refreshing = false;
    emit(app, &inner);
    let current = inner.current.clone();
    drop(inner);
    update_selected_tray(app);
    process_claude_alerts(app, generation, &current);
    current
}
fn current_claude_snapshot(
    enabled: bool,
    generation: u64,
    expected: u64,
    current: &DesktopState,
    snapshot: &crate::codex::QuotaSnapshot,
) -> bool {
    enabled && generation == expected && crate::notification::matches_current(current, snapshot)
}

// Runs in the quota worker after releasing provider/UI state. Native waits never
// hold the provider mutex; disconnect/clear can invalidate this snapshot promptly.
fn process_claude_alerts(app: &tauri::AppHandle, generation: u64, current: &DesktopState) {
    let Some(snapshot) = current.snapshot.as_ref() else {
        return;
    };
    let service = app.state::<Providers>();
    let mut ledger = service.ledger.lock().unwrap();
    let still_valid = || {
        let inner = service.inner.lock().unwrap();
        current_claude_snapshot(
            inner.settings.claude_enabled,
            inner.generation,
            generation,
            &inner.current,
            snapshot,
        )
    };
    if !still_valid() {
        return;
    }
    let settings = app
        .state::<crate::AppState>()
        .alert_settings
        .lock()
        .unwrap()
        .clone();
    ledger.process(
        snapshot,
        &settings,
        crate::now_ms(),
        !cfg!(debug_assertions),
        |record| alerts::save_record(&service.alerts_path, record),
        |event| {
            crate::platform::notification::send(
                app,
                &event.title.replace("Codex", "Claude"),
                &event.body,
                false,
                still_valid,
            )
            .map(|_| ())
            .map_err(|error| error.kind)
        },
    );
    let _ = app.emit("claude-notification-status", &ledger.status);
}

#[tauri::command]
pub(crate) fn get_claude_notification_status(state: State<'_, Providers>) -> String {
    state.ledger.lock().unwrap().status.clone()
}

fn failed_state(
    previous: &DesktopState,
    same_credential: bool,
    auth: bool,
    message: &'static str,
) -> DesktopState {
    let empty = DesktopState::disconnected();
    let mut next = DesktopState::with_error(
        if same_credential && !auth {
            previous
        } else {
            &empty
        },
        message,
    );
    if auth {
        next.status = "auth-required".into();
    }
    next
}
pub(crate) fn start(app: tauri::AppHandle) {
    let quota_app = app.clone();
    thread::spawn(move || loop {
        let due = {
            let service = quota_app.state::<Providers>();
            let inner = service.inner.lock().unwrap();
            inner.settings.claude_enabled && !inner.refreshing && crate::now_ms() >= inner.next_auto
        };
        if due {
            refresh(&quota_app);
        }
        thread::sleep(Duration::from_secs(2));
    });
    thread::spawn(move || {
        let mut collector = Collector::new(crate::platform::claude_home());
        let mut epoch = u64::MAX;
        loop {
            let (enabled, generation) = {
                let service = app.state::<Providers>();
                let inner = service.inner.lock().unwrap();
                (inner.settings.claude_enabled, inner.generation)
            };
            if generation != epoch {
                collector = Collector::new(crate::platform::claude_home());
                epoch = generation;
            }
            if enabled {
                let usage = collector.poll(crate::now_ms());
                let service = app.state::<Providers>();
                let mut inner = service.inner.lock().unwrap();
                if inner.generation == generation && inner.settings.claude_enabled {
                    inner.usage = usage;
                    let _ = app.emit("claude-local-usage-updated", &inner.usage);
                }
            }
            thread::sleep(Duration::from_secs(2));
        }
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn claude_errors_never_reuse_another_account_or_auth_rejected_snapshot() {
        let mut old = DesktopState::disconnected();
        old.snapshot = Some(
            claude::normalize_quota(&serde_json::json!({"seven_day":{"utilization":80}}), "a", 1)
                .unwrap(),
        );
        assert!(failed_state(&old, false, false, "network")
            .snapshot
            .is_none());
        assert!(failed_state(&old, true, true, "auth").snapshot.is_none());
        assert_eq!(failed_state(&old, true, false, "network").status, "stale");
    }
    #[test]
    fn claude_notifications_require_verified_current_identity_and_generation() {
        let snapshot = claude::normalize_quota(
            &serde_json::json!({"seven_day":{"utilization":90}}),
            "account-a",
            1000,
        )
        .unwrap();
        let mut current = DesktopState::disconnected();
        current.connected = true;
        current.status = "available".into();
        current.identity_verified = true;
        current.snapshot = Some(snapshot.clone());
        assert!(current_claude_snapshot(true, 2, 2, &current, &snapshot));
        assert!(!current_claude_snapshot(false, 2, 2, &current, &snapshot));
        assert!(!current_claude_snapshot(true, 3, 2, &current, &snapshot));
        current.identity_verified = false;
        assert!(!current_claude_snapshot(true, 2, 2, &current, &snapshot));
        current.identity_verified = true;
        current.snapshot.as_mut().unwrap().identity_key = "claude:other".into();
        assert!(!current_claude_snapshot(true, 2, 2, &current, &snapshot));
        current.snapshot = Some(snapshot.clone());
        current.status = "stale".into();
        assert!(!current_claude_snapshot(true, 2, 2, &current, &snapshot));
    }
    #[test]
    fn old_settings_default_to_overview_and_opt_out_of_claude() {
        let settings: Settings = serde_json::from_str("{}").unwrap();
        assert!(settings.provider == Provider::All);
        assert!(!settings.claude_enabled);
        assert!(serde_json::from_str::<Provider>("\"unknown\"").is_err());
    }
}

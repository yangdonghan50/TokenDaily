mod alerts;
mod api_cost;
mod app_update;
mod claude;
mod claude_statusline;
mod claude_usage;
mod codex;
mod connection;
mod credit_cache;
mod credit_display;
mod floating_hotkey;
mod leaderboard;
mod local_usage;
mod notification;
mod platform;
mod providers;
mod total_history;
mod trae;
mod trae_credits;
mod trae_usage;
mod usage;
mod workbuddy;
mod workbuddy_credits;
mod workbuddy_usage;

pub use claude_statusline::{run_helper as claude_statusline, FLAG as CLAUDE_STATUSLINE_FLAG};

use alerts::AlertSettings;
use codex::DesktopState;
use platform::log_watch;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Mutex,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State, WindowEvent,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use usage::UsageState;

struct AppState {
    connection: Mutex<connection::Settings>,
    connection_transition: Mutex<()>,
    local_usage: Mutex<UsageState>,
    local_epoch: AtomicU32,
    local_worker: Mutex<Option<log_watch::SharedWake>>,
    current: Mutex<DesktopState>,
    refreshing: AtomicBool,
    failures: AtomicU32,
    connected: AtomicBool,
    floating_enabled: AtomicBool,
    floating_open_requested: AtomicBool,
    hotkey_settings_requested: AtomicBool,
    floating_edge: AtomicBool,
    floating_mode: Mutex<FloatingMode>,
    floating_transition: Mutex<()>,
    settings_write: Mutex<()>,
    cache_path: PathBuf,
    settings_path: PathBuf,
    alert_settings: Mutex<AlertSettings>,
    alert_ledger: Mutex<alerts::Delivery>,
    alert_worker: Mutex<notification::WorkerGate>,
    alerts_path: PathBuf,
    local_index_path: PathBuf,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum FloatingMode {
    #[default]
    Usage,
    Quota,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct LocalSettings {
    connection: connection::Settings,
    connected: bool,
    alerts: AlertSettings,
    floating_enabled: bool,
    floating_edge: bool,
    floating_mode: FloatingMode,
}

fn load_settings(path: &PathBuf) -> LocalSettings {
    fs::read(path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<LocalSettings>(&bytes).ok())
        .unwrap_or_default()
}

fn save_settings(path: &PathBuf, settings: &LocalSettings) -> Result<(), String> {
    let bytes = serde_json::to_vec(settings).map_err(|_| "无法编码本地设置".to_string())?;
    fs::write(path, bytes).map_err(|_| "无法保存本地设置，请检查磁盘空间和目录权限".into())
}

fn persist_settings(state: &AppState) -> Result<(), String> {
    let _write = state.settings_write.lock().expect("settings write lock");
    save_settings(&state.settings_path, &settings_snapshot(state))
}

fn settings_snapshot(state: &AppState) -> LocalSettings {
    let alerts = state
        .alert_settings
        .lock()
        .expect("alert settings lock")
        .clone();
    LocalSettings {
        connection: state.connection.lock().expect("connection lock").clone(),
        connected: state.connected.load(Ordering::SeqCst),
        alerts,
        floating_enabled: state.floating_enabled.load(Ordering::SeqCst),
        floating_edge: state.floating_edge.load(Ordering::SeqCst),
        floating_mode: *state.floating_mode.lock().expect("floating mode lock"),
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CachedState {
    source_key: String,
    state: DesktopState,
}

fn load_cache(path: &PathBuf, source_key: Option<&str>) -> DesktopState {
    let Ok(bytes) = fs::read(path) else {
        return DesktopState::disconnected();
    };
    let Ok(envelope) = serde_json::from_slice::<CachedState>(&bytes) else {
        return DesktopState::disconnected();
    };
    if Some(envelope.source_key.as_str()) != source_key {
        return DesktopState::disconnected();
    }
    let mut cached = envelope.state;
    if !cached.identity_verified {
        return DesktopState::disconnected();
    }
    if cached.snapshot.is_none() && cached.usage.snapshot.is_none() {
        return DesktopState::disconnected();
    }
    if cached.snapshot.is_some() {
        cached.status = "stale".into();
        cached.message = Some("上次连接账户缓存，当前身份待核对".into());
    }
    if cached.usage.snapshot.is_some() {
        cached.usage.status = "stale".into();
        cached.usage.message = Some("上次日用量记录，当前身份待核对".into());
    }
    cached
}

fn save_cache(path: &PathBuf, state: &DesktopState, source_key: &str) {
    if state.snapshot.is_none() && state.usage.snapshot.is_none() {
        let _ = fs::remove_file(path);
    } else if let Ok(bytes) = serde_json::to_vec(&CachedState {
        source_key: source_key.into(),
        state: state.clone(),
    }) {
        let _ = fs::write(path, bytes); // Only normalized quota, source-day counts and masked identity are saved.
    }
}

fn tray_tooltip(state: &DesktopState) -> String {
    let summary = match state.status.as_str() {
        "disconnected" => "尚未连接 Codex".to_owned(),
        "loading" => "正在获取额度".to_owned(),
        "stale" => "上次成功数据，等待刷新".to_owned(),
        "auth-required" => "需要在 Codex 中登录".to_owned(),
        "unsupported" => "当前账户无法读取额度".to_owned(),
        "error" => "额度更新失败".to_owned(),
        _ => state
            .snapshot
            .as_ref()
            .and_then(|snapshot| {
                snapshot
                    .windows
                    .iter()
                    .filter_map(|window| {
                        window
                            .remaining_percent
                            .map(|remaining| (window, remaining))
                    })
                    .min_by(|left, right| left.1.total_cmp(&right.1))
            })
            .map(|(window, remaining)| {
                let percent = if remaining > 0.0 && remaining < 1.0 {
                    "<1%".to_owned()
                } else if remaining > 99.0 && remaining < 100.0 {
                    ">99%".to_owned()
                } else {
                    format!("{remaining:.0}%")
                };
                format!("{} 剩余 {percent} · 数据已更新", window.label)
            })
            .unwrap_or_else(|| "额度未知".to_owned()),
    };
    format!("TokenDaily · {summary}")
        .chars()
        .take(120)
        .collect()
}

fn update_tray(app: &tauri::AppHandle, state: &DesktopState) {
    if providers::is_claude(app) || providers::is_workbuddy(app) || providers::is_trae(app) {
        return;
    }
    if let Some(tray) = app.tray_by_id("main") {
        let tooltip = if providers::is_all(app) {
            providers::overview_tooltip(app, state)
        } else {
            tray_tooltip(state)
        };
        let _ = tray.set_tooltip(Some(tooltip));
    }
}

#[tauri::command]
fn get_state(state: State<'_, AppState>) -> DesktopState {
    state.current.lock().expect("state lock").clone()
}

#[tauri::command]
fn get_refreshing(state: State<'_, AppState>) -> bool {
    state.refreshing.load(Ordering::SeqCst)
}

#[tauri::command]
fn get_local_usage(state: State<'_, AppState>) -> UsageState {
    state.local_usage.lock().expect("local usage lock").clone()
}

#[tauri::command]
async fn get_local_total_history(app: tauri::AppHandle) -> UsageState {
    let trae_enabled = trae::get_trae_state(app.state::<trae::Service>()).connected;
    let now = now_ms();
    let state = app.state::<AppState>();
    let settings = state.connection.lock().expect("connection lock").clone();
    tauri::async_runtime::spawn_blocking(move || {
        total_history::collect(now, settings, trae_enabled)
    })
    .await
    .unwrap_or_else(|_| UsageState {
        source_unavailable: false,
        status: "error".into(),
        snapshot: None,
        last_success_at_ms: None,
        message: Some("本机总 Token 历史读取中断，请重试。".into()),
    })
}

fn reset_local_usage(app: &tauri::AppHandle, state: &AppState) {
    state.local_epoch.fetch_add(1, Ordering::SeqCst);
    let mut usage = state.local_usage.lock().expect("local usage lock");
    *usage = UsageState::default();
    let _ = app.emit("local-usage-updated", &*usage);
    if let Some(worker) = state
        .local_worker
        .lock()
        .expect("local worker lock")
        .as_ref()
    {
        worker.unpark();
    }
}

async fn refresh_state(app: &tauri::AppHandle, state: &AppState) -> DesktopState {
    let (source, epoch) = {
        let _transition = state
            .connection_transition
            .lock()
            .expect("connection transition lock");
        if !state.connected.load(Ordering::SeqCst) {
            return DesktopState::disconnected();
        }
        if state.refreshing.swap(true, Ordering::SeqCst) {
            return state.current.lock().expect("state lock").clone();
        }
        (
            state.connection.lock().expect("connection lock").resolve(),
            state.local_epoch.load(Ordering::SeqCst),
        )
    };
    if let Some(worker) = state
        .local_worker
        .lock()
        .expect("local worker lock")
        .as_ref()
    {
        worker.unpark();
    }
    let _ = app.emit("refresh-status", true);
    let source_key = source.as_ref().ok().map(connection::Source::key);
    let result = tauri::async_runtime::spawn_blocking(move || {
        source.and_then(|source| codex::read_once(&source))
    })
    .await;
    let _transition = state
        .connection_transition
        .lock()
        .expect("connection transition lock");
    let mut guard = state.current.lock().expect("state lock");
    if !connection::accept_result(
        state.connected.load(Ordering::SeqCst),
        state.local_epoch.load(Ordering::SeqCst),
        epoch,
    ) {
        state.refreshing.store(false, Ordering::SeqCst);
        let _ = app.emit("refresh-status", false);
        return guard.clone();
    }
    let (next, succeeded) = match result {
        Ok(Ok(outcome)) => merge_outcome(&guard, outcome),
        Ok(Err(codex::IDENTITY_CHANGED)) => (
            DesktopState::with_error(&DesktopState::disconnected(), codex::IDENTITY_CHANGED),
            false,
        ),
        Ok(Err("需要连接 Codex 账户")) => (DesktopState::auth_required(), false),
        Ok(Err("当前 Codex 登录类型不支持额度读取")) => {
            (DesktopState::unsupported(), false)
        }
        Ok(Err(message)) => {
            let prior = if guard.identity_verified {
                guard.clone()
            } else {
                DesktopState::disconnected()
            };
            let mut next = DesktopState::with_error(&prior, message);
            next.usage = UsageState::with_error(&prior.usage, message);
            (next, false)
        }
        Err(_) => {
            let prior = if guard.identity_verified {
                guard.clone()
            } else {
                DesktopState::disconnected()
            };
            let mut next = DesktopState::with_error(&prior, "Codex 读取任务意外结束");
            next.usage = UsageState::with_error(&prior.usage, "Codex 读取任务意外结束");
            (next, false)
        }
    };
    *guard = next.clone();
    if let Some(key) = source_key {
        save_cache(&state.cache_path, &next, &key);
    }
    drop(guard);
    update_tray(app, &next);
    if succeeded {
        state.failures.store(0, Ordering::SeqCst);
    } else {
        state.failures.fetch_add(1, Ordering::SeqCst);
    }
    let _ = app.emit("state-updated", &next);
    state.refreshing.store(false, Ordering::SeqCst);
    let _ = app.emit("refresh-status", false);
    if succeeded {
        schedule_alerts(app.clone(), epoch);
    }
    next
}

// Bound native callback waits to one background worker, outside UI/current-state locks.
fn schedule_alerts(app: tauri::AppHandle, epoch: u32) {
    if !app
        .state::<AppState>()
        .alert_worker
        .lock()
        .expect("notification worker lock")
        .request(epoch)
    {
        return;
    }
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        loop {
            let epoch = state
                .alert_worker
                .lock()
                .expect("notification worker lock")
                .take();
            let Some(epoch) = epoch else {
                break;
            };
            process_alerts(&app, &state, epoch);
        }
    });
}

fn process_alerts(app: &tauri::AppHandle, state: &AppState, epoch: u32) {
    // Take the ledger before checking epoch; clear_local_data uses this same
    // lock to delete files, so an invalidated job cannot recreate cleared data.
    let mut ledger = state.alert_ledger.lock().expect("alert ledger lock");
    if !connection::accept_result(
        state.connected.load(Ordering::SeqCst),
        state.local_epoch.load(Ordering::SeqCst),
        epoch,
    ) {
        return;
    }
    let next = state.current.lock().expect("state lock").clone();
    let settings = state
        .alert_settings
        .lock()
        .expect("alert settings lock")
        .clone();
    if !next.identity_verified {
        ledger.identity_unverified();
    } else if let Some(snapshot) = next
        .snapshot
        .as_ref()
        .filter(|snapshot| notification::matches_current(&next, snapshot))
    {
        ledger.process(
            snapshot,
            &settings,
            now_ms(),
            !cfg!(debug_assertions),
            |record| alerts::save_record(&state.alerts_path, record),
            |event| {
                platform::notification::send(app, &event.title, &event.body, false, || {
                    connection::accept_result(
                        state.connected.load(Ordering::SeqCst),
                        state.local_epoch.load(Ordering::SeqCst),
                        epoch,
                    ) && notification::matches_current(
                        &state.current.lock().expect("state lock"),
                        snapshot,
                    )
                })
                .map(|_| ())
                .map_err(|error| error.kind)
            },
        );
    }
    let _ = app.emit("notification-status", &ledger.status);
}

fn merge_outcome(previous: &DesktopState, outcome: codex::ReadOutcome) -> (DesktopState, bool) {
    let same_identity = outcome.identity_verified
        && previous.identity_verified
        && (previous
            .snapshot
            .as_ref()
            .is_some_and(|snapshot| snapshot.identity_key == outcome.identity_key)
            || previous
                .usage
                .snapshot
                .as_ref()
                .is_some_and(|snapshot| snapshot.identity_key == outcome.identity_key));
    let prior = if same_identity {
        previous.clone()
    } else {
        DesktopState::disconnected()
    };
    let quota_succeeded = outcome.quota.is_ok();
    let mut next = match outcome.quota {
        Ok(snapshot) => snapshot,
        Err(message) => DesktopState::with_error(&prior, message),
    };
    next.identity_label = Some(outcome.identity_label);
    next.identity_verified = outcome.identity_verified;
    next.connected = true;
    next.usage = match outcome.usage {
        Ok(snapshot) => UsageState::available(snapshot),
        Err(message) => UsageState::with_error(&prior.usage, message),
    };
    (next, quota_succeeded)
}

#[tauri::command]
async fn refresh_quota(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<DesktopState, String> {
    Ok(refresh_state(&app, &state).await)
}

#[tauri::command]
async fn connect_codex(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<DesktopState, String> {
    {
        let _transition = state
            .connection_transition
            .lock()
            .expect("connection transition lock");
        if state.refreshing.load(Ordering::SeqCst) {
            return Err("上次读取仍在结束，请稍后再试".into());
        }
        let previous = state.connected.swap(true, Ordering::SeqCst);
        if let Err(error) = persist_settings(&state) {
            state.connected.store(previous, Ordering::SeqCst);
            return Err(error);
        }
    }
    Ok(refresh_state(&app, &state).await)
}

#[tauri::command]
fn disconnect_codex(app: tauri::AppHandle, state: State<'_, AppState>) -> DesktopState {
    let _transition = state
        .connection_transition
        .lock()
        .expect("connection transition lock");
    state.connected.store(false, Ordering::SeqCst);
    reset_local_usage(&app, &state);
    let _ = persist_settings(&state);
    let mut guard = state.current.lock().expect("state lock");
    *guard = DesktopState::disconnected();
    let next = guard.clone();
    drop(guard);
    update_tray(&app, &next);
    let _ = app.emit("state-updated", &next);
    next
}

#[tauri::command]
async fn clear_local_data(app: tauri::AppHandle) -> Result<DesktopState, String> {
    let rank_app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        rank_app.state::<leaderboard::Service>().clear_local()
    })
    .await
    .map_err(|_| "清除参榜凭据失败".to_string())??;
    trae::clear(&app)?;
    workbuddy::clear(&app)?;
    providers::clear(&app);
    let hotkey_app = app.clone();
    let dispatch = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        floating_hotkey::on_thread(&dispatch, move || {
            hotkey_app.state::<floating_hotkey::Service>().clear()?;
            floating_hotkey::publish(&hotkey_app);
            Ok(())
        })
    })
    .await
    .map_err(|_| "快捷键清理中断")??;
    let state = app.state::<AppState>();
    let _connection_transition = state
        .connection_transition
        .lock()
        .expect("connection transition lock");
    let _transition = state
        .floating_transition
        .lock()
        .expect("floating transition lock");
    state.floating_enabled.store(false, Ordering::SeqCst);
    if let Some(window) = app.get_webview_window("floating") {
        let _ = window.hide();
    }
    let _ = app.emit("floating-updated", false);
    state.connected.store(false, Ordering::SeqCst);
    reset_local_usage(&app, &state);
    if !cfg!(debug_assertions) {
        let _ = app.autolaunch().disable();
    }
    {
        let _write = state.settings_write.lock().expect("settings write lock");
        let _ = fs::remove_file(&state.settings_path);
        *state.connection.lock().expect("connection lock") = connection::Settings::default();
        *state.floating_mode.lock().expect("floating mode lock") = FloatingMode::default();
        let _ = app.emit("floating-mode-updated", FloatingMode::default());
        state.floating_edge.store(false, Ordering::SeqCst);
        let _ = app.emit("floating-edge-updated", false);
    }
    let _ = fs::remove_file(&state.cache_path);
    let mut ledger = state.alert_ledger.lock().expect("alert ledger lock");
    let _ = fs::remove_file(&state.alerts_path);
    for path in alerts::ledger_paths(&state.alerts_path) {
        let _ = fs::remove_file(path);
    }
    let _ = fs::remove_file(&state.local_index_path);
    for path in local_usage::index_paths(&state.local_index_path) {
        let _ = fs::remove_file(path);
    }
    let _ = fs::remove_file(state.local_index_path.with_extension("tmp"));
    *state.alert_settings.lock().expect("alert settings lock") = AlertSettings::default();
    *ledger = alerts::Delivery::default();
    drop(ledger);
    let _ = app.emit("notification-status", "idle");
    let mut guard = state.current.lock().expect("state lock");
    *guard = DesktopState::disconnected();
    let next = guard.clone();
    drop(guard);
    update_tray(&app, &next);
    let _ = app.emit("state-updated", &next);
    let _ = app.emit("preferences-updated", AlertSettings::default());
    Ok(next)
}

#[tauri::command]
fn get_connection(state: State<'_, AppState>) -> connection::Info {
    connection::Info::new(state.connection.lock().expect("connection lock").clone())
}

#[tauri::command]
fn set_connection(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    settings: connection::Settings,
) -> Result<connection::Info, String> {
    if !cfg!(target_os = "macos") {
        return Err("当前平台尚未开放手动来源设置".into());
    }
    let _transition = state
        .connection_transition
        .lock()
        .expect("connection transition lock");
    connection::can_change(
        state.connected.load(Ordering::SeqCst),
        state.refreshing.load(Ordering::SeqCst),
    )?;
    // Clearing both fields must remain possible even if discovery currently fails.
    if settings.executable.is_some() {
        crate::platform::codex_executable(settings.executable.as_deref())?;
    }
    if let Some(home) = settings.home.as_ref() {
        connection::validate_home(home)?;
    }
    let _write = state.settings_write.lock().expect("settings write lock");
    let mut saved = settings_snapshot(&state);
    saved.connection = settings.clone();
    save_settings(&state.settings_path, &saved)?;
    // Hold the publication lock while replacing settings and invalidating the
    // collector epoch: a worker can no longer checkpoint the previous source.
    {
        let mut usage = state.local_usage.lock().expect("local usage lock");
        *state.connection.lock().expect("connection lock") = settings.clone();
        state.local_epoch.fetch_add(1, Ordering::SeqCst);
        *usage = UsageState::default();
        let _ = app.emit("local-usage-updated", &*usage);
    }
    let next = DesktopState::disconnected();
    *state.current.lock().expect("state lock") = next.clone();
    let _ = fs::remove_file(&state.cache_path);
    state.failures.store(0, Ordering::SeqCst);
    let _ = app.emit("state-updated", &next);
    update_tray(&app, &next);
    if let Some(worker) = state
        .local_worker
        .lock()
        .expect("local worker lock")
        .as_ref()
    {
        worker.unpark();
    }
    Ok(connection::Info::new(settings))
}

#[tauri::command]
fn get_preferences(state: State<'_, AppState>) -> AlertSettings {
    state
        .alert_settings
        .lock()
        .expect("alert settings lock")
        .clone()
}

#[tauri::command]
fn update_preferences(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    settings: AlertSettings,
) -> Result<AlertSettings, String> {
    if !settings.low_threshold.is_finite() || !(1.0..=50.0).contains(&settings.low_threshold) {
        return Err("提醒阈值必须在 1% 到 50% 之间".into());
    }
    *state.alert_settings.lock().expect("alert settings lock") = settings.clone();
    persist_settings(&state)?;
    let _ = app.emit("preferences-updated", &settings);
    Ok(settings)
}

#[tauri::command]
fn get_floating_enabled(state: State<'_, AppState>) -> bool {
    state.floating_enabled.load(Ordering::SeqCst)
}

#[tauri::command]
fn get_floating_supported() -> bool {
    cfg!(any(target_os = "windows", target_os = "macos"))
}

#[tauri::command]
fn get_floating_edge(state: State<'_, AppState>) -> bool {
    state.floating_edge.load(Ordering::SeqCst)
}

#[tauri::command]
fn set_floating_edge(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<bool, String> {
    if !get_floating_supported() {
        return Err("当前平台的悬浮窗尚未实现".into());
    }
    let _write = state.settings_write.lock().expect("settings write lock");
    let mut settings = settings_snapshot(&state);
    settings.floating_edge = enabled;
    save_settings(&state.settings_path, &settings)?;
    state.floating_edge.store(enabled, Ordering::SeqCst);
    let _ = app.emit("floating-edge-updated", enabled);
    Ok(enabled)
}

#[tauri::command]
fn get_floating_mode(state: State<'_, AppState>) -> FloatingMode {
    *state.floating_mode.lock().expect("floating mode lock")
}

#[tauri::command]
fn set_floating_mode(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    mode: FloatingMode,
) -> Result<FloatingMode, String> {
    let _write = state.settings_write.lock().expect("settings write lock");
    let mut settings = settings_snapshot(&state);
    settings.floating_mode = mode;
    save_settings(&state.settings_path, &settings)?;
    *state.floating_mode.lock().expect("floating mode lock") = mode;
    let _ = app.emit("floating-mode-updated", mode);
    Ok(mode)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FloatingPointerState {
    visible: bool,
    inside: bool,
    primary_down: bool,
}

#[tauri::command]
async fn start_floating_drag(window: tauri::WebviewWindow) -> Result<(), String> {
    if window.label() != "floating" {
        return Err("仅悬浮窗可开始拖动".into());
    }
    platform::floating::start(window).await
}

#[tauri::command]
async fn get_floating_pointer_state(
    window: tauri::WebviewWindow,
) -> Result<FloatingPointerState, String> {
    if window.label() != "floating" {
        return Err("仅悬浮窗可检查拖动状态".into());
    }
    let visible = window.is_visible().map_err(|_| "无法读取窗口状态")?;
    let position = window.outer_position().map_err(|_| "无法读取窗口位置")?;
    let size = window.outer_size().map_err(|_| "无法读取窗口大小")?;
    let cursor = platform::floating::pointer_for_window(&window)?;
    let primary_down = platform::floating::primary_down();
    Ok(FloatingPointerState {
        visible,
        inside: visible
            && cursor.x >= position.x as f64
            && cursor.y >= position.y as f64
            && cursor.x < position.x as f64 + size.width as f64
            && cursor.y < position.y as f64 + size.height as f64,
        primary_down,
    })
}

fn change_floating(app: &tauri::AppHandle, enabled: bool) -> Result<bool, String> {
    if !get_floating_supported() {
        return Err("当前平台的悬浮窗尚未实现".into());
    }
    let state = app.state::<AppState>();
    let _transition = state
        .floating_transition
        .lock()
        .expect("floating transition lock");
    let previous = state.floating_enabled.load(Ordering::SeqCst);
    let window = app.get_webview_window("floating").ok_or("悬浮窗暂不可用")?;
    if enabled {
        window.show()
    } else {
        window.hide()
    }
    .map_err(|_| "无法切换悬浮窗".to_string())?;
    state.floating_enabled.store(enabled, Ordering::SeqCst);
    if let Err(error) = persist_settings(&state) {
        state.floating_enabled.store(previous, Ordering::SeqCst);
        let _ = if previous {
            window.show()
        } else {
            window.hide()
        };
        return Err(error);
    }
    let _ = app.emit("floating-updated", enabled);
    let hotkey_app = app.clone();
    app.run_on_main_thread(move || {
        hotkey_app
            .state::<floating_hotkey::Service>()
            .enabled(enabled);
        floating_hotkey::publish(&hotkey_app);
    })
    .map_err(|_| "无法更新快捷键状态")?;
    Ok(enabled)
}

#[tauri::command]
async fn set_floating_enabled(app: tauri::AppHandle, enabled: bool) -> Result<bool, String> {
    change_floating(&app, enabled)
}

#[tauri::command]
fn open_main_panel(app: tauri::AppHandle) {
    show_window(&app);
}

#[tauri::command]
fn expand_floating_window(app: tauri::AppHandle) -> Result<(), String> {
    if !get_floating_supported() {
        return Err("当前平台尚不支持悬浮窗".into());
    }
    if !app
        .state::<AppState>()
        .floating_enabled
        .load(Ordering::SeqCst)
    {
        change_floating(&app, true)?;
    }
    let window = app.get_webview_window("floating").ok_or("悬浮窗暂不可用")?;
    window.show().map_err(|_| "无法显示悬浮窗")?;
    app.state::<AppState>()
        .floating_open_requested
        .store(true, Ordering::SeqCst);
    app.emit_to("floating", "floating-open", ())
        .map_err(|_| "无法展开悬浮窗".into())
}

#[tauri::command]
fn open_hotkey_settings(app: tauri::AppHandle) {
    app.state::<AppState>()
        .hotkey_settings_requested
        .store(true, Ordering::SeqCst);
    show_window(&app);
    let _ = app.emit_to("main", "open-hotkey-settings", ());
}

#[tauri::command]
fn take_floating_open(app: tauri::AppHandle) -> bool {
    app.state::<AppState>()
        .floating_open_requested
        .swap(false, Ordering::SeqCst)
}
#[tauri::command]
fn take_hotkey_settings_request(app: tauri::AppHandle) -> bool {
    app.state::<AppState>()
        .hotkey_settings_requested
        .swap(false, Ordering::SeqCst)
}

#[tauri::command]
fn get_autostart(app: tauri::AppHandle) -> Result<bool, String> {
    if cfg!(debug_assertions) {
        return Ok(false);
    }
    app.autolaunch()
        .is_enabled()
        .map_err(|_| "无法读取开机启动状态".into())
}

#[tauri::command]
fn set_autostart(app: tauri::AppHandle, enabled: bool) -> Result<bool, String> {
    if cfg!(debug_assertions) {
        return Err("开机启动请在安装版中设置".into());
    }
    let manager = app.autolaunch();
    if enabled {
        manager.enable()
    } else {
        manager.disable()
    }
    .map_err(|_| "无法更新开机启动设置".to_string())?;
    manager
        .is_enabled()
        .map_err(|_| "无法确认开机启动状态".into())
}

#[tauri::command]
async fn get_notification_status(app: tauri::AppHandle) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        app.state::<AppState>()
            .alert_ledger
            .lock()
            .expect("alert ledger lock")
            .status
            .clone()
    })
    .await
    .map_err(|_| "无法读取提醒记录状态".into())
}

#[tauri::command]
async fn get_notification_permission() -> Result<notification::Permission, String> {
    tauri::async_runtime::spawn_blocking(platform::notification::permission)
        .await
        .map_err(|_| "读取通知权限任务失败")?
        .map_err(str::to_owned)
}

static NOTIFICATION_ACTION_BUSY: AtomicBool = AtomicBool::new(false);
struct NotificationAction;
impl NotificationAction {
    fn begin() -> Result<Self, String> {
        if NOTIFICATION_ACTION_BUSY.swap(true, Ordering::SeqCst) {
            Err("通知操作正在进行，请稍候".into())
        } else {
            Ok(Self)
        }
    }
}
impl Drop for NotificationAction {
    fn drop(&mut self) {
        NOTIFICATION_ACTION_BUSY.store(false, Ordering::SeqCst);
    }
}

#[tauri::command]
async fn request_notification_permission() -> Result<notification::Permission, String> {
    if cfg!(debug_assertions) {
        return Err("请使用打包后的 TokenDaily 申请系统通知权限".into());
    }
    let guard = NotificationAction::begin()?;
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = guard;
        platform::notification::request_permission().map_err(str::to_owned)
    })
    .await
    .map_err(|_| "通知授权任务失败")?
}

#[tauri::command]
async fn send_test_notification(app: tauri::AppHandle) -> Result<String, String> {
    if cfg!(debug_assertions) {
        return Err("请使用打包后的 TokenDaily 测试系统通知".into());
    }
    let guard = NotificationAction::begin()?;
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = guard;
        platform::notification::send(
            &app,
            "TokenDaily 测试提醒",
            "这是一条测试通知，不代表额度变化。点击可打开面板。",
            true,
            || true,
        )
        .map(str::to_owned)
        .map_err(|error| error.message.to_owned())
    })
    .await
    .map_err(|_| "通知测试任务失败，结果未知，请先检查通知中心")?
}

#[tauri::command]
async fn check_test_notification() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(platform::notification::check_test)
        .await
        .map_err(|_| "检查通知任务失败")?
        .map(str::to_owned)
        .map_err(str::to_owned)
}

fn show_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn spawn_refresh(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        if providers::is_all(&app) {
            let trae_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                trae::refresh_trae(trae_handle).await;
            });
            let wb_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                workbuddy::refresh_workbuddy(wb_handle).await;
            });
            let handle = app.clone();
            tauri::async_runtime::spawn(async move {
                providers::refresh_claude(handle).await;
            });
        }
        if providers::is_trae(&app) {
            trae::refresh_trae(app).await;
            return;
        }
        if providers::is_workbuddy(&app) {
            workbuddy::refresh_workbuddy(app).await;
            return;
        }
        if providers::is_claude(&app) {
            providers::refresh_claude(app).await;
            return;
        }
        let state = app.state::<AppState>();
        refresh_state(&app, &state).await;
    });
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            show_window(app)
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            app_update::get_app_update,
            app_update::check_app_update,
            app_update::install_app_update,
            floating_hotkey::get_floating_hotkey,
            floating_hotkey::set_floating_hotkey,
            floating_hotkey::record_floating_hotkey,
            leaderboard::leaderboard_action,
            providers::get_provider,
            providers::set_provider,
            providers::get_claude_state,
            providers::get_claude_statusline,
            providers::set_claude_statusline,
            providers::get_claude_notification_status,
            providers::get_claude_local_usage,
            providers::get_claude_refreshing,
            providers::connect_claude,
            providers::disconnect_claude,
            providers::refresh_claude,
            trae::get_trae_state,
            trae::get_trae_local_usage,
            trae::get_trae_refreshing,
            trae::connect_trae,
            trae::disconnect_trae,
            trae::refresh_trae,
            trae_credits::get_trae_credits,
            trae_credits::authorize_trae_credits,
            trae_credits::refresh_trae_credits,
            trae_credits::cancel_trae_authorization,
            trae_credits::disconnect_trae_credits,
            workbuddy::get_workbuddy_state,
            workbuddy::get_workbuddy_local_usage,
            workbuddy::get_workbuddy_refreshing,
            workbuddy::connect_workbuddy,
            workbuddy::disconnect_workbuddy,
            workbuddy::refresh_workbuddy,
            workbuddy_credits::get_workbuddy_credits,
            workbuddy_credits::authorize_workbuddy_credits,
            workbuddy_credits::cancel_workbuddy_authorization,
            workbuddy_credits::disconnect_workbuddy_credits,
            workbuddy_credits::refresh_workbuddy_credits,
            get_state,
            get_connection,
            set_connection,
            get_local_usage,
            get_local_total_history,
            get_refreshing,
            refresh_quota,
            connect_codex,
            disconnect_codex,
            clear_local_data,
            get_preferences,
            update_preferences,
            get_autostart,
            set_autostart,
            get_floating_enabled,
            get_floating_supported,
            get_floating_edge,
            set_floating_edge,
            get_floating_mode,
            set_floating_mode,
            get_floating_pointer_state,
            start_floating_drag,
            set_floating_enabled,
            open_main_panel,
            send_test_notification,
            get_notification_status,
            get_notification_permission,
            request_notification_permission,
            check_test_notification,
            expand_floating_window,
            open_hotkey_settings,
            take_floating_open,
            take_hotkey_settings_request
        ])
        .setup(|app| {
            platform::app_update::initialize(app.handle())?;
            if let Some(window) = app.get_webview_window("floating") {
                platform::floating_placement::initialize(&window)?;
                platform::floating::install(window)?;
            }
            platform::notification::install(app.handle())?;
            let data_dir = app.path().app_data_dir()?;
            fs::create_dir_all(&data_dir)?;
            let cache_path = data_dir.join("quota-state.json");
            let settings_path = data_dir.join("settings.json");
            let alerts_path = data_dir.join(if cfg!(debug_assertions) {
                "alerts-dev.json"
            } else {
                "alerts.json"
            });
            let local_settings = load_settings(&settings_path);
            app.manage(providers::Providers::new(&data_dir));
            app.manage(trae::Service::new(&data_dir));
            app.manage(trae_credits::Credits::new(&data_dir));
            app.manage(workbuddy::Service::new(&data_dir));
            app.manage(workbuddy_credits::Credits::new(&data_dir));
            app.manage(leaderboard::Service::new(&data_dir));
            let connected = local_settings.connected;
            let initial = if connected {
                load_cache(
                    &cache_path,
                    local_settings
                        .connection
                        .resolve()
                        .ok()
                        .map(|source| source.key())
                        .as_deref(),
                )
            } else {
                DesktopState::disconnected()
            };
            app.manage(AppState {
                connection: Mutex::new(local_settings.connection),
                connection_transition: Mutex::new(()),
                local_usage: Mutex::new(UsageState::default()),
                local_epoch: AtomicU32::new(0),
                local_worker: Mutex::new(None),
                current: Mutex::new(initial),
                refreshing: AtomicBool::new(false),
                failures: AtomicU32::new(0),
                connected: AtomicBool::new(connected),
                floating_enabled: AtomicBool::new(local_settings.floating_enabled),
                floating_open_requested: AtomicBool::new(false),
                hotkey_settings_requested: AtomicBool::new(false),
                floating_edge: AtomicBool::new(local_settings.floating_edge),
                floating_mode: Mutex::new(local_settings.floating_mode),
                floating_transition: Mutex::new(()),
                settings_write: Mutex::new(()),
                cache_path,
                settings_path,
                alert_settings: Mutex::new(local_settings.alerts),
                alert_ledger: Mutex::new(alerts::load_delivery(&alerts_path)),
                alert_worker: Mutex::new(notification::WorkerGate::default()),
                alerts_path,
                local_index_path: data_dir.join("local-usage-index.json"),
            });
            app.manage(floating_hotkey::Service::new(
                app.handle().clone(),
                &data_dir,
                local_settings.floating_enabled,
            ));
            if local_settings.floating_enabled {
                // Both workers share product rules but keep independent source state.
                if let Some(window) = app.get_webview_window("floating") {
                    window.show()?;
                }
            }
            let open = MenuItem::with_id(app, "open", "打开设置", true, None::<&str>)?;
            let floating_open = MenuItem::with_id(
                app,
                "floating-open",
                "展开悬浮窗",
                get_floating_supported(),
                None::<&str>,
            )?;
            let hotkey_settings = MenuItem::with_id(
                app,
                "hotkey-settings",
                "设置快捷键…",
                get_floating_supported(),
                None::<&str>,
            )?;
            providers::start(app.handle().clone());
            trae::start(app.handle().clone());
            trae_credits::start(app.handle().clone());
            workbuddy::start(app.handle().clone());
            workbuddy_credits::start(app.handle().clone());
            leaderboard::start(app.handle().clone());
            let refresh = MenuItem::with_id(app, "refresh", "刷新", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出 TokenDaily", true, None::<&str>)?;
            let menu = Menu::with_items(
                app,
                &[&floating_open, &hotkey_settings, &open, &refresh, &quit],
            )?;
            // Full-bleed brand alpha works in small tray/menu-bar slots.
            let tray_icon = tauri::include_image!("icons/tray/64x64.png");
            TrayIconBuilder::with_id("main")
                .icon(tray_icon)
                .icon_as_template(cfg!(target_os = "macos"))
                .tooltip("TokenDaily · 额度状态待获取")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "open" => {
                        show_window(app);
                        let _ = app.emit_to("main", "open-settings", ());
                    }
                    "floating-open" => {
                        if let Err(error) = expand_floating_window(app.clone()) {
                            show_window(app);
                            let _ = app.emit_to("main", "open-settings", ());
                            let _ = app.emit_to("main", "floating-error", error);
                        }
                    }
                    "hotkey-settings" => open_hotkey_settings(app.clone()),
                    "refresh" => {
                        show_window(app);
                        let _ = app.emit_to("main", "open-settings", ());
                        spawn_refresh(app.clone());
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                show_window(app);
                                let _ = app.emit_to("main", "open-settings", ());
                            }
                        }
                    }
                })
                .build(app)?;
            update_tray(
                app.handle(),
                &app.state::<AppState>().current.lock().expect("state lock"),
            );
            let handle = app.handle().clone();
            let local_handle = handle.clone();
            std::thread::spawn(move || {
                let Some(wake) = log_watch::Wake::new() else {
                    return;
                };
                *local_handle
                    .state::<AppState>()
                    .local_worker
                    .lock()
                    .expect("local worker lock") = Some(wake.clone());
                let mut home = PathBuf::new();
                let index = local_handle.state::<AppState>().local_index_path.clone();
                let mut collector = local_usage::Collector::new(home.clone());
                let mut watch = log_watch::Watch::new(wake);
                let mut watching = false;
                let mut periodic = now_ms();
                let mut dirty = None;
                let mut force = true;
                let mut save_at = 0;
                let mut hot_at = now_ms() + 2000;
                let mut collected_day = String::new();
                let mut previous_clock = now_ms();
                let mut epoch = 0;
                loop {
                    let state = local_handle.state::<AppState>();
                    let current_epoch = state.local_epoch.load(Ordering::SeqCst);
                    if current_epoch != epoch {
                        collector.reset();
                        epoch = current_epoch;
                        force = true;
                    }
                    if !state.connected.load(Ordering::SeqCst) {
                        watch.stop();
                        watching = false;
                        watch.wait(Duration::from_secs(30));
                        continue;
                    }
                    let selected_home = state.connection.lock().expect("connection lock").home();
                    let selected_home = match selected_home {
                        Ok(home) => home,
                        Err(message) => {
                            watch.stop();
                            watching = false;
                            let mut current = state.local_usage.lock().expect("local usage lock");
                            if state.connected.load(Ordering::SeqCst)
                                && state.local_epoch.load(Ordering::SeqCst) == epoch
                            {
                                *current = UsageState::with_error(&UsageState::default(), message);
                                let _ = local_handle.emit("local-usage-updated", &*current);
                            }
                            drop(current);
                            watch.wait(Duration::from_secs(30));
                            continue;
                        }
                    };
                    if home != selected_home {
                        home = selected_home;
                        collector = local_usage::Collector::restore(home.clone(), &index);
                        watch.stop();
                        watching = false;
                        force = true;
                        save_at = 0;
                    }
                    let now = now_ms();
                    if now < previous_clock {
                        periodic = now;
                        hot_at = now;
                    }
                    previous_clock = now;
                    if now >= hot_at {
                        force |= collector.hot_changed();
                        hot_at = now + 2000;
                    }
                    force |= collected_day != local_usage::beijing_day(now);
                    if !watching || now >= periodic {
                        watch.reset(&home);
                        watching = true;
                        periodic = now + log_watch::RECONCILE_MS;
                        force = true;
                    }
                    if force || log_watch::wait_ms(now, periodic, dirty) == 0 {
                        let mut next = collector.poll(now_ms());
                        next.message = next
                            .message
                            .or_else(|| log_watch::warning(&watch).map(str::to_owned));
                        collected_day = local_usage::beijing_day(now);
                        if next.status == "available" {
                            let completed_at = now_ms();
                            next.last_success_at_ms = Some(completed_at);
                            if let Some(snapshot) = next.snapshot.as_mut() {
                                snapshot.collected_at_ms = completed_at;
                            }
                        }
                        let mut current = state.local_usage.lock().expect("local usage lock");
                        if state.connected.load(Ordering::SeqCst)
                            && state.local_epoch.load(Ordering::SeqCst) == epoch
                        {
                            if next.status == "available" && collector.dirty && now_ms() >= save_at
                            {
                                let _ = collector.checkpoint(&index);
                                save_at = now_ms() + 30_000;
                            }
                            *current = next;
                            let _ = local_handle.emit("local-usage-updated", &*current);
                        }
                        force = false;
                        dirty = None;
                    }
                    match watch.wait(Duration::from_millis(
                        log_watch::wait_ms(now_ms(), periodic.min(hot_at), dirty).min(30_000),
                    )) {
                        log_watch::Signal::Manual => force = true,
                        log_watch::Signal::Changed => {
                            dirty.get_or_insert(now_ms() + log_watch::COALESCE_MS);
                        }
                        log_watch::Signal::Timeout => {}
                    }
                }
            });
            std::thread::spawn(move || loop {
                let failures = handle
                    .state::<AppState>()
                    .failures
                    .load(Ordering::SeqCst)
                    .min(4);
                let delay_secs = (300_u64 * (1_u64 << failures)).min(3600);
                std::thread::sleep(Duration::from_secs(delay_secs));
                if handle.state::<AppState>().connected.load(Ordering::SeqCst) {
                    spawn_refresh(handle.clone());
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "trae-official" {
                return;
            }
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                if window.label() == "floating" {
                    let app = window.app_handle().clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(message) = change_floating(&app, false) {
                            let _ = app.emit("floating-error", message);
                        }
                    });
                } else {
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("TokenDaily failed to start");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "tokendaily-{name}-{}-{stamp}.json",
            std::process::id()
        ))
    }

    #[test]
    fn connection_settings_default_and_round_trip() {
        let path = temp_file("connection");
        fs::write(&path, r#"{"connected":false}"#).unwrap();
        let mut settings = load_settings(&path);
        assert_eq!(settings.connection, connection::Settings::default());
        settings.connection.executable = Some("/example/Codex CLI/codex".into());
        settings.connection.home = Some("/example/codex home".into());
        save_settings(&path, &settings).unwrap();
        assert_eq!(load_settings(&path).connection, settings.connection);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn source_changes_require_a_disconnected_idle_reader() {
        assert!(connection::can_change(false, false).is_ok());
        for (connected, refreshing) in [(true, false), (false, true), (true, true)] {
            assert!(connection::can_change(connected, refreshing).is_err());
        }
        assert!(!connection::accept_result(true, 2, 1));
        assert!(!connection::accept_result(false, 1, 1));
        assert!(connection::accept_result(true, 2, 2));
    }

    #[test]
    fn settings_connection_flag_round_trips() {
        let path = temp_file("settings");
        assert!(!load_settings(&path).connected);
        save_settings(
            &path,
            &LocalSettings {
                connected: true,
                alerts: AlertSettings::default(),
                ..LocalSettings::default()
            },
        )
        .unwrap();
        assert!(load_settings(&path).connected);
        save_settings(
            &path,
            &LocalSettings {
                connected: false,
                alerts: AlertSettings::default(),
                ..LocalSettings::default()
            },
        )
        .unwrap();
        assert!(!load_settings(&path).connected);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn floating_window_is_opt_in_for_existing_settings_and_survives_restart() {
        let path = temp_file("floating-settings");
        fs::write(&path, br#"{"connected":true}"#).unwrap();
        let mut settings = load_settings(&path);
        assert!(settings.connected);
        assert!(!settings.floating_enabled);
        settings.floating_enabled = true;
        save_settings(&path, &settings).unwrap();
        assert!(load_settings(&path).floating_enabled);
        assert!(load_settings(&path).connected);
        settings.floating_enabled = false;
        save_settings(&path, &settings).unwrap();
        assert!(!load_settings(&path).floating_enabled);
        assert_eq!(
            load_settings(&path).alerts.low_threshold,
            settings.alerts.low_threshold
        );
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn settings_write_failure_is_reported_instead_of_claiming_saved() {
        let missing_parent = temp_file("missing-parent").join("settings.json");
        assert!(save_settings(&missing_parent, &LocalSettings::default()).is_err());
    }

    #[test]
    fn floating_mode_defaults_and_round_trips_without_changing_other_preferences() {
        let path = temp_file("floating-mode");
        fs::write(&path, r#"{"connected":true,"floating_enabled":true}"#).unwrap();
        let mut settings = load_settings(&path);
        assert_eq!(settings.floating_mode, FloatingMode::Usage);
        for mode in [FloatingMode::Quota, FloatingMode::Usage] {
            settings.floating_mode = mode;
            save_settings(&path, &settings).unwrap();
            let saved = load_settings(&path);
            assert_eq!(saved.floating_mode, mode);
            assert!(saved.connected && saved.floating_enabled);
            assert_eq!(saved.alerts.low_threshold, settings.alerts.low_threshold);
        }
        assert!(serde_json::from_str::<FloatingMode>(r#""invalid""#).is_err());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn edge_hiding_is_opt_in_and_persists_without_changing_other_settings() {
        let path = temp_file("floating-edge");
        fs::write(
            &path,
            r#"{"connected":true,"floating_enabled":true,"floating_mode":"quota"}"#,
        )
        .unwrap();
        let mut settings = load_settings(&path);
        assert!(!settings.floating_edge);
        for enabled in [true, false] {
            settings.floating_edge = enabled;
            save_settings(&path, &settings).unwrap();
            let saved = load_settings(&path);
            assert_eq!(saved.floating_edge, enabled);
            assert!(saved.connected && saved.floating_enabled);
            assert_eq!(saved.floating_mode, FloatingMode::Quota);
            assert_eq!(saved.alerts.low_threshold, settings.alerts.low_threshold);
        }
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn cached_quota_is_always_marked_old_until_identity_is_checked() {
        let path = temp_file("cache");
        let fixture = serde_json::json!({
            "status":"available", "connected":true, "identityLabel":"a***@example.com", "identityVerified":true,
            "lastSuccessAtMs":1792051200000_i64, "message":null,
            "snapshot":{ "identityKey":"codex:opaque:", "status":"available",
                "collectedAtMs":1792051200000_i64,
                "windows":[{"poolId":"codex","windowId":"primary","label":"1 小时额度",
                    "usedPercent":73.0,"remainingPercent":27.0,"resetsAtMs":1792054800000_i64,
                    "durationMinutes":60,"reachedType":null}] }
        });
        fs::write(
            &path,
            serde_json::to_vec(&serde_json::json!({"sourceKey":"source-a", "state":fixture}))
                .unwrap(),
        )
        .unwrap();
        let mut legacy = fixture.clone();
        legacy.as_object_mut().unwrap().remove("identityVerified");
        let legacy_path = temp_file("legacy-cache");
        fs::write(
            &legacy_path,
            serde_json::to_vec(&serde_json::json!({"sourceKey":"source-a","state":legacy}))
                .unwrap(),
        )
        .unwrap();
        assert!(load_cache(&legacy_path, Some("source-a"))
            .snapshot
            .is_none());
        fs::remove_file(legacy_path).unwrap();
        let cached = load_cache(&path, Some("source-a"));
        assert!(load_cache(&path, Some("source-b")).snapshot.is_none());
        assert!(load_cache(&path, None).snapshot.is_none());
        assert_eq!(cached.status, "stale");
        assert!(cached.message.unwrap().contains("身份待核对"));
        assert!(cached.snapshot.is_some());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn quota_failure_does_not_discard_new_daily_usage() {
        let snapshot = usage::normalize(
            &serde_json::json!({"dailyUsageBuckets":[{"startDate":"2026-10-15","tokens":42}]}),
            "codex:a:".into(),
            123,
        )
        .unwrap();
        let outcome = codex::ReadOutcome {
            identity_key: "codex:a:".into(),
            identity_label: "a***@example.com".into(),
            identity_verified: true,
            quota: Err("额度失败"),
            usage: Ok(snapshot),
        };
        let (state, quota_succeeded) = merge_outcome(&DesktopState::disconnected(), outcome);
        assert!(!quota_succeeded);
        assert_eq!(state.status, "error");
        assert_eq!(state.usage.status, "available");
        assert_eq!(state.usage.snapshot.unwrap().days[0].tokens, 42);
    }

    #[test]
    fn usage_failure_keeps_same_identity_history_but_account_change_discards_it() {
        let mut prior = DesktopState::disconnected();
        prior.identity_verified = true;
        prior.usage = UsageState::available(
            usage::normalize(
                &serde_json::json!({"dailyUsageBuckets":[{"startDate":"2026-10-15","tokens":42}]}),
                "codex:a:".into(),
                123,
            )
            .unwrap(),
        );
        let same = codex::ReadOutcome {
            identity_key: "codex:a:".into(),
            identity_label: "a***@example.com".into(),
            identity_verified: true,
            quota: Err("额度失败"),
            usage: Err("用量失败"),
        };
        let (stale, _) = merge_outcome(&prior, same);
        assert_eq!(stale.usage.status, "stale");
        assert_eq!(stale.usage.snapshot.as_ref().unwrap().days[0].tokens, 42);
        let unverified = codex::ReadOutcome {
            identity_key: "codex:a:".into(),
            identity_label: "a***@example.com".into(),
            identity_verified: false,
            quota: Err("额度失败"),
            usage: Err("用量失败"),
        };
        assert!(merge_outcome(&prior, unverified).0.usage.snapshot.is_none());
        let switched = codex::ReadOutcome {
            identity_key: "codex:b:".into(),
            identity_label: "b***@example.com".into(),
            identity_verified: true,
            quota: Err("额度失败"),
            usage: Err("用量失败"),
        };
        let (state, _) = merge_outcome(&prior, switched);
        assert!(state.usage.snapshot.is_none());
        assert_eq!(state.usage.status, "error");
    }

    #[test]
    fn tray_tooltip_distinguishes_fresh_small_balance_from_old_data() {
        let mut state = DesktopState::disconnected();
        state.status = "available".into();
        state.snapshot = Some(codex::QuotaSnapshot {
            identity_key: "opaque".into(),
            status: "available".into(),
            collected_at_ms: 1,
            windows: vec![codex::QuotaWindow {
                pool_id: "codex".into(),
                window_id: "primary".into(),
                label: "周期额度".into(),
                used_percent: Some(99.6),
                remaining_percent: Some(0.4),
                resets_at_ms: Some(2),
                duration_minutes: None,
                reached_type: None,
            }],
        });
        assert!(tray_tooltip(&state).contains("<1%"));
        state.status = "stale".into();
        assert!(tray_tooltip(&state).contains("上次成功数据"));
        assert!(!tray_tooltip(&state).contains("<1%"));
    }
}

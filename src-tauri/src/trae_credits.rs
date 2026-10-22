//! Official TRAE dashboard in TokenDaily's isolated WebView profile.
//! We observe quota response numbers only. Cookies/tokens never enter Rust or React.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::PathBuf, sync::Mutex, thread, time::Duration};
#[cfg(target_os = "macos")]
use std::{process::Command, sync::OnceLock};
use tauri::{Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

const WINDOW: &str = "trae-official";
const DASHBOARD: &str = "https://www.trae.cn/dashboard#usage";
// macOS 14+ uses a dedicated persistent WKWebsiteDataStore for TRAE.
// It is separate from the default app WebView and every other provider.
#[cfg(target_os = "macos")]
const MAC_DATA_STORE_ID: [u8; 16] = [
    0xa6, 0xe6, 0xf9, 0x49, 0x78, 0x49, 0x4b, 0x9d, 0xa2, 0x1e, 0x0f, 0x4e, 0x63, 0x8d, 0x71, 0x84,
];
#[cfg(target_os = "macos")]
fn mac_store_supported() -> bool {
    static SUPPORTED: OnceLock<bool> = OnceLock::new();
    *SUPPORTED.get_or_init(|| {
        Command::new("/usr/bin/sw_vers")
            .arg("-productVersion")
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|version| version.split('.').next()?.trim().parse::<u32>().ok())
            .is_some_and(|major| major >= 14)
    })
}
fn platform_supported() -> bool {
    #[cfg(target_os = "windows")]
    {
        true
    }
    #[cfg(target_os = "macos")]
    {
        mac_store_supported()
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        false
    }
}
const TRAE_CACHE_SCOPE: &str = "trae-cn-official-profile-v1";
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Package {
    name: String,
    remaining: String,
    expires_at_ms: Option<i64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Balance {
    remaining: String,
    packages: Vec<Package>,
    next_expiry_at_ms: Option<i64>,
    collected_at_ms: i64,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreditState {
    status: String,
    supported: bool,
    authorized: bool,
    refreshing: bool,
    identity_label: Option<String>,
    balance: Option<Balance>,
    message: String,
}
impl Default for CreditState {
    fn default() -> Self {
        let supported = platform_supported();
        Self {
            status: if supported {
                "auth-required"
            } else {
                "unsupported"
            }
            .into(),
            supported,
            authorized: false,
            refreshing: false,
            identity_label: None,
            balance: None,
            message: if supported {
                "在设置中登录 TRAE 官方账户后读取积分。"
            } else {
                "当前系统暂不支持独立的 TRAE 官方授权窗口（Mac 需要 macOS 14 或更新版本）。"
            }
            .into(),
        }
    }
}
struct Inner {
    state: CreditState,
    enabled: bool,
    nonce: Option<String>,
    deadline: i64,
    last_attempt: i64,
    failed: bool,
    auth_failures: u8,
    visible_login: bool,
    /// Nonce baked into the open official window's injected script, if any.
    window_nonce: Option<String>,
}
pub(crate) struct Credits {
    inner: Mutex<Inner>,
    path: PathBuf,
    #[cfg(target_os = "windows")]
    profile: PathBuf,
    cache_path: PathBuf,
}
fn valid_cached_balance(b: &Balance) -> bool {
    let now = crate::now_ms();
    b.collected_at_ms > 0
        && b.collected_at_ms <= now.saturating_add(60_000)
        && b.packages.len() <= 200
        && decimal(&Value::String(b.remaining.clone())).is_ok()
        && b.packages.iter().all(|p| {
            p.name.chars().count() <= 80
                && decimal(&Value::String(p.remaining.clone())).is_ok()
                && p.expires_at_ms.is_none_or(|t| t > 0)
        })
        && b.next_expiry_at_ms.is_none_or(|t| t > 0)
}
impl Credits {
    pub(crate) fn new(dir: &std::path::Path) -> Self {
        let path = dir.join("trae-credits.json");
        let enabled = platform_supported() && fs::read(&path).is_ok_and(|b| b == b"true");
        let profile = dir.join("trae-official-profile");
        let cache_path = dir.join(if cfg!(target_os = "macos") {
            "trae-last-balance.json"
        } else {
            "trae-last-balance.protected"
        });
        let mut state = CreditState::default();
        if enabled {
            // The macOS WKWebsiteDataStore is identified by UUID rather than by
            // the Windows profile directory. Its enable flag is the local scope.
            if cfg!(target_os = "macos") || profile.exists() {
                state.balance = crate::credit_cache::load_last(&cache_path, TRAE_CACHE_SCOPE)
                    .filter(valid_cached_balance);
            }
            state.status = if state.balance.is_some() {
                "stale"
            } else {
                "loading"
            }
            .into();
            state.message = if state.balance.is_some() {
                "显示上次成功查询的积分，正在向官方更新"
            } else {
                "正在恢复官方积分连接"
            }
            .into();
        }
        Self {
            inner: Mutex::new(Inner {
                state,
                enabled,
                nonce: None,
                deadline: 0,
                last_attempt: 0,
                failed: false,
                auth_failures: 0,
                visible_login: false,
                window_nonce: None,
            }),
            path,
            #[cfg(target_os = "windows")]
            profile,
            cache_path,
        }
    }
}
fn decimal(v: &Value) -> Result<i64, &'static str> {
    let text = v
        .as_number()
        .map(|n| n.to_string())
        .or_else(|| v.as_str().map(str::to_owned))
        .ok_or("TRAE 积分数值缺失")?;
    let mut pieces = text.split('.');
    let whole = pieces.next().unwrap();
    let frac = pieces.next().unwrap_or("");
    if pieces.next().is_some()
        || whole.is_empty()
        || frac.len() > 6
        || !whole
            .bytes()
            .chain(frac.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return Err("TRAE 积分精度或格式未知");
    }
    whole
        .parse::<i64>()
        .ok()
        .and_then(|n| n.checked_mul(1_000_000))
        .and_then(|n| n.checked_add(format!("{frac:0<6}").parse::<i64>().ok()?))
        .ok_or("TRAE 积分超出范围")
}
fn display(n: i64) -> String {
    format!("{}.{:06}", n / 1_000_000, n % 1_000_000)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .into()
}
fn parse_balance(v: &Value, now: i64) -> Result<Balance, &'static str> {
    if v["is_credits_billing"] != true || v["is_dollar_usage_billing"] == true {
        return Err("当前 TRAE 计费类型不是已支持的积分余额");
    }
    let rows = v["user_entitlement_pack_list"]
        .as_array()
        .filter(|a| a.len() <= 200)
        .ok_or("TRAE 积分包缺失或过多")?;
    let mut packages = Vec::new();
    let mut total = 0i64;
    let mut seen = std::collections::HashMap::new();
    for row in rows {
        if row["is_last_period"] == true {
            continue;
        }
        let base = &row["entitlement_base_info"];
        let Some(limit) = base["quota"].get("credits_limit") else {
            continue;
        };
        let limit = decimal(limit)?;
        let expiry = base["end_time"]
            .as_i64()
            .or_else(|| row["expire_time"].as_i64())
            .filter(|n| *n > 0)
            .map(|n| n.checked_mul(1000).ok_or("TRAE 有效期超限"))
            .transpose()?;
        if expiry.is_some_and(|t| t <= now) {
            continue;
        }
        let id = &base["entitlement_id"];
        if !id.is_string() && !id.is_number() {
            return Err("TRAE 积分包标识缺失");
        }
        let key = id.to_string();
        let canonical = row.to_string();
        if let Some(previous) = seen.insert(key, canonical.clone()) {
            if previous != canonical {
                return Err("TRAE 积分包重复且数据冲突");
            }
            continue;
        }
        let usage = row["usage"].as_object().ok_or("TRAE 积分使用量缺失")?;
        let used = usage
            .get("credits_amount")
            .map(decimal)
            .transpose()?
            .unwrap_or(0);
        let remaining = limit.saturating_sub(used).max(0);
        total = total.checked_add(remaining).ok_or("TRAE 积分合计超限")?;
        packages.push(Package {
            name: row["display_desc"]
                .as_str()
                .filter(|s| !s.is_empty() && s.chars().count() < 80)
                .unwrap_or("官方积分包")
                .into(),
            remaining: display(remaining),
            expires_at_ms: expiry,
        });
    }
    if packages.is_empty() {
        return Err("未返回可识别的有效 TRAE 积分包");
    }
    let next_expiry_at_ms = packages
        .iter()
        .filter(|p| p.remaining != "0")
        .filter_map(|p| p.expires_at_ms)
        .min();
    Ok(Balance {
        remaining: display(total),
        packages,
        next_expiry_at_ms,
        collected_at_ms: now,
    })
}
fn emit(app: &tauri::AppHandle, inner: &Inner) {
    let _ = app.emit("trae-credits-updated", &inner.state);
}
#[tauri::command]
pub(crate) fn get_trae_credits(s: State<'_, Credits>) -> CreditState {
    s.inner.lock().unwrap().state.clone()
}
fn failure(app: &tauri::AppHandle, message: &str) {
    let s = app.state::<Credits>();
    let mut i = s.inner.lock().unwrap();
    i.nonce = None;
    i.state.refreshing = false;
    i.failed = true;
    i.auth_failures = 0;
    i.visible_login = false;
    i.state.status = if i.state.balance.is_some() {
        "stale"
    } else {
        "error"
    }
    .into();
    i.state.message = message.into();
    emit(app, &i);
}

fn retry_due(now: i64, last: i64, manual: bool, failed: bool) -> bool {
    now.saturating_sub(last)
        >= if manual {
            10_000
        } else if failed {
            60_000
        } else {
            300_000
        }
}

fn auth_failure(i: &mut Inner) -> bool {
    if i.visible_login {
        // An unauthenticated page is expected while the user is signing in.
        // Keep observing this window for a successful response until its deadline.
        i.state.message = "请在官方页面完成登录，登录后将自动读取积分。".into();
        return false;
    }
    i.nonce = None;
    i.state.refreshing = false;
    i.failed = true;
    i.auth_failures = i.auth_failures.saturating_add(1);
    if i.auth_failures >= 2 {
        i.state.status = "auth-required".into();
        i.state.authorized = false;
        i.state.balance = None;
        i.state.message = "TRAE 官方登录已失效，请重新登录。".into();
        return true;
    }
    i.state.status = if i.state.balance.is_some() {
        "stale"
    } else {
        "error"
    }
    .into();
    i.state.message = "TRAE 官方返回授权异常，正在复核；可稍后手动刷新。".into();
    false
}

fn script(nonce: &str) -> String {
    // No request headers, cookies, response identifiers or messages are copied.
    let template = r#"(()=>{
      if(window.top!==window || location.origin!=='https://www.trae.cn')return;
      const nonce=__NONCE__;
      const path='/trae/api/v2/pay/user_current_entitlement_list';
      const send=value=>location.href='tokendaily-trae://quota#'+encodeURIComponent(JSON.stringify({nonce,value}));
      const accept=(url,status,text)=>{
        try{const u=new URL(url,location.href);if(u.origin!=='https://api.trae.cn'||u.pathname!==path)return;
          if(status===401||status===403){send({authRequired:true});return;}
          if(status!==200)return;const data=JSON.parse(text);
          if(!Array.isArray(data.user_entitlement_pack_list))return;
          send({is_credits_billing:data.is_credits_billing,is_dollar_usage_billing:data.is_dollar_usage_billing,
            user_entitlement_pack_list:data.user_entitlement_pack_list.slice(0,201).map(p=>({is_last_period:p.is_last_period,
              display_desc:typeof p.display_desc==='string'?p.display_desc.slice(0,80):undefined,expire_time:p.expire_time,
              entitlement_base_info:{entitlement_id:p.entitlement_base_info?.entitlement_id,end_time:p.entitlement_base_info?.end_time,quota:{credits_limit:p.entitlement_base_info?.quota?.credits_limit}},
              usage:p.usage&&typeof p.usage==='object'?{credits_amount:p.usage.credits_amount}:null}))});
        }catch{}
      };
      const original=window.fetch;window.fetch=async function(...args){const response=await original.apply(this,args);if(response.url.includes(path))void response.clone().text().then(t=>accept(response.url,response.status,t)).catch(()=>{});return response;};
      const open=XMLHttpRequest.prototype.open,sendXHR=XMLHttpRequest.prototype.send;
      XMLHttpRequest.prototype.open=function(...args){return open.apply(this,args);};
      XMLHttpRequest.prototype.send=function(...args){this.addEventListener('load',()=>{try{accept(this.responseURL,this.status,this.responseType==='json'?JSON.stringify(this.response):this.responseText);}catch{}});return sendXHR.apply(this,args);};
    })();"#;
    template.replace("__NONCE__", &serde_json::to_string(nonce).unwrap())
}
fn callback(app: &tauri::AppHandle, url: &reqwest::Url) {
    let Some(fragment) = url.fragment().filter(|v| v.len() < 100_000) else {
        return;
    };
    let decoded = match percent_decode(fragment) {
        Some(s) => s,
        None => return,
    };
    let Ok(v) = serde_json::from_str::<Value>(&decoded) else {
        return;
    };
    let s = app.state::<Credits>();
    let mut i = s.inner.lock().unwrap();
    if i.nonce.as_deref() != v["nonce"].as_str()
        || i.nonce.is_none()
        || crate::now_ms() > i.deadline
    {
        return;
    }
    // The callback must originate from our own official dashboard WebView.
    if !app
        .get_webview_window(WINDOW)
        .and_then(|w| w.url().ok())
        .is_some_and(|u| u.origin().ascii_serialization() == "https://www.trae.cn")
    {
        return;
    }
    if v["value"]["authRequired"] == true {
        if auth_failure(&mut i) {
            let _ = crate::credit_cache::clear_last(&s.cache_path);
        }
        emit(app, &i);
        return;
    }
    match parse_balance(&v["value"], crate::now_ms()) {
        Ok(balance) => {
            if fs::write(&s.path, b"true").is_err() {
                drop(i);
                failure(app, "TRAE 连接设置保存失败");
                return;
            }
            i.enabled = true;
            i.nonce = None;
            i.state.authorized = true;
            i.state.refreshing = false;
            i.state.status = "available".into();
            i.state.identity_label = Some("TRAE 官方账户".into());
            let saved = crate::credit_cache::save_last(&s.cache_path, TRAE_CACHE_SCOPE, &balance);
            i.state.balance = Some(balance);
            i.state.message = if saved.is_ok() {
                "官方账户积分；与本地 Token 记录分别统计，有效期不代表重置。".into()
            } else {
                "官方积分已读取，但上次结果保存失败".into()
            };
            i.failed = false;
            i.auth_failures = 0;
            i.visible_login = false;
            emit(app, &i);
            drop(i);
            if let Some(w) = app.get_webview_window(WINDOW) {
                let _ = w.hide();
            }
        }
        Err(message) => {
            drop(i);
            failure(app, message);
        }
    }
}
fn percent_decode(text: &str) -> Option<String> {
    let mut bytes = Vec::new();
    let mut it = text.bytes();
    while let Some(b) = it.next() {
        if b == b'%' {
            let hi = (it.next()? as char).to_digit(16)?;
            let lo = (it.next()? as char).to_digit(16)?;
            bytes.push((hi * 16 + lo) as u8);
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8(bytes).ok()
}

// macOS keeps the native window of a destroyed WebView alive, so rebuilding the
// hidden official window on every refresh leaks one window per attempt there.
// Reuse the open window and its injected script instead.
const REUSE_WINDOW: bool = cfg!(target_os = "macos");

/// Returns the nonce for a new attempt and whether the open window can be reused.
fn attempt_nonce(
    reuse_supported: bool,
    window_open: bool,
    window_nonce: Option<&str>,
    fresh: String,
) -> (String, bool) {
    match window_nonce {
        Some(nonce) if reuse_supported && window_open => (nonce.to_owned(), true),
        _ => (fresh, false),
    }
}

#[derive(Debug, PartialEq)]
enum Reload {
    /// The dashboard is open: reload so the injected script observes a new request.
    Page,
    /// Navigating to the same `#usage` URL would only change the fragment.
    Dashboard,
}
fn reload_action(current: Option<&reqwest::Url>) -> Reload {
    match current {
        Some(url)
            if url.origin().ascii_serialization() == "https://www.trae.cn"
                && url.path() == "/dashboard" =>
        {
            Reload::Page
        }
        _ => Reload::Dashboard,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn reuse(app: &tauri::AppHandle, visible: bool) -> bool {
    let Some(window) = app.get_webview_window(WINDOW) else {
        return false;
    };
    let reloaded = match reload_action(window.url().ok().as_ref()) {
        Reload::Page => window.reload(),
        Reload::Dashboard => window.navigate(DASHBOARD.parse().unwrap()),
    };
    if reloaded.is_err() {
        return false;
    }
    if visible {
        let _ = window.show();
        let _ = window.set_focus();
    }
    true
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn open(
    app: &tauri::AppHandle,
    visible: bool,
    nonce: &str,
    reuse_window: bool,
) -> Result<(), String> {
    if reuse_window && reuse(app, visible) {
        return Ok(());
    }
    if let Some(window) = app.get_webview_window(WINDOW) {
        window.destroy().map_err(|_| "无法重建 TRAE 官方窗口")?;
        // Window destruction reaches the manager asynchronously on Windows.
        // Do not race a new WebView with the previous window's label/profile.
        for _ in 0..100 {
            if app.get_webview_window(WINDOW).is_none() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        if app.get_webview_window(WINDOW).is_some() {
            return Err("TRAE 官方窗口仍在关闭，请稍后刷新".into());
        }
    }
    let handle = app.clone();
    let builder = WebviewWindowBuilder::new(
        app,
        WINDOW,
        WebviewUrl::External(DASHBOARD.parse().unwrap()),
    )
    .title("TRAE 官方账户 · TokenDaily 积分连接")
    .inner_size(1024., 760.)
    .visible(visible);
    #[cfg(target_os = "windows")]
    let builder = builder.data_directory(app.state::<Credits>().profile.clone());
    #[cfg(target_os = "macos")]
    let builder = builder.data_store_identifier(MAC_DATA_STORE_ID);
    let window = builder
        .initialization_script(script(nonce))
        .on_navigation(move |url| {
            if url.scheme() == "tokendaily-trae" {
                if url.host_str() == Some("quota") {
                    callback(&handle, url);
                }
                return false;
            }
            // Official login redirects are allowed. No remote IPC capability is granted.
            url.scheme() == "https" || url.as_str() == "about:blank"
        })
        .build()
        .map_err(|_| "无法打开 TRAE 官方登录窗口")?;
    if visible {
        let _ = window.set_focus();
    }
    Ok(())
}
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn open(_: &tauri::AppHandle, _: bool, _: &str, _: bool) -> Result<(), String> {
    Err("当前平台尚未支持 TRAE 官方授权".into())
}

fn begin(app: &tauri::AppHandle, visible: bool, manual: bool) -> Result<(), String> {
    if !platform_supported() {
        return Err("当前平台尚未支持 TRAE 官方授权".into());
    }
    let s = app.state::<Credits>();
    let mut i = s.inner.lock().unwrap();
    let now = crate::now_ms();
    if i.state.refreshing
        || !visible && (!i.enabled || !retry_due(now, i.last_attempt, manual, i.failed))
    {
        return Ok(());
    }
    if visible {
        crate::credit_cache::clear_last(&s.cache_path).map_err(|_| "上次积分清除失败，请重试")?;
    }
    let mut bytes = [0; 32];
    getrandom::fill(&mut bytes).map_err(|_| "TRAE 授权初始化失败")?;
    let fresh = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    let window_open = app.get_webview_window(WINDOW).is_some();
    let (nonce, reuse_window) =
        attempt_nonce(REUSE_WINDOW, window_open, i.window_nonce.as_deref(), fresh);
    // A rebuilt window injects this nonce; keep it so later attempts can reuse it.
    i.window_nonce = Some(nonce.clone());
    i.nonce = Some(nonce.clone());
    i.visible_login = visible;
    if visible {
        i.auth_failures = 0;
    }
    i.deadline = now + if visible { 600_000 } else { 60_000 };
    i.last_attempt = now;
    if visible {
        i.state.balance = None;
        i.state.identity_label = None;
        i.state.authorized = false;
    }
    i.state.refreshing = true;
    i.state.status = if visible { "authorizing" } else { "loading" }.into();
    i.state.message = if visible {
        "请在独立窗口登录 TRAE 正在使用的官方账户。"
    } else {
        "正在更新 TRAE 官方积分"
    }
    .into();
    emit(app, &i);
    drop(i);
    if let Err(e) = open(app, visible, &nonce, reuse_window) {
        failure(app, &e);
        return Err(e);
    }
    Ok(())
}
#[tauri::command]
pub(crate) async fn authorize_trae_credits(app: tauri::AppHandle) -> Result<(), String> {
    begin(&app, true, true)
}
#[tauri::command]
pub(crate) async fn refresh_trae_credits(app: tauri::AppHandle) {
    let _ = begin(&app, false, true);
}
#[tauri::command]
pub(crate) fn cancel_trae_authorization(app: tauri::AppHandle) {
    failure(&app, "已取消 TRAE 登录，可重新打开官方窗口。");
    if let Some(w) = app.get_webview_window(WINDOW) {
        let _ = w.close();
    }
}
#[tauri::command]
pub(crate) async fn disconnect_trae_credits(app: tauri::AppHandle) -> Result<(), String> {
    clear(&app)
}
pub(crate) fn clear(app: &tauri::AppHandle) -> Result<(), String> {
    let s = app.state::<Credits>();
    let mut i = s.inner.lock().unwrap();
    crate::credit_cache::clear_last(&s.cache_path).map_err(|_| "上次积分清除失败，请重试")?;
    fs::write(&s.path, b"false").map_err(|_| "TRAE 授权设置清除失败")?;
    i.enabled = false;
    i.nonce = None;
    i.auth_failures = 0;
    i.visible_login = false;
    i.window_nonce = None;
    i.state = CreditState::default();
    emit(app, &i);
    drop(i);
    let mut window = app.get_webview_window(WINDOW);
    #[cfg(target_os = "windows")]
    if window.is_none() && s.profile.exists() {
        window = Some(
            WebviewWindowBuilder::new(
                app,
                WINDOW,
                WebviewUrl::External("about:blank".parse().unwrap()),
            )
            .visible(false)
            .data_directory(s.profile.clone())
            .build()
            .map_err(|_| "无法清除 TRAE 独立网页数据")?,
        );
    }
    #[cfg(target_os = "macos")]
    if window.is_none() && mac_store_supported() {
        window = Some(
            WebviewWindowBuilder::new(
                app,
                WINDOW,
                WebviewUrl::External("about:blank".parse().unwrap()),
            )
            .visible(false)
            .data_store_identifier(MAC_DATA_STORE_ID)
            .build()
            .map_err(|_| "无法清除 TRAE 独立网页数据")?,
        );
    }
    if let Some(w) = window {
        w.clear_all_browsing_data()
            .map_err(|_| "TRAE 网页登录数据清除失败")?;
        let _ = w.close();
    }
    Ok(())
}
pub(crate) fn tooltip(app: &tauri::AppHandle) -> String {
    let s = app.state::<Credits>();
    let i = s.inner.lock().unwrap();
    i.state
        .balance
        .as_ref()
        .map(|b| {
            format!(
                "{}{} 积分",
                if i.state.status != "available" {
                    "上次 "
                } else {
                    ""
                },
                crate::credit_display::amount(&b.remaining)
            )
        })
        .unwrap_or_else(|| "积分待授权".into())
}
pub(crate) fn start(app: tauri::AppHandle) {
    thread::spawn(move || loop {
        let (due, timed_out) = {
            let s = app.state::<Credits>();
            let i = s.inner.lock().unwrap();
            let now = crate::now_ms();
            (
                i.enabled
                    && !i.state.refreshing
                    && i.state.status != "auth-required"
                    && retry_due(now, i.last_attempt, false, i.failed),
                i.state.refreshing && now >= i.deadline,
            )
        };
        if timed_out {
            failure(&app, "TRAE 官方页面未返回积分，请重新登录或稍后刷新。");
        }
        if due {
            let _ = begin(&app, false, false);
        }
        thread::sleep(Duration::from_secs(2));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn background_refresh_reuses_the_open_window_and_its_nonce() {
        let fresh = || "fresh".to_string();
        assert_eq!(
            attempt_nonce(true, true, Some("kept"), fresh()),
            ("kept".into(), true)
        );
        assert_eq!(
            attempt_nonce(true, false, Some("kept"), fresh()),
            ("fresh".into(), false)
        );
        assert_eq!(
            attempt_nonce(true, true, None, fresh()),
            ("fresh".into(), false)
        );
        assert_eq!(
            attempt_nonce(false, true, Some("kept"), fresh()),
            ("fresh".into(), false)
        );
    }

    #[test]
    fn reuse_reloads_the_dashboard_instead_of_a_fragment_only_navigation() {
        let url = |s: &str| reqwest::Url::parse(s).unwrap();
        assert_eq!(
            reload_action(Some(&url("https://www.trae.cn/dashboard#usage"))),
            Reload::Page
        );
        assert_eq!(
            reload_action(Some(&url("https://www.trae.cn/login?x=1"))),
            Reload::Dashboard
        );
        assert_eq!(
            reload_action(Some(&url("https://evil.example/dashboard"))),
            Reload::Dashboard
        );
        assert_eq!(reload_action(None), Reload::Dashboard);
    }

    #[test]
    fn transient_auth_reply_retries_then_confirmed_loss_requires_login() {
        let mut i = Inner {
            state: CreditState::default(),
            enabled: true,
            nonce: Some("test".into()),
            deadline: 100_000,
            last_attempt: 1,
            failed: false,
            auth_failures: 0,
            visible_login: false,
            window_nonce: None,
        };
        i.state.authorized = true;
        i.state.balance = Some(parse_balance(
            &json!({"is_credits_billing":true,"user_entitlement_pack_list":[pack(json!(10),json!(1))]}),
            1,
        ).unwrap());
        i.visible_login = true;
        assert!(!auth_failure(&mut i));
        assert_eq!(i.auth_failures, 0);
        assert_eq!(i.nonce.as_deref(), Some("test"));
        assert!(i.state.message.contains("完成登录"));
        i.visible_login = false;
        assert!(!auth_failure(&mut i));
        assert_eq!(i.state.status, "stale");
        assert!(i.state.authorized);
        assert!(i.state.balance.is_some());
        assert!(retry_due(60_001, 1, false, true));
        assert!(!retry_due(59_999, 1, false, true));
        assert!(retry_due(10_001, 1, true, true));
        assert!(auth_failure(&mut i));
        assert_eq!(i.state.status, "auth-required");
        assert!(i.state.balance.is_none());
        assert!(!i.state.authorized);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn last_success_requires_enabled_profile_and_restores_as_stale() {
        let root = std::env::temp_dir().join(format!("tokendaily-trae-last-{}", crate::now_ms()));
        fs::create_dir_all(root.join("trae-official-profile")).unwrap();
        fs::write(root.join("trae-credits.json"), b"true").unwrap();
        let balance = parse_balance(&json!({"is_credits_billing":true,"user_entitlement_pack_list":[pack(json!(10),json!(1))]}), crate::now_ms()).unwrap();
        crate::credit_cache::save_last(
            &root.join("trae-last-balance.protected"),
            TRAE_CACHE_SCOPE,
            &balance,
        )
        .unwrap();
        let restored = Credits::new(&root);
        assert_eq!(restored.inner.lock().unwrap().state.status, "stale");
        assert_eq!(
            restored
                .inner
                .lock()
                .unwrap()
                .state
                .balance
                .as_ref()
                .unwrap()
                .remaining,
            "9"
        );
        fs::remove_dir_all(root.join("trae-official-profile")).unwrap();
        assert!(Credits::new(&root)
            .inner
            .lock()
            .unwrap()
            .state
            .balance
            .is_none());
        fs::create_dir_all(root.join("trae-official-profile")).unwrap();
        fs::write(root.join("trae-credits.json"), b"false").unwrap();
        assert!(Credits::new(&root)
            .inner
            .lock()
            .unwrap()
            .state
            .balance
            .is_none());
        fs::write(root.join("trae-credits.json"), b"true").unwrap();
        let mut future = balance;
        future.collected_at_ms = crate::now_ms() + 120_000;
        crate::credit_cache::save_last(
            &root.join("trae-last-balance.protected"),
            TRAE_CACHE_SCOPE,
            &future,
        )
        .unwrap();
        assert!(Credits::new(&root)
            .inner
            .lock()
            .unwrap()
            .state
            .balance
            .is_none());
        fs::remove_dir_all(root).unwrap();
    }
    fn pack(limit: Value, used: Value) -> Value {
        json!({"entitlement_base_info":{"entitlement_id":format!("test-{limit}-{used}"),"quota":{"credits_limit":limit},"end_time":2000000000},"usage":{"credits_amount":used}})
    }
    #[test]
    fn exact_packages_not_rounded_summary_and_expired_packs_are_excluded() {
        let mut expired = pack(json!(100), json!(0));
        expired["entitlement_base_info"]["end_time"] = json!(1);
        let v = json!({"is_credits_billing":true,"usage_summary":{"consumed_amount":2.44},"user_entitlement_pack_list":[pack(json!(4000),json!(0)),pack(json!(500),json!(2.4428)),pack(json!(100),json!(0)),expired]});
        let b = parse_balance(&v, 1792380000000).unwrap();
        assert_eq!(b.remaining, "4597.5572");
        assert_eq!(b.packages.len(), 3);
    }
    #[test]
    fn invalid_missing_and_other_billing_never_become_zero() {
        for p in [
            pack(json!(-1), json!(0)),
            pack(json!(100), json!(-1)),
            json!({"entitlement_base_info":{"quota":{"credits_limit":100}}}),
        ] {
            assert!(parse_balance(
                &json!({"is_credits_billing":true,"user_entitlement_pack_list":[p]}),
                1
            )
            .is_err());
        }
        assert!(parse_balance(&json!({"is_credits_billing":false}), 1).is_err());
        assert!(parse_balance(
            &json!({"is_credits_billing":true,"user_entitlement_pack_list":[]}),
            1
        )
        .is_err());
    }
    #[test]
    fn bridge_never_reads_authorization_or_cookies() {
        let js = script("nonce");
        for secret in [
            "document.cookie",
            "localStorage",
            "sessionStorage",
            "Authorization",
            "getRequestHeader",
        ] {
            assert!(!js.contains(secret));
        }
        assert!(js.contains("window.top!==window"));
        assert_eq!(percent_decode("%E4%BD%A0"), Some("你".into()));
        assert!(percent_decode("%GG").is_none());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn mac_credits_restore_last_balance_only_when_enabled() {
        let supported = mac_store_supported();
        let dir = std::env::temp_dir().join(format!("tokendaily-trae-credit-{}", crate::now_ms()));
        std::fs::create_dir_all(&dir).unwrap();
        let initial = Credits::new(&dir);
        assert_eq!(initial.inner.lock().unwrap().state.supported, supported);
        let balance = parse_balance(&json!({"is_credits_billing":true,"user_entitlement_pack_list":[pack(json!(10),json!(1))]}), crate::now_ms()).unwrap();
        crate::credit_cache::save_last(
            &dir.join("trae-last-balance.json"),
            TRAE_CACHE_SCOPE,
            &balance,
        )
        .unwrap();
        std::fs::write(dir.join("trae-credits.json"), b"true").unwrap();
        let restored = Credits::new(&dir);
        let inner = restored.inner.lock().unwrap();
        assert_eq!(inner.enabled, supported);
        assert_eq!(
            inner.state.status,
            if supported { "stale" } else { "unsupported" }
        );
        assert_eq!(
            inner.state.balance.as_ref().map(|b| b.remaining.as_str()),
            supported.then_some("9")
        );
        drop(inner);
        std::fs::remove_dir_all(dir).unwrap();
    }
}

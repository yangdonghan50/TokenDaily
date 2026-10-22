//! Read-only WorkBuddy credits using a separate official web authorization.
//! No client credentials, cookies, inference or account mutations.
use reqwest::{blocking::Client, redirect::Policy, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, io::Read, path::PathBuf, sync::Mutex, thread, time::Duration};
use tauri::{Emitter, Manager, State};

const ORIGIN: &str = "https://www.codebuddy.cn";
const SCALE: i64 = 1_000_000;
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Package {
    name: String,
    remaining: String,
    expires_at_ms: Option<i64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Balance {
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
        let supported = cfg!(any(target_os = "windows", target_os = "macos"));
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
                "请在设置中通过官方网页登录，授权读取积分余额。"
            } else {
                "当前平台的积分授权与安全存储尚未实现。"
            }
            .into(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
struct Credential {
    token: String,
    domain: String,
    identity: String,
    expires_at_ms: Option<i64>,
}
#[derive(Clone)]
struct Ticket {
    state: String,
    expires: i64,
}
struct Inner {
    state: CreditState,
    credential: Option<Credential>,
    ticket: Option<Ticket>,
    restore_attempted: bool,
    restoring: bool,
    generation: u64,
    last_attempt: i64,
    failed: bool,
    auth_failures: u8,
}
pub(crate) struct Credits {
    inner: Mutex<Inner>,
    path: PathBuf,
    cache_path: PathBuf,
}
fn cache_scope(c: &Credential) -> String {
    format!(
        "{:x}",
        Sha256::digest(format!("{}:{}", c.domain, c.token).as_bytes())
    )
}
fn valid_cached_balance(b: &Balance) -> bool {
    let now = crate::now_ms();
    b.collected_at_ms > 0
        && b.collected_at_ms <= now.saturating_add(60_000)
        && b.packages.len() <= 100
        && decimal(&Value::String(b.remaining.clone())).is_ok()
        && b.packages.iter().all(|p| {
            p.name.chars().count() <= 80
                && decimal(&Value::String(p.remaining.clone())).is_ok()
                && p.expires_at_ms.is_none_or(|t| t > 0)
        })
        && b.next_expiry_at_ms.is_none_or(|t| t > 0)
}
fn cached_balance(path: &std::path::Path, credential: &Credential) -> Option<Balance> {
    credential
        .expires_at_ms
        .is_none_or(|t| t > crate::now_ms())
        .then(|| crate::credit_cache::load_last(path, &cache_scope(credential)))
        .flatten()
        .filter(valid_cached_balance)
}
impl Credits {
    pub(crate) fn new(dir: &std::path::Path) -> Self {
        let path = dir.join("workbuddy-credits.protected");
        let cache_path = dir.join(if cfg!(target_os = "macos") {
            "workbuddy-last-balance.json"
        } else {
            "workbuddy-last-balance.protected"
        });
        #[cfg(target_os = "windows")]
        let (state, credential) = {
            let credential = crate::platform::workbuddy_credentials::load(&path)
                .ok()
                .flatten()
                .and_then(|v| serde_json::from_slice::<Credential>(&v).ok())
                .filter(|v| {
                    valid_domain(&v.domain)
                        && !v.token.is_empty()
                        && v.expires_at_ms.is_none_or(|t| t > crate::now_ms())
                });
            let mut state = CreditState::default();
            if let Some(c) = &credential {
                state.balance = cached_balance(&cache_path, c);
                state.authorized = true;
                state.status = if state.balance.is_some() {
                    "stale"
                } else {
                    "loading"
                }
                .into();
                state.identity_label = Some(c.identity.clone());
                state.message = if state.balance.is_some() {
                    "显示上次成功查询的积分，正在向官方更新"
                } else {
                    "正在查询官方积分余额"
                }
                .into();
            }
            (state, credential)
        };
        #[cfg(target_os = "macos")]
        let (state, credential) = (CreditState::default(), None);
        Self {
            path,
            cache_path,
            inner: Mutex::new(Inner {
                // Keychain access may need user interaction. Keep application setup free of
                // that synchronous system call; start() restores after the first window exists.
                state,
                credential,
                ticket: None,
                restore_attempted: false,
                restoring: false,
                generation: 0,
                last_attempt: 0,
                failed: false,
                auth_failures: 0,
            }),
        }
    }
}
fn restoring_state() -> CreditState {
    let mut state = CreditState::default();
    state.status = "loading".into();
    state.refreshing = true;
    state.message = "正在恢复本机已保存的积分授权".into();
    state
}
fn should_restore(retry: bool, attempted: bool, restoring: bool, authorizing: bool) -> bool {
    !restoring && !authorizing && (retry || !attempted)
}
fn restore_authorization(
    stored: Result<Option<Vec<u8>>, &'static str>,
) -> (CreditState, Option<Credential>) {
    let mut state = CreditState::default();
    let credential = stored.and_then(|bytes| match bytes {
        None => Ok(None),
        Some(bytes) => serde_json::from_slice::<Credential>(&bytes)
            .ok()
            .filter(|c| valid_domain(&c.domain) && !c.token.is_empty() && c.token.len() < 32768)
            .map(Some)
            .ok_or("本机积分授权内容无效，请在设置中重新登录官方账户"),
    });
    match credential {
        Ok(Some(c)) => {
            state.authorized = true;
            state.status = "loading".into();
            state.identity_label = Some(c.identity.clone());
            state.message = "正在查询官方积分余额".into();
            (state, Some(c))
        }
        Ok(None) => (state, None),
        Err(error) => {
            state.status = "error".into();
            state.message = error.into();
            (state, None)
        }
    }
}
fn decimal(v: &Value) -> Result<i64, &'static str> {
    let raw = match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return Err("积分字段缺失或无效"),
    };
    let parts: Vec<_> = raw.split('.').collect();
    if parts.len() > 2
        || parts[0].is_empty()
        || !parts
            .iter()
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
        || parts.get(1).is_some_and(|p| p.len() > 6)
    {
        return Err("积分精度或格式无法识别");
    }
    let whole = parts[0].parse::<i64>().map_err(|_| "积分数值超限")?;
    let frac = parts
        .get(1)
        .map(|s| format!("{s:0<6}").parse::<i64>().unwrap())
        .unwrap_or(0);
    whole
        .checked_mul(SCALE)
        .and_then(|n| n.checked_add(frac))
        .ok_or("积分数值超限")
}
fn display(n: i64) -> String {
    let s = format!("{}.{:06}", n / SCALE, n % SCALE);
    s.trim_end_matches('0').trim_end_matches('.').into()
}
fn timestamp(v: &Value) -> Result<Option<i64>, &'static str> {
    if v.is_null() {
        return Ok(None);
    }
    let n = v
        .as_i64()
        .or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok()));
    if let Some(n) = n {
        return if n <= 0 {
            Err("积分有效期无效")
        } else {
            Ok(Some(if n < 100_000_000_000 {
                n.checked_mul(1000).ok_or("有效期超限")?
            } else {
                n
            }))
        };
    }
    let s = v.as_str().ok_or("积分有效期格式未知")?;
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| Some(d.timestamp_millis()))
        .map_err(|_| "积分有效期缺少明确时区，暂不显示余额")
}
fn parse_balance(v: &Value, now: i64) -> Result<Balance, &'static str> {
    let container = [
        "/data/Response/Data",
        "/data/data/Response/Data",
        "/data/data",
        "/data",
        "/Response/Data",
    ]
    .iter()
    .filter_map(|p| v.pointer(p))
    .find(|v| v["Accounts"].is_array() || v["Packages"].is_array())
    .ok_or("积分接口数据格式未知")?;
    let rows = container["Accounts"]
        .as_array()
        .or_else(|| container["Packages"].as_array())
        .unwrap();
    if container
        .get("TotalCount")
        .and_then(|v| v.as_u64().or_else(|| v.as_str()?.parse().ok()))
        .is_some_and(|n| n as usize != rows.len())
        || rows.len() >= 100
    {
        return Err("积分包列表可能不完整，暂不汇总");
    }
    let mut seen: HashMap<String, Value> = HashMap::new();
    let mut total = 0i64;
    let mut packages = vec![];
    let mut expiry = None;
    for row in rows {
        if !row.is_object() {
            return Err("积分包格式未知");
        }
        if let Some(id) = row.get("ResourceId").or_else(|| row.get("PackageCode")) {
            let key = id.to_string();
            if let Some(old) = seen.get(&key) {
                if old == row {
                    continue;
                } else {
                    return Err("重复积分包数值冲突");
                }
            }
            seen.insert(key, row.clone());
        }
        let mut end = None;
        for key in ["DeductionEndTime", "ExpiredTime", "CycleEndTime"] {
            if let Some(t) = timestamp(&row[key])? {
                end = Some(end.map_or(t, |old: i64| old.min(t)));
            }
        }
        if end.is_some_and(|t| t <= now) {
            continue;
        }
        let keys = [
            "CycleCapacityRemainPrecise",
            "CycleCapacityRemain",
            "CycleRemainCapacity",
            "CapacityRemainPrecise",
            "CapacityRemain",
            "SlicePeriodCapacityRemainPrecise",
        ];
        // Never infer remaining from capacity/usage; multiple slices cannot be guessed.
        let slice = row["SlicePeriodUsageDetails"]
            .as_array()
            .filter(|v| v.len() == 1)
            .map(|v| &v[0]);
        let value = keys
            .iter()
            .find_map(|k| row.get(*k))
            .or_else(|| slice.and_then(|s| keys.iter().find_map(|k| s.get(*k))))
            .ok_or("积分包未返回剩余字段")?;
        let remaining = decimal(value)?;
        total = total.checked_add(remaining).ok_or("积分汇总超限")?;
        if remaining > 0 {
            if let Some(t) = end {
                expiry = Some(expiry.map_or(t, |old: i64| old.min(t)));
            }
        }
        packages.push(Package {
            name: row["PackageName"]
                .as_str()
                .map(|s| s.chars().take(80).collect())
                .unwrap_or_else(|| {
                    row["TotalCount"]
                        .as_u64()
                        .filter(|n| *n > 0)
                        .map(|n| format!("积分汇总（{n} 项）"))
                        .unwrap_or_else(|| "积分项目".into())
                }),
            remaining: display(remaining),
            expires_at_ms: end,
        });
    }
    Ok(Balance {
        remaining: display(total),
        packages,
        next_expiry_at_ms: expiry,
        collected_at_ms: now,
    })
}
fn valid_domain(s: &str) -> bool {
    matches!(
        s,
        "www.codebuddy.cn" | "codebuddy.cn" | "www.workbuddy.cn" | "workbuddy.cn"
    )
}
fn login_url(s: &str) -> Result<Url, &'static str> {
    let u = Url::parse(s).map_err(|_| "官方登录地址无效")?;
    if u.scheme() != "https"
        || !matches!(u.host_str(), Some("www.codebuddy.cn" | "www.workbuddy.cn"))
        || !u.username().is_empty()
        || u.password().is_some()
        || u.port().is_some()
    {
        return Err("授权地址不是受信任的官方网页");
    }
    Ok(u)
}
fn client() -> Result<Client, &'static str> {
    Client::builder()
        .redirect(Policy::none())
        .timeout(Duration::from_secs(15))
        .user_agent("WorkBuddy")
        .build()
        .map_err(|_| "无法初始化积分网络服务")
}
fn response(r: Result<reqwest::blocking::Response, reqwest::Error>) -> Result<Value, &'static str> {
    response_mode(r, false)
}
fn response_mode(
    r: Result<reqwest::blocking::Response, reqwest::Error>,
    login_poll: bool,
) -> Result<Value, &'static str> {
    let r = r.map_err(|_| "积分服务网络连接失败，请稍后重试")?;
    if matches!(r.status().as_u16(), 401 | 403) {
        return Err("授权失效，请重新登录");
    }
    if !r.status().is_success() {
        return Err("官方服务暂不可用，请稍后重试");
    }
    let mut bytes = vec![];
    r.take(1_048_577)
        .read_to_end(&mut bytes)
        .map_err(|_| "官方响应读取失败")?;
    if bytes.len() > 1_048_576 {
        return Err("官方响应过大，已停止读取");
    }
    let v: Value = serde_json::from_slice(&bytes).map_err(|_| "官方服务返回未知格式")?;
    if let Some(c) = v.get("code").or_else(|| v.pointer("/data/code")) {
        let code = c
            .as_i64()
            .or_else(|| c.as_str().and_then(|s| s.parse().ok()));
        if matches!(code, Some(401 | 403 | 12153)) {
            return Err("授权失效，请重新登录");
        }
        // Observed official state: authorization exists but the user has not logged in.
        if login_poll && code == Some(11217) {
            return Ok(json!({"data":null}));
        }
        if !matches!(code, Some(0 | 200)) {
            return Err("官方服务拒绝请求，请稍后重试");
        }
    }
    Ok(v)
}
fn fetch(c: &Credential) -> Result<Balance, &'static str> {
    if c.expires_at_ms.is_some_and(|t| t <= crate::now_ms()) {
        return Err("授权失效，请重新登录");
    }
    let origin = if matches!(c.domain.as_str(), "workbuddy.cn" | "www.workbuddy.cn") {
        "https://www.workbuddy.cn"
    } else {
        ORIGIN
    };
    let client = client()?;
    let request = |path: &str, body: Value| {
        client
            .post(format!("{origin}{path}"))
            .bearer_auth(&c.token)
            .header("X-Client-Platform", "web")
            .header("X-Domain", origin.trim_start_matches("https://"))
            .header("Origin", origin)
            .header("Referer", format!("{origin}/profile/plans-usage"))
            .json(&body)
            .send()
    };
    let result = request(
        "/billing/meter/get-user-resource-summary",
        json!({"ProductCode":"p_tcaca"}),
    );
    let result = if result.as_ref().is_ok_and(|r| r.status().as_u16() == 404) {
        request(
            "/v2/billing/meter/get-user-resource",
            json!({"ProductCode":"p_tcaca","PageNumber":1,"PageSize":100,"Status":[0,3]}),
        )
    } else {
        result
    };
    parse_balance(&response(result)?, crate::now_ms())
}
fn poll(t: &Ticket) -> Result<Option<Credential>, &'static str> {
    let client = client()?;
    let auth_url = |path: &str| {
        let mut url = Url::parse(&format!("{ORIGIN}{path}")).unwrap();
        url.query_pairs_mut().append_pair("state", &t.state);
        url
    };
    let v = response_mode(
        client
            .get(auth_url("/v2/plugin/auth/token"))
            .header("X-Client-Platform", "web")
            .send(),
        true,
    )?;
    let d = &v["data"];
    let token = d["accessToken"]
        .as_str()
        .or_else(|| d["access_token"].as_str());
    let Some(token) = token.filter(|s| !s.is_empty() && s.len() < 32768) else {
        return Ok(None);
    };
    let domain = d["domain"].as_str().unwrap_or("www.codebuddy.cn");
    if !valid_domain(domain) {
        return Err("官方授权服务域名不匹配");
    }
    let profile = response(
        client
            .get(auth_url("/v2/plugin/login/account"))
            .bearer_auth(token)
            .header("X-Domain", domain)
            .header("X-Client-Platform", "web")
            .send(),
    )?;
    let uid = profile["data"]["uid"]
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 200)
        .ok_or("官方授权未返回账户标识")?;
    let hash = format!("{:x}", Sha256::digest(uid.as_bytes()));
    let expiry = timestamp(&d["expiresAt"])?.or_else(|| {
        d["expiresIn"]
            .as_i64()
            .filter(|n| *n > 0 && *n < 31_536_000)
            .map(|n| crate::now_ms() + n * 1000)
    });
    Ok(Some(Credential {
        token: token.into(),
        domain: domain.into(),
        identity: format!("授权账户 · {}", &hash[..8]),
        expires_at_ms: expiry,
    }))
}
fn emit(app: &tauri::AppHandle, i: &Inner) {
    let _ = app.emit("workbuddy-credits-updated", &i.state);
}
fn accept(current: u64, started: u64) -> bool {
    current == started
}
fn due(now: i64, last: i64, force: bool, failed: bool) -> bool {
    last == 0
        || now.saturating_sub(last)
            >= if force {
                10_000
            } else if failed {
                60_000
            } else {
                300_000
            }
}
#[tauri::command]
pub(crate) fn get_workbuddy_credits(s: State<'_, Credits>) -> CreditState {
    s.inner.lock().unwrap().state.clone()
}

// Start only after WorkBuddy is enabled, and never make Tauri setup wait for Keychain.
// A manual refresh can retry a denied or locked Keychain, but the periodic worker must
// not repeatedly create permission prompts.
fn restore_saved_authorization(app: &tauri::AppHandle, retry: bool) {
    if !CreditState::default().supported
        || !crate::workbuddy::get_workbuddy_state(app.state()).connected
    {
        return;
    }
    let (epoch, path, cache_path) = {
        let s = app.state::<Credits>();
        let mut i = s.inner.lock().unwrap();
        if !should_restore(retry, i.restore_attempted, i.restoring, i.ticket.is_some()) {
            return;
        }
        i.restore_attempted = true;
        i.restoring = true;
        i.generation += 1;
        i.state = restoring_state();
        emit(app, &i);
        (i.generation, s.path.clone(), s.cache_path.clone())
    };
    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let (mut state, credential) =
            restore_authorization(crate::platform::workbuddy_credentials::load(&path));
        if let Some(c) = &credential {
            state.balance = cached_balance(&cache_path, c);
            if state.balance.is_some() {
                state.status = "stale".into();
                state.message = "显示上次成功查询的积分，正在向官方更新".into();
            }
        }
        let should_refresh = {
            let s = app.state::<Credits>();
            let mut i = s.inner.lock().unwrap();
            if !accept(i.generation, epoch)
                || !crate::workbuddy::get_workbuddy_state(app.state()).connected
            {
                return;
            }
            i.restoring = false;
            i.state = state;
            i.credential = credential;
            i.last_attempt = 0;
            i.failed = false;
            emit(&app, &i);
            i.credential.is_some()
        };
        crate::providers::update_selected_tray(&app);
        if should_refresh {
            refresh(&app, false);
        }
    });
}
#[tauri::command]
pub(crate) async fn authorize_workbuddy_credits(app: tauri::AppHandle) -> Result<(), String> {
    if !CreditState::default().supported {
        return Err("当前平台尚不支持积分授权".into());
    }
    if !crate::workbuddy::get_workbuddy_state(app.state()).connected {
        return Err("请先连接 WorkBuddy 本机监测".into());
    }
    let epoch = {
        let s = app.state::<Credits>();
        let mut i = s.inner.lock().unwrap();
        if i.ticket.is_some() {
            return Ok(());
        }
        i.generation += 1;
        i.restoring = false;
        i.state.refreshing = true;
        emit(&app, &i);
        i.generation
    };
    tauri::async_runtime::spawn_blocking(move || {
        let result = (|| {
            let v = response(
                client()?
                    .post(format!("{ORIGIN}/v2/plugin/auth/state?platform=workbuddy"))
                    .header("X-Client-Platform", "web")
                    .json(&json!({}))
                    .send(),
            )?;
            let d = &v["data"];
            let state = d["state"]
                .as_str()
                .filter(|s| !s.is_empty() && s.len() <= 500)
                .ok_or("官方授权请求无效")?;
            let fallback = format!("{ORIGIN}/login");
            let mut url = login_url(
                d["authUrl"]
                    .as_str()
                    .or_else(|| d["auth_url"].as_str())
                    .unwrap_or(&fallback),
            )?;
            if d["authUrl"].is_null() && d["auth_url"].is_null() {
                url.query_pairs_mut().append_pair("state", state);
            }
            Ok::<_, &str>((
                Ticket {
                    state: state.into(),
                    expires: crate::now_ms() + 300_000,
                },
                url,
            ))
        })();
        let s = app.state::<Credits>();
        let mut i = s.inner.lock().unwrap();
        if !accept(i.generation, epoch) {
            return Ok(());
        }
        i.state.refreshing = false;
        match result {
            Ok((ticket, url)) => {
                // Serialize the browser launch with cancellation; no cancelled request opens a tab.
                match crate::platform::open_official_url(url.as_str()) {
                    Ok(()) => {
                        i.ticket = Some(ticket);
                        i.state.status = "authorizing".into();
                        i.state.message =
                            "请在已打开的官方网页完成登录；五分钟内自动读取授权结果。".into();
                        emit(&app, &i);
                        Ok(())
                    }
                    Err(e) => {
                        i.state.message = e.into();
                        emit(&app, &i);
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                i.state.message = e.into();
                emit(&app, &i);
                Err(e.to_string())
            }
        }
    })
    .await
    .map_err(|_| "授权任务意外结束".to_string())?
}
#[tauri::command]
pub(crate) fn cancel_workbuddy_authorization(app: tauri::AppHandle) {
    let s = app.state::<Credits>();
    let mut i = s.inner.lock().unwrap();
    i.generation += 1;
    i.ticket = None;
    i.state.refreshing = false;
    i.state.status = if i.credential.is_some() {
        if i.state.balance.is_some() {
            "stale"
        } else {
            "loading"
        }
    } else {
        "auth-required"
    }
    .into();
    i.state.message = "已取消本次授权".into();
    emit(&app, &i);
}
pub(crate) fn clear(app: &tauri::AppHandle) -> Result<(), String> {
    let s = app.state::<Credits>();
    let mut i = s.inner.lock().unwrap();
    // Serialize persistence and deletion with generation changes.
    crate::credit_cache::clear_last(&s.cache_path).map_err(|_| "上次积分清除失败，请重试")?;
    crate::platform::workbuddy_credentials::remove(&s.path)?;
    i.generation += 1;
    i.credential = None;
    i.ticket = None;
    // disconnect has removed our item, so reconnecting in this process must not
    // recreate a Keychain read just to rediscover the deliberate removal.
    i.restore_attempted = true;
    i.restoring = false;
    i.state = CreditState::default();
    i.last_attempt = 0;
    i.failed = false;
    i.auth_failures = 0;
    emit(app, &i);
    Ok(())
}
#[tauri::command]
pub(crate) fn disconnect_workbuddy_credits(app: tauri::AppHandle) -> Result<(), String> {
    clear(&app)
}
#[tauri::command]
pub(crate) async fn refresh_workbuddy_credits(app: tauri::AppHandle) {
    let needs_restore = {
        let s = app.state::<Credits>();
        let i = s.inner.lock().unwrap();
        i.credential.is_none() && i.ticket.is_none()
    };
    if needs_restore {
        restore_saved_authorization(&app, true);
        return;
    }
    let _ = tauri::async_runtime::spawn_blocking(move || refresh(&app, true)).await;
}
// A single unauthorized reply during startup can precede restoration of a web session.
// Require a second consecutive reply before discarding the last known balance.
fn fail(i: &mut Inner, e: &str, locally_expired: bool) -> bool {
    i.failed = true;
    i.state.refreshing = false;
    i.state.message = e.into();
    if e.starts_with("授权失效") {
        i.auth_failures = i.auth_failures.saturating_add(1);
        if locally_expired || i.auth_failures >= 2 {
            i.credential = None;
            i.state.authorized = false;
            i.state.balance = None;
            i.state.status = "auth-required".into();
            return true;
        }
        i.state.message = "官方返回授权异常，正在复核；可稍后手动刷新。".into();
    } else {
        i.auth_failures = 0;
    }
    i.state.status = if i.state.balance.is_some() {
        "stale"
    } else {
        "error"
    }
    .into();
    false
}
fn forget_invalid_credential(s: &Credits, i: &mut Inner) {
    let cache_cleared = crate::credit_cache::clear_last(&s.cache_path).is_ok();
    let credential_cleared = crate::platform::workbuddy_credentials::remove(&s.path).is_ok();
    if cache_cleared && credential_cleared {
        i.state.identity_label = None;
    } else {
        i.state.message = "官方授权已失效；本机旧授权清除失败，请在设置中重试清除。".into();
    }
}
fn refresh(app: &tauri::AppHandle, force: bool) {
    if !crate::workbuddy::get_workbuddy_state(app.state()).connected {
        return;
    }
    let s = app.state::<Credits>();
    let (epoch, credential) = {
        let mut i = s.inner.lock().unwrap();
        if i.state.refreshing
            || i.ticket.is_some()
            || !due(crate::now_ms(), i.last_attempt, force, i.failed)
        {
            return;
        }
        let Some(c) = i.credential.clone() else {
            return;
        };
        i.state.refreshing = true;
        i.last_attempt = crate::now_ms();
        emit(app, &i);
        (i.generation, c)
    };
    let result = fetch(&credential);
    {
        let mut i = s.inner.lock().unwrap();
        if !accept(i.generation, epoch) {
            return;
        }
        i.state.refreshing = false;
        match result {
            Ok(balance) => {
                let saved = crate::credit_cache::save_last(
                    &s.cache_path,
                    &cache_scope(&credential),
                    &balance,
                );
                i.state.balance = Some(balance);
                i.state.status = "available".into();
                i.state.message = if saved.is_ok() {
                    "官方账户积分 · 独立于本机 Token".into()
                } else {
                    "官方积分已读取，但上次结果保存失败".into()
                };
                i.failed = false;
                i.auth_failures = 0;
            }
            Err(e) => {
                if fail(
                    &mut i,
                    e,
                    credential
                        .expires_at_ms
                        .is_some_and(|t| t <= crate::now_ms()),
                ) {
                    forget_invalid_credential(&s, &mut i);
                }
            }
        }
        emit(app, &i);
    }
    crate::providers::update_selected_tray(app);
}
fn login_tick(app: &tauri::AppHandle) {
    let s = app.state::<Credits>();
    let (epoch, ticket) = {
        let mut i = s.inner.lock().unwrap();
        let Some(t) = i.ticket.clone() else {
            return;
        };
        if t.expires <= crate::now_ms() {
            i.ticket = None;
            fail(&mut i, "授权已超时，请重新开始登录", false);
            emit(app, &i);
            return;
        }
        (i.generation, t)
    };
    let result = poll(&ticket);
    let mut i = s.inner.lock().unwrap();
    if !accept(i.generation, epoch) {
        return;
    }
    match result {
        Ok(None) => {}
        Ok(Some(c)) => {
            let saved = crate::credit_cache::clear_last(&s.cache_path)
                .map_err(|_| "上次积分清除失败")
                .and_then(|_| serde_json::to_vec(&c).map_err(|_| "授权序列化失败"))
                .and_then(|b| crate::platform::workbuddy_credentials::save(&s.path, &b));
            i.ticket = None;
            match saved {
                Ok(()) => {
                    i.state.identity_label = Some(c.identity.clone());
                    i.state.balance = None;
                    i.credential = Some(c);
                    i.state.authorized = true;
                    i.state.status = "loading".into();
                    i.state.message = "官方登录完成，正在查询积分".into();
                    i.last_attempt = 0;
                    i.failed = false;
                    i.auth_failures = 0;
                }
                Err(e) => {
                    fail(&mut i, e, false);
                }
            }
            emit(app, &i);
        }
        Err(e) => {
            i.state.message = format!("{e}；授权窗口有效期内继续等待");
            emit(app, &i);
        }
    }
}
pub(crate) fn start(app: tauri::AppHandle) {
    thread::spawn(move || loop {
        restore_saved_authorization(&app, false);
        login_tick(&app);
        refresh(&app, false);
        thread::sleep(Duration::from_secs(3));
    });
}
pub(crate) fn tooltip(app: &tauri::AppHandle) -> String {
    let s = app.state::<Credits>();
    let i = s.inner.lock().unwrap();
    if let Some(b) = &i.state.balance {
        format!(
            "{}{} 积分",
            if i.state.status != "available"
                || crate::now_ms() - b.collected_at_ms > 600_000
                || b.next_expiry_at_ms.is_some_and(|t| t <= crate::now_ms())
            {
                "上次余额 "
            } else {
                ""
            },
            crate::credit_display::amount(&b.remaining)
        )
    } else if i.state.authorized {
        "积分待查询".into()
    } else {
        "积分待授权".into()
    }
}
#[cfg(test)]
#[path = "workbuddy_credits_tests.rs"]
mod tests;

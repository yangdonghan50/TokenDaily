use crate::usage::{self, UsageSnapshot, UsageState};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read, Write},
    process::{Child, Stdio},
    sync::{mpsc, Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QuotaWindow {
    pub(crate) pool_id: String,
    pub(crate) window_id: String,
    pub(crate) label: String,
    pub(crate) used_percent: Option<f64>,
    pub(crate) remaining_percent: Option<f64>,
    pub(crate) resets_at_ms: Option<i64>,
    pub(crate) duration_minutes: Option<i64>,
    pub(crate) reached_type: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QuotaSnapshot {
    pub(crate) identity_key: String,
    pub(crate) status: String,
    pub(crate) collected_at_ms: i64,
    pub(crate) windows: Vec<QuotaWindow>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DesktopState {
    pub status: String,
    pub snapshot: Option<QuotaSnapshot>,
    pub identity_label: Option<String>,
    #[serde(default)]
    pub identity_verified: bool,
    pub last_success_at_ms: Option<i64>,
    pub message: Option<String>,
    #[serde(default)]
    pub connected: bool,
    #[serde(default)]
    pub usage: UsageState,
}

impl DesktopState {
    pub(crate) fn disconnected() -> Self {
        Self {
            status: "disconnected".into(),
            snapshot: None,
            identity_label: None,
            identity_verified: false,
            last_success_at_ms: None,
            message: None,
            connected: false,
            usage: UsageState::default(),
        }
    }
    pub(crate) fn with_error(previous: &Self, message: &'static str) -> Self {
        let mut next = previous.clone();
        next.status = if next.snapshot.is_some() {
            "stale"
        } else {
            "error"
        }
        .into();
        next.message = Some(message.into());
        next.connected = true;
        next
    }
    pub(crate) fn unsupported() -> Self {
        Self {
            status: "unsupported".into(),
            snapshot: None,
            identity_label: None,
            identity_verified: false,
            last_success_at_ms: None,
            message: Some("当前 Codex 登录类型不支持额度读取".into()),
            connected: true,
            usage: UsageState::default(),
        }
    }
    pub(crate) fn auth_required() -> Self {
        Self {
            status: "auth-required".into(),
            snapshot: None,
            identity_label: None,
            identity_verified: false,
            last_success_at_ms: None,
            message: Some("请先在 Codex 中登录账户".into()),
            connected: true,
            usage: UsageState::default(),
        }
    }
}

fn send(stdin: &mut impl Write, value: Value) -> Result<(), &'static str> {
    writeln!(stdin, "{value}").map_err(|_| "无法向 Codex 发送只读请求")
}

fn receive(reader: &mut BufReader<impl Read>, id: i64) -> Result<Value, &'static str> {
    for _ in 0..200 {
        let mut line = String::new();
        if reader
            .read_line(&mut line)
            .map_err(|_| "无法读取 Codex 响应")?
            == 0
        {
            return Err("Codex 连接提前结束");
        }
        let Ok(value) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        if value.get("id").and_then(Value::as_i64) != Some(id) {
            continue;
        }
        if value.get("error").is_some() {
            return Err("Codex 拒绝了只读状态请求");
        }
        return value.get("result").cloned().ok_or("Codex 响应缺少结果");
    }
    Err("Codex 响应包含过多无关消息")
}

fn query(
    mut stdin: impl Write,
    stdout: impl Read,
    first_sender: mpsc::Sender<Result<(Value, Result<Value, &'static str>), &'static str>>,
    usage_sender: mpsc::Sender<Result<Value, &'static str>>,
) {
    let mut reader = BufReader::new(stdout);
    let first = (|| -> Result<_, &'static str> {
        send(
            &mut stdin,
            json!({"method":"initialize","id":1,"params":{"clientInfo":{
            "name":"tokendaily_local","title":"TokenDaily","version":"0.1.0"}}}),
        )?;
        receive(&mut reader, 1)?;
        send(&mut stdin, json!({"method":"initialized"}))?;
        send(
            &mut stdin,
            json!({"method":"account/read","id":2,"params":{"refreshToken":false}}),
        )?;
        let account = receive(&mut reader, 2)?;
        send(
            &mut stdin,
            json!({"method":"account/rateLimits/read","id":3}),
        )?;
        let limits = receive(&mut reader, 3);
        send(
            &mut stdin,
            json!({"method":"account/read","id":5,"params":{"refreshToken":false}}),
        )?;
        let checked = receive(&mut reader, 5)?;
        if !same_account(&account, &checked) {
            return Err(IDENTITY_CHANGED);
        }
        Ok((account, limits))
    })();
    let initial_account = first.as_ref().ok().map(|(account, _)| account.clone());
    let ready = first.is_ok();
    let _ = first_sender.send(first);
    if !ready {
        return;
    }
    let usage = send(&mut stdin, json!({"method":"account/usage/read","id":4}))
        .and_then(|_| receive(&mut reader, 4));
    let checked = send(
        &mut stdin,
        json!({"method":"account/read","id":6,"params":{"refreshToken":false}}),
    )
    .and_then(|_| receive(&mut reader, 6));
    let usage = match checked {
        Ok(account)
            if initial_account
                .as_ref()
                .is_some_and(|initial| same_account(initial, &account)) =>
        {
            usage
        }
        Ok(_) => Err(IDENTITY_CHANGED),
        Err(_) => Err("账号日统计读取后无法核对身份"),
    };
    let _ = usage_sender.send(usage);
}

fn stop_child(child: &Arc<Mutex<Child>>) {
    if let Ok(mut child) = child.lock() {
        let _ = child.kill(); // The process TokenDaily created, never a user Codex process.
        let _ = child.wait();
    }
}

fn query_with_timeout(
    source: &crate::connection::Source,
) -> Result<
    (
        Value,
        Result<Value, &'static str>,
        Result<Value, &'static str>,
    ),
    &'static str,
> {
    let mut command = source.command();
    command
        .args(["app-server", "--listen", "stdio://"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    let mut child = command.spawn().map_err(|_| "无法启动本机 Codex CLI")?;
    let stdin = child.stdin.take().ok_or("Codex 输入管道不可用")?;
    let stdout = child.stdout.take().ok_or("Codex 输出管道不可用")?;
    let child = Arc::new(Mutex::new(child));
    let (first_sender, first_receiver) = mpsc::channel();
    let (usage_sender, usage_receiver) = mpsc::channel();
    std::thread::spawn(move || {
        query(stdin, stdout, first_sender, usage_sender);
    });
    let first = first_receiver
        .recv_timeout(Duration::from_secs(15))
        .unwrap_or(Err("Codex 状态请求超时"));
    let result = first.map(|(account, limits)| {
        let usage = usage_receiver
            .recv_timeout(Duration::from_secs(5))
            .unwrap_or(Err("日用量请求超时"));
        (account, limits, usage)
    });
    stop_child(&child);
    result
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

fn label(duration: Option<i64>, pool_name: Option<&str>, slot: &str) -> String {
    let period = duration.map(|minutes| {
        if minutes % 1440 == 0 {
            format!("{} 天额度", minutes / 1440)
        } else if minutes % 60 == 0 {
            format!("{} 小时额度", minutes / 60)
        } else {
            format!("{minutes} 分钟额度")
        }
    });
    match (pool_name.filter(|name| !name.is_empty()), period) {
        (Some(name), Some(period)) => format!("{name} · {period}"),
        (Some(name), None) => name.to_owned(),
        (None, Some(period)) => period,
        (None, None) => format!("{}额度窗口", if slot == "primary" { "主" } else { "次" }),
    }
}

fn valid_percent(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .filter(|percent| percent.is_finite() && (0.0..=100.0).contains(percent))
}

fn parse_windows(raw: &Value) -> Vec<QuotaWindow> {
    let mut pools: BTreeMap<String, &Value> = BTreeMap::new();
    if let Some(multi) = raw.get("rateLimitsByLimitId").and_then(Value::as_object) {
        for (id, bucket) in multi {
            if bucket.is_object() {
                pools.insert(id.clone(), bucket);
            }
        }
    }
    if let Some(single) = raw.get("rateLimits").filter(|value| value.is_object()) {
        let id = single
            .get("limitId")
            .and_then(Value::as_str)
            .filter(|id| !id.is_empty())
            .unwrap_or("default");
        pools.entry(id.to_owned()).or_insert(single);
    }
    let mut windows = Vec::new();
    for (pool_id, bucket) in pools {
        let name = bucket.get("limitName").and_then(Value::as_str);
        let reached_type = bucket
            .get("rateLimitReachedType")
            .and_then(Value::as_str)
            .map(str::to_owned);
        for slot in ["primary", "secondary"] {
            let Some(source) = bucket.get(slot).filter(|value| value.is_object()) else {
                continue;
            };
            let used_percent = source.get("usedPercent").and_then(valid_percent);
            let duration_minutes = source
                .get("windowDurationMins")
                .and_then(Value::as_i64)
                .filter(|minutes| *minutes > 0);
            let resets_at_ms = source
                .get("resetsAt")
                .and_then(Value::as_i64)
                .filter(|seconds| (0..100_000_000_000).contains(seconds))
                .and_then(|seconds| seconds.checked_mul(1000));
            windows.push(QuotaWindow {
                pool_id: pool_id.clone(),
                window_id: slot.to_owned(),
                label: label(duration_minutes, name, slot),
                used_percent,
                remaining_percent: used_percent.map(|used| 100.0 - used),
                resets_at_ms,
                duration_minutes,
                reached_type: reached_type.clone(),
            });
        }
    }
    windows
}

fn mask_email(email: &str) -> String {
    let Some((local, domain)) = email.split_once('@') else {
        return "已连接账户".into();
    };
    let first = local.chars().next().unwrap_or('*');
    format!("{first}***@{domain}")
}

pub(crate) const IDENTITY_CHANGED: &str = "Codex 身份在读取期间发生变化，请刷新重试";

struct Identity {
    key: String,
    label: String,
    verified: bool,
}
fn text_field(value: &Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.len() <= 4096)
        .map(str::to_owned)
}
fn identify(account_result: &Value, source: &str) -> Result<Identity, &'static str> {
    let account = account_result
        .get("account")
        .filter(|a| !a.is_null())
        .ok_or("需要连接 Codex 账户")?;
    if account.get("type").and_then(Value::as_str) != Some("chatgpt") {
        return Err("当前 Codex 登录类型不支持额度读取");
    }
    let email = text_field(account, "email").ok_or("Codex 未返回可核对的账户身份")?;
    let routing = &account_result["workspaceRouting"];
    let workspace = text_field(routing, "chatgptAccountId");
    let origin = text_field(routing, "backendOrigin");
    let verified = workspace.is_some() && origin.is_some();
    let bytes =
        serde_json::to_vec(&(source, &email, &workspace, &origin)).expect("identity fields");
    Ok(Identity {
        key: format!("codex:v2:{:x}", Sha256::digest(bytes)),
        label: mask_email(&email),
        verified,
    })
}
fn same_account(before: &Value, after: &Value) -> bool {
    // Null/unsupported accounts must still reach the normal auth diagnostics.
    if before.get("account").is_none_or(Value::is_null)
        && after.get("account").is_none_or(Value::is_null)
    {
        return true;
    }
    match (identify(before, ""), identify(after, "")) {
        (Ok(a), Ok(b)) => a.key == b.key && a.verified == b.verified,
        (Err(a), Err(b)) => a == b && before.get("account") == after.get("account"),
        _ => false,
    }
}

fn normalize(identity: &Identity, limits: &Value) -> Result<DesktopState, &'static str> {
    let identity_key = identity.key.clone();
    let identity_label = identity.label.clone();
    let windows = parse_windows(limits);
    let collected_at_ms = now_ms();
    let status = if windows
        .iter()
        .any(|window| window.remaining_percent.is_some())
    {
        "available"
    } else {
        "unknown"
    };
    Ok(DesktopState {
        status: status.into(),
        snapshot: Some(QuotaSnapshot {
            identity_key,
            status: status.into(),
            collected_at_ms,
            windows,
        }),
        identity_label: Some(identity_label),
        identity_verified: identity.verified,
        last_success_at_ms: Some(collected_at_ms),
        message: None,
        connected: true,
        usage: UsageState::default(),
    })
}

pub(crate) struct ReadOutcome {
    pub(crate) identity_key: String,
    pub(crate) identity_label: String,
    pub(crate) identity_verified: bool,
    pub(crate) quota: Result<DesktopState, &'static str>,
    pub(crate) usage: Result<UsageSnapshot, &'static str>,
}

pub(crate) fn read_once(source: &crate::connection::Source) -> Result<ReadOutcome, &'static str> {
    let (account, limits, usage) = query_with_timeout(source)?;
    if matches!(usage, Err(IDENTITY_CHANGED)) {
        return Err(IDENTITY_CHANGED);
    }
    let identity = identify(&account, &source.key())?;
    let identity_key = identity.key.clone();
    let identity_label = identity.label.clone();
    let quota = limits.and_then(|limits| normalize(&identity, &limits));
    let usage = usage.and_then(|raw| usage::normalize(&raw, identity_key.clone(), now_ms()));
    Ok(ReadOutcome {
        identity_key,
        identity_label,
        identity_verified: identity.verified,
        quota,
        usage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(workspace: Option<&str>) -> Value {
        json!({"account":{"type":"chatgpt","email":"same@example.test","planType":"team"},
            "workspaceRouting":workspace.map(|id|json!({"chatgptAccountId":id,"backendOrigin":"https://example.test","accountRoutingOverride":"NO_CONSTRAINT"}))})
    }

    #[test]
    fn protocol_rechecks_identity_around_quota_and_usage_reads() {
        for change_after_quota in [true, false] {
            let a = account(Some("workspace-a"));
            let b = account(Some("workspace-b"));
            let mut responses = vec![
                json!({"id":1,"result":{}}),
                json!({"id":2,"result":a}),
                json!({"id":3,"result":{"rateLimits":{}}}),
                json!({"id":5,"result":if change_after_quota {&b} else {&a}}),
            ];
            if !change_after_quota {
                responses.extend([
                    json!({"id":4,"result":{"dailyUsageBuckets":[]}}),
                    json!({"id":6,"result":b}),
                ]);
            }
            let bytes = responses
                .iter()
                .map(|v| format!("{v}\n"))
                .collect::<String>();
            let (first_tx, first_rx) = mpsc::channel();
            let (usage_tx, usage_rx) = mpsc::channel();
            let mut sent = Vec::new();
            query(&mut sent, std::io::Cursor::new(bytes), first_tx, usage_tx);
            if change_after_quota {
                assert!(matches!(first_rx.recv().unwrap(), Err(IDENTITY_CHANGED)));
            } else {
                assert!(first_rx.recv().unwrap().is_ok());
                assert!(matches!(usage_rx.recv().unwrap(), Err(IDENTITY_CHANGED)));
            }
            let requests = String::from_utf8(sent).unwrap();
            assert!(requests.contains("\"refreshToken\":false"));
            assert!(!requests.contains("turn/start"));
        }
    }

    #[test]
    fn same_email_workspace_switch_and_source_are_separate() {
        let a = identify(&account(Some("workspace-a")), "source-a").unwrap();
        let b = identify(&account(Some("workspace-b")), "source-a").unwrap();
        let c = identify(&account(Some("workspace-a")), "source-b").unwrap();
        assert_ne!(a.key, b.key);
        assert_ne!(a.key, c.key);
        assert!(a.verified);
        assert!(!a.key.contains("workspace-a") && !a.key.contains("example.test"));
        assert!(same_account(
            &account(Some("workspace-a")),
            &account(Some("workspace-a"))
        ));
        assert!(!same_account(
            &account(Some("workspace-a")),
            &account(Some("workspace-b"))
        ));
    }
    #[test]
    fn missing_or_malformed_workspace_never_claims_verified() {
        assert!(!identify(&account(None), "source").unwrap().verified);
        let mut malformed = account(Some("workspace-a"));
        malformed["workspaceRouting"]["chatgptAccountId"] = json!(" ");
        assert!(!identify(&malformed, "source").unwrap().verified);
        assert!(!same_account(&account(Some("workspace-a")), &account(None)));
        assert!(identify(&json!({"account":null}), "source").is_err());
    }

    #[test]
    fn converts_and_deduplicates_official_bucket_shapes() {
        let bucket = json!({"limitId":"codex","primary":{"usedPercent":73,"windowDurationMins":60,"resetsAt":1792051200}});
        let raw = json!({"rateLimits":bucket,"rateLimitsByLimitId":{"codex":bucket}});
        let windows = parse_windows(&raw);
        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].remaining_percent, Some(27.0));
        assert_eq!(windows[0].resets_at_ms, Some(1792051200000));
    }

    #[test]
    fn invalid_percent_does_not_become_full_or_empty() {
        let raw = json!({"rateLimits":{"limitId":"codex","primary":{"usedPercent":null}}});
        assert_eq!(parse_windows(&raw)[0].remaining_percent, None);
    }

    #[test]
    fn failed_first_refresh_remains_connected_for_retry_or_disconnect() {
        let failed = DesktopState::with_error(&DesktopState::disconnected(), "读取失败");
        assert_eq!(failed.status, "error");
        assert!(failed.connected);
        assert!(failed.snapshot.is_none());
    }

    #[test]
    #[ignore = "requires an existing local Codex ChatGPT login; run explicitly during M0/M2"]
    fn live_read_only_contract() {
        let source = crate::connection::Settings::default()
            .resolve()
            .expect("local source");
        let outcome = read_once(&source).expect("read-only Codex status request");
        let state = outcome.quota.expect("quota result");
        assert_eq!(state.status, "available");
        assert!(
            state.identity_verified,
            "live workspace routing must be verified on this CLI"
        );
        assert!(state
            .identity_label
            .as_ref()
            .is_some_and(|label| label.contains("***@")));
        assert!(state
            .snapshot
            .as_ref()
            .is_some_and(|snapshot| !snapshot.windows.is_empty()));
        assert!(outcome.usage.expect("daily usage result").days.len() > 0);
    }
}

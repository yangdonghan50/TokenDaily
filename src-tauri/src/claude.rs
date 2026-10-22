//! Read-only subscription usage. This internal OAuth endpoint can change independently of the app.
use crate::codex::{DesktopState, QuotaSnapshot, QuotaWindow};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{io::Read, time::Duration};

pub(crate) struct Failure {
    pub(crate) message: &'static str,
    pub(crate) auth: bool,
}
impl From<&'static str> for Failure {
    fn from(message: &'static str) -> Self {
        Self {
            message,
            auth: false,
        }
    }
}
fn auth(message: &'static str) -> Failure {
    Failure {
        message,
        auth: true,
    }
}
fn parse_credentials(raw: &str, now: i64) -> Result<String, &'static str> {
    let data: Value = serde_json::from_str(raw)
        .map_err(|_| "Claude 登录凭据格式异常，请在 Claude Code 中重新登录")?;
    let oauth = &data["claudeAiOauth"];
    if !oauth["scopes"]
        .as_array()
        .is_some_and(|a| a.iter().any(|s| s.as_str() == Some("user:profile")))
    {
        return Err("Claude 凭据无额度读取权限，请使用 Claude Code 的订阅账户登录");
    }
    if !oauth["expiresAt"]
        .as_i64()
        .is_some_and(|expiry| expiry > now)
    {
        return Err("Claude 登录凭据已过期，请打开 Claude Code 更新登录后刷新");
    }
    oauth["accessToken"]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .ok_or("Claude 订阅登录凭据缺失")
}
pub(crate) fn normalize_quota(
    raw: &Value,
    identity: &str,
    now: i64,
) -> Result<QuotaSnapshot, &'static str> {
    let mut windows = Vec::new();
    // Only materialize windows actually returned; never infer a plan's limits.
    for (key, label, minutes) in [
        ("five_hour", "5 小时额度", 300),
        ("seven_day", "7 天额度", 10080),
        ("seven_day_sonnet", "Sonnet · 7 天", 10080),
        ("seven_day_opus", "Opus · 7 天", 10080),
    ] {
        let Some(value) = raw.get(key).filter(|v| v.is_object()) else {
            continue;
        };
        let used = value["utilization"]
            .as_f64()
            .filter(|v| v.is_finite() && (0.0..=100.0).contains(v));
        let resets = value["resets_at"]
            .as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|t| t.timestamp_millis());
        windows.push(QuotaWindow {
            pool_id: format!("claude:{key}"),
            window_id: "primary".into(),
            label: label.into(),
            used_percent: used,
            remaining_percent: used.map(|v| 100.0 - v),
            resets_at_ms: resets,
            duration_minutes: Some(minutes),
            reached_type: used.filter(|v| *v >= 100.0).map(|_| "limit-reached".into()),
        });
    }
    if windows.is_empty() {
        return Err("Claude 来源未返回可识别的额度窗口");
    }
    Ok(QuotaSnapshot {
        identity_key: format!("claude:{:x}", Sha256::digest(identity.as_bytes())),
        status: if windows.iter().any(|w| w.remaining_percent.is_some()) {
            "available"
        } else {
            "unknown"
        }
        .into(),
        collected_at_ms: now,
        windows,
    })
}

// Claude Desktop writes this small, short-lived usage history itself. It stores
// utilization percentages, not chat Token counts, credentials or reset times.
pub(crate) fn parse_desktop_history(
    raw: &[u8],
    now: i64,
) -> Result<(DesktopState, String), &'static str> {
    let history: Value =
        serde_json::from_slice(raw).map_err(|_| "Claude Desktop 额度记录格式异常")?;
    if history["version"].as_u64() != Some(2) {
        return Err("Claude Desktop 额度记录版本尚不支持");
    }
    let samples = history["samples"]
        .as_array()
        .ok_or("Claude Desktop 额度记录缺少样本")?;
    // Desktop keeps weeks of samples; bound work without rejecting a normal history.
    if samples.len() > 16_000 {
        return Err("Claude Desktop 额度记录样本过多");
    }
    if samples.iter().any(|sample| sample["t"].as_i64().is_none()) {
        return Err("Claude Desktop 额度记录时间异常");
    }
    // Never fall back to an earlier account's populated sample when the latest
    // sample is empty or malformed.
    let sample = samples
        .iter()
        .max_by_key(|s| s["t"].as_i64().unwrap_or(i64::MIN))
        .ok_or("Claude Desktop 尚无额度使用记录")?;
    let collected = sample["t"]
        .as_i64()
        .ok_or("Claude Desktop 额度记录时间异常")?;
    let age = now
        .checked_sub(collected)
        .ok_or("Claude Desktop 额度记录时间异常")?;
    if collected <= 0 || age < -5_000 || age > 86_400_000 {
        return Err("Claude Desktop 额度记录已过期或时间异常");
    }
    let org = sample["org"]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or("Claude Desktop 额度记录缺少账户标识")?;
    let used = sample["u"]
        .as_object()
        .ok_or("Claude Desktop 额度记录缺少用量")?;
    let mut windows = Vec::new();
    for (key, label, minutes, short) in [
        ("five_hour", "5 小时额度", 300, "fh"),
        ("seven_day", "7 天额度", 10_080, "sd"),
        ("seven_day_sonnet", "Sonnet · 7 天", 10_080, "sn"),
        ("seven_day_opus", "Opus · 7 天", 10_080, "so"),
        ("seven_day_oauth_apps", "应用 · 7 天", 10_080, "oa"),
        ("seven_day_cowork", "Cowork · 7 天", 10_080, "cw"),
    ] {
        let Some(value) = used.get(short).filter(|value| !value.is_null()) else {
            continue;
        };
        let percent = value
            .as_f64()
            .filter(|v| v.is_finite() && (0.0..=100.0).contains(v))
            .ok_or("Claude Desktop 额度百分比异常")?;
        windows.push(QuotaWindow {
            pool_id: format!("claude:{key}"),
            window_id: "primary".into(),
            label: label.into(),
            used_percent: Some(percent),
            remaining_percent: Some(100.0 - percent),
            resets_at_ms: None,
            duration_minutes: Some(minutes),
            reached_type: (percent >= 100.0).then(|| "limit-reached".into()),
        });
    }
    if windows.is_empty() {
        return Err("Claude Desktop 尚无可识别的额度使用记录");
    }
    let fingerprint = format!("desktop:{:x}", Sha256::digest(org.as_bytes()));
    let stale = age > 600_000;
    let snapshot = QuotaSnapshot {
        identity_key: format!("claude:{fingerprint}"),
        status: "available".into(),
        collected_at_ms: collected,
        windows,
    };
    let mut state = DesktopState::disconnected();
    state.connected = true;
    state.status = if stale { "stale" } else { "available" }.into();
    state.identity_label = Some("Claude Desktop · 本机记录".into());
    state.last_success_at_ms = Some(collected);
    state.message = Some(if stale {
        "Claude Desktop 本机额度记录已超过 10 分钟；打开 Claude 设置中的用量页可促使客户端更新"
    } else {
        "来自 Claude Desktop 本机额度记录；重置时间与聊天 Token 未提供，额度提醒暂停。登录 Claude Code 订阅账户后可显示重置时间"
    }.into());
    // The local record does not independently verify the currently logged-in
    // account. Never send low/recovery notifications from this source.
    state.identity_verified = false;
    state.snapshot = Some(snapshot);
    Ok((state, fingerprint))
}

fn desktop_state() -> Result<(DesktopState, String), Failure> {
    let raw = crate::platform::claude_desktop_history().map_err(Failure::from)?;
    parse_desktop_history(&raw, crate::now_ms()).map_err(Failure::from)
}
fn request(
    client: &reqwest::blocking::Client,
    token: &str,
    endpoint: &str,
) -> Result<Value, Failure> {
    let mut authorization = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
        .map_err(|_| auth("Claude 登录凭据无效"))?;
    authorization.set_sensitive(true);
    let response = client
        .get(format!("https://api.anthropic.com/api/oauth/{endpoint}"))
        .header(reqwest::header::AUTHORIZATION, authorization)
        .header("anthropic-beta", "oauth-2025-04-20")
        .send()
        .map_err(|_| Failure::from("Claude 额度服务连接失败，请检查网络或代理"))?;
    match response.status().as_u16() {
        401 | 403 => {
            return Err(auth(
                "Claude 拒绝额度读取，请在 Claude Code 中检查订阅账户登录与权限",
            ))
        }
        429 => return Err("Claude 请求受到限流，请稍后刷新".into()),
        200 => {}
        _ => return Err("Claude 额度服务暂不可用".into()),
    }
    let mut bytes = Vec::new();
    response
        .take(1_048_577)
        .read_to_end(&mut bytes)
        .map_err(|_| Failure::from("Claude 响应读取失败"))?;
    if bytes.len() > 1_048_576 {
        return Err("Claude 额度响应过大".into());
    }
    serde_json::from_slice(&bytes).map_err(|_| "Claude 额度响应格式发生变化".into())
}
enum Source {
    Code(String),
    Desktop((DesktopState, String)),
}
// A usable Claude Code subscription login verifies the account and returns reset
// times, so it wins. Desktop's unverified local record is only the fallback; the
// two are never merged because they may belong to different accounts.
fn choose_source(
    code: Result<String, Failure>,
    desktop: impl FnOnce() -> Result<(DesktopState, String), Failure>,
) -> Result<Source, Failure> {
    match code {
        Ok(token) => Ok(Source::Code(token)),
        Err(missing) => desktop().map(Source::Desktop).map_err(|_| missing),
    }
}
pub(crate) fn source_fingerprint() -> Result<String, Failure> {
    match choose_source(credentials(), desktop_state)? {
        Source::Code(token) => Ok(format!("{:x}", Sha256::digest(token.as_bytes()))),
        Source::Desktop((_, fingerprint)) => Ok(fingerprint),
    }
}
fn credentials() -> Result<String, Failure> {
    let raw = crate::platform::claude_credentials().map_err(auth)?;
    parse_credentials(&raw, crate::now_ms()).map_err(auth)
}
pub(crate) fn read_once() -> Result<(DesktopState, String), Failure> {
    match choose_source(credentials(), desktop_state)? {
        Source::Code(token) => read_code_once(&token),
        Source::Desktop(desktop) => Ok(desktop),
    }
}

fn read_code_once(token: &str) -> Result<(DesktopState, String), Failure> {
    let fingerprint = format!("{:x}", Sha256::digest(token.as_bytes()));
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(8))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| Failure::from("无法初始化 Claude 安全连接"))?;
    let profile = request(&client, token, "profile")?;
    let account = profile["account"]["uuid"]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or("Claude 未返回可核验的账户身份")?;
    let organization = profile["organization"]["uuid"]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or("Claude 未返回可核验的组织身份")?;
    let raw = request(&client, token, "usage")?;
    let snapshot = normalize_quota(&raw, &format!("{account}:{organization}"), crate::now_ms())?;
    // A login switch during the network call must not publish the previous user's quota.
    if source_fingerprint()? != fingerprint {
        return Err(auth("Claude 登录已变化，请重新刷新"));
    }
    let mut state = DesktopState::disconnected();
    state.connected = true;
    state.identity_verified = true;
    state.status = snapshot.status.clone();
    state.last_success_at_ms = Some(snapshot.collected_at_ms);
    state.identity_label = Some(format!("订阅账户 · {}", &snapshot.identity_key[7..13]));
    state.snapshot = Some(snapshot);
    Ok((state, fingerprint))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn desktop_history_uses_latest_sample_and_never_invents_reset_time() {
        let raw = json!({"version":2,"samples":[
            {"t":1_000_000,"org":"first","u":{"fh":90,"sd":20}},
            {"t":1_100_000,"org":"second","u":{"fh":25,"sd":0,"so":55}}
        ]});
        let (state, fingerprint) =
            parse_desktop_history(raw.to_string().as_bytes(), 1_200_000).unwrap();
        assert_eq!(state.status, "available");
        assert_eq!(state.last_success_at_ms, Some(1_100_000));
        assert!(!state.identity_verified);
        let snapshot = state.snapshot.unwrap();
        assert_eq!(snapshot.windows[0].remaining_percent, Some(75.0));
        assert_eq!(snapshot.windows[1].remaining_percent, Some(100.0));
        assert!(snapshot.windows.iter().all(|w| w.resets_at_ms.is_none()));
        assert!(!fingerprint.contains("second"));
    }
    #[test]
    fn desktop_history_does_not_reuse_previous_account_or_stale_sample() {
        let raw = json!({"version":2,"samples":[
            {"t":1_000_000,"org":"first","u":{"fh":90}},
            {"t":1_100_000,"org":"second","u":{}}
        ]});
        assert!(parse_desktop_history(raw.to_string().as_bytes(), 1_200_000).is_err());
        let old = json!({"version":2,"samples":[{"t":1_000_000,"org":"first","u":{"fh":90}}]});
        assert_eq!(
            parse_desktop_history(old.to_string().as_bytes(), 1_700_001)
                .unwrap()
                .0
                .status,
            "stale"
        );
        assert!(parse_desktop_history(old.to_string().as_bytes(), 1_000_000 + 86_400_001).is_err());
        assert!(parse_desktop_history(old.to_string().as_bytes(), 994_000).is_err());
    }
    #[test]
    fn desktop_history_rejects_invalid_percent_and_schema() {
        for raw in [
            json!({"version":1,"samples":[{"t":1_000_000,"org":"a","u":{"fh":50}}]}),
            json!({"version":2,"samples":[{"t":1_000_000,"org":"a","u":{"fh":101}}]}),
            json!({"version":2,"samples":[{"t":1_000_000,"org":"a","u":{"fh":-1}}]}),
            json!({"version":2,"samples":[{"t":1_000_000,"org":null,"u":{"fh":50}}]}),
            json!({"version":2,"samples":[{"t":1_000_000,"org":"a","u":{"fh":50}},{"org":"b","u":{}}]}),
        ] {
            assert!(parse_desktop_history(raw.to_string().as_bytes(), 1_000_001).is_err());
        }
        let partial =
            json!({"version":2,"samples":[{"t":1_000_000,"org":"a","u":{"fh":42,"sd":null}}]});
        assert_eq!(
            parse_desktop_history(partial.to_string().as_bytes(), 1_000_001)
                .unwrap()
                .0
                .snapshot
                .unwrap()
                .windows
                .len(),
            1
        );
    }
    #[test]
    fn quota_preserves_zero_unknown_and_model_windows() {
        let snapshot=normalize_quota(&json!({"five_hour":{"utilization":0,"resets_at":"2026-10-17T16:00:00Z"},"seven_day":{"utilization":73},"seven_day_sonnet":{"utilization":null},"seven_day_opus":{"utilization":101},"extra_usage":{"used_credits":100}}),"account-a",1000).unwrap();
        assert_eq!(snapshot.windows.len(), 4);
        assert_eq!(snapshot.windows[0].remaining_percent, Some(100.0));
        assert_eq!(snapshot.windows[1].remaining_percent, Some(27.0));
        assert_eq!(snapshot.windows[2].remaining_percent, None);
        assert_eq!(snapshot.windows[3].remaining_percent, None);
        assert!(snapshot.identity_key.starts_with("claude:"));
        assert!(normalize_quota(&json!({"error":"bad"}), "a", 0).is_err());
    }
    fn desktop_fixture() -> Result<(DesktopState, String), Failure> {
        let raw = json!({"version":2,"samples":[{"t":1_000_000,"org":"desktop-org","u":{"fh":13,"sd":5}}]});
        parse_desktop_history(raw.to_string().as_bytes(), 1_000_001).map_err(Failure::from)
    }
    fn login(expires_at: i64) -> Result<String, Failure> {
        let raw = json!({"claudeAiOauth":{"accessToken":"test-only","expiresAt":expires_at,"scopes":["user:profile"]}});
        parse_credentials(&raw.to_string(), 1000).map_err(auth)
    }
    #[test]
    fn usable_code_login_is_preferred_over_desktop_record() {
        let mut desktop_read = false;
        let source = choose_source(login(2000), || {
            desktop_read = true;
            desktop_fixture()
        });
        assert!(matches!(source, Ok(Source::Code(token)) if token == "test-only"));
        assert!(
            !desktop_read,
            "a usable Code login must not mix in Desktop data"
        );
    }
    #[test]
    fn missing_or_expired_code_login_falls_back_to_desktop_record() {
        for code in [Err(auth("未发现 Claude Code 订阅登录")), login(500)] {
            let Ok(Source::Desktop((state, fingerprint))) = choose_source(code, desktop_fixture)
            else {
                panic!("Desktop record should be used without a usable Code login");
            };
            assert!(fingerprint.starts_with("desktop:"));
            assert!(!state.identity_verified);
            assert!(state
                .snapshot
                .unwrap()
                .windows
                .iter()
                .all(|w| w.resets_at_ms.is_none()));
            assert!(state.message.unwrap().contains("登录 Claude Code"));
        }
    }
    #[test]
    fn without_either_source_the_code_login_hint_is_kept() {
        let Err(failure) = choose_source(Err(auth("未发现 Claude Code 订阅登录")), || {
            Err(Failure::from("Claude Desktop 尚无本机额度记录"))
        }) else {
            panic!("no source must stay an error, never an empty quota");
        };
        assert!(failure.auth);
        assert_eq!(failure.message, "未发现 Claude Code 订阅登录");
    }
    #[test]
    #[ignore = "read-only check of this machine's real Claude Desktop record; no credential or keychain access"]
    fn live_desktop_record_parses_on_this_platform() {
        let raw = crate::platform::claude_desktop_history().expect("Desktop usage record");
        let (state, fingerprint) =
            parse_desktop_history(&raw, crate::now_ms()).expect("parsable record");
        let snapshot = state.snapshot.expect("windows");
        println!(
            "status={} windows={:?}",
            state.status,
            snapshot
                .windows
                .iter()
                .map(|w| (w.pool_id.as_str(), w.remaining_percent))
                .collect::<Vec<_>>()
        );
        assert!(
            fingerprint.starts_with("desktop:")
                && snapshot.identity_key.starts_with("claude:desktop:")
        );
        assert!(!state.identity_verified);
    }
    #[test]
    fn credentials_require_profile_scope_and_unexpired_token() {
        let credentials = json!({"claudeAiOauth":{"accessToken":"test-only","expiresAt":2000,"scopes":["user:profile"]}});
        assert!(parse_credentials(&credentials.to_string(), 1000).is_ok());
        assert!(parse_credentials(&credentials.to_string(), 2000).is_err());
        assert!(parse_credentials(
            r#"{"claudeAiOauth":{"accessToken":"x","expiresAt":2000,"scopes":["user:inference"]}}"#,
            1000
        )
        .is_err());
    }
}

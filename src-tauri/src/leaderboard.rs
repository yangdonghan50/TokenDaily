//! Opt-in reporting. Own TokenDaily session only; never reads host credentials.
use crate::{
    claude_usage,
    local_usage::{self, RankEvent},
    platform, workbuddy_usage,
};
use chrono::{Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::Duration,
};
use tauri::{Emitter, Manager};

const CONSENT: &str = "leaderboard-all-tools-v2-2026-10-20";
const INTERVAL: i64 = 30 * 60 * 1000;
const DAY_MS: i64 = 86_400_000;
fn acknowledgment(event: &RankEvent) -> String {
    format!("{}:{}:{}", event.tool, event.id, event.tokens)
}
fn account_batches(mut events: Vec<RankEvent>, timestamp: i64) -> Vec<Vec<RankEvent>> {
    events.sort_by(|a, b| {
        b.at.cmp(&a.at)
            .then_with(|| (&a.tool, &a.id).cmp(&(&b.tool, &b.id)))
    });
    let today = (timestamp + 8 * 3600000).div_euclid(DAY_MS);
    let mut batches: Vec<Vec<RankEvent>> = Vec::new();
    for event in events {
        let current = (event.at + 8 * 3600000).div_euclid(DAY_MS) == today;
        let new_batch = batches.last().map_or(true, |batch| {
            batch.len() == 200
                || ((batch[0].at + 8 * 3600000).div_euclid(DAY_MS) == today) != current
        });
        if new_batch {
            batches.push(Vec::new());
        }
        batches.last_mut().unwrap().push(event);
    }
    batches
}
fn collect_events(
    home: PathBuf,
    since: i64,
    now: i64,
) -> Result<(Vec<RankEvent>, Vec<&'static str>), String> {
    let mut events = Vec::new();
    let mut readable = 0;
    let mut unread = Vec::new();
    for (name, source) in [
        ("Codex", local_usage::rank_events(home, since, now)),
        (
            "Claude Code",
            claude_usage::history_events(platform::claude_home(), since, now),
        ),
        (
            "WorkBuddy",
            workbuddy_usage::history_events(platform::workbuddy_home(), since, now),
        ),
    ] {
        match source {
            Ok(mut source_events) => {
                readable += 1;
                events.append(&mut source_events);
            }
            Err(_) => unread.push(name),
        }
    }
    if readable == 0 {
        return Err("未能读取任何已接入工具的本机用量，未上传".into());
    }
    events.sort_by(|a, b| (a.at, &a.tool, &a.id).cmp(&(b.at, &b.tool, &b.id)));
    Ok((events, unread))
}
fn now() -> i64 {
    Utc::now().timestamp_millis()
}
fn random_id() -> Result<String, String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|_| "无法生成安全设备标识")?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}
fn week_start(ms: i64) -> i64 {
    let date = Utc.timestamp_millis_opt(ms + 8 * 3600000).unwrap();
    date.date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis()
        - date.weekday().num_days_from_monday() as i64 * 86400000
        - 8 * 3600000
}
// A public browser bookmark only. Never place a device ID or credential in a URL.
fn is_public_id(id: &str) -> bool {
    id.len() == 36
        && id.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
}
fn floating_participant(status: &Value) -> Option<&str> {
    if status["loggedIn"] != true {
        return None;
    }
    let remote = &status["remote"];
    let account = &remote["account"];
    let id = if account.is_object() {
        if account["boardHeld"].as_i64() != Some(0) {
            return None;
        }
        account["publicId"].as_str()?
    } else {
        let enrollment = &remote["enrollment"];
        if enrollment["held"].as_i64() != Some(0)
            || enrollment["device"].as_str().is_none()
            || enrollment["device"] != status["thisDevice"]
        {
            return None;
        }
        enrollment["user_id"].as_str()?
    };
    is_public_id(id).then_some(id)
}
fn daily_board_path(status: &Value) -> Option<String> {
    floating_participant(status)
        .map(|id| format!("/api/leaderboard?period=day&tool=all&participant={id}"))
}
fn is_current_daily_board(board: &Value, participant: &str, timestamp: i64) -> bool {
    let day = local_usage::beijing_day(timestamp);
    board["period"] == "day"
        && board["tool"] == "all"
        && board["start"].as_str() == Some(day.as_str())
        && board["end"].as_str() == Some(day.as_str())
        && board
            .get("me")
            .is_some_and(|me| me.is_null() || me["id"].as_str() == Some(participant))
}
fn public_board_url(status: &Value) -> String {
    let base = "https://tokendaily.net/leaderboard/";
    let remote = &status["remote"];
    let enrollment = &remote["enrollment"];
    let account = &remote["account"];
    let id = remote["board"]["me"]["id"].as_str().unwrap_or("");
    let valid_id = is_public_id(id);
    if status["loggedIn"] == true
        && valid_id
        && ((account["publicId"].as_str() == Some(id))
            || (enrollment["held"].as_i64() == Some(0)
                && enrollment["user_id"].as_str() == Some(id)
                && enrollment["device"].as_str().is_some()
                && enrollment["device"] == status["thisDevice"]))
    {
        format!("{base}#participant={id}")
    } else {
        base.into()
    }
}
fn origin() -> String {
    #[cfg(debug_assertions)]
    if let Ok(value) = std::env::var("TOKENDAILY_RANK_LOCAL") {
        if value == "http://127.0.0.1:8787" {
            return value;
        }
    }
    "https://tokendaily.net".into()
}
#[derive(Default, Serialize, Deserialize)]
struct Saved {
    device: String,
    token: Option<String>,
    #[serde(default)]
    trial_pending: bool,
    enabled: bool,
    #[serde(default)]
    account_sync: bool,
    epoch: Option<String>,
    acknowledged: HashSet<String>,
    last_success: Option<i64>,
    #[serde(default)]
    last_current_success: Option<i64>,
}
struct Preview {
    key: String,
    events: Vec<RankEvent>,
    created: i64,
    rebind: bool,
    home: PathBuf,
}
struct Inner {
    saved: Saved,
    preview: Option<Preview>,
    last_attempt: i64,
    message: Option<String>,
    loaded: bool,
    storage_error: Option<String>,
}
pub(crate) struct Service {
    path: PathBuf,
    inner: Mutex<Inner>,
    stopped: AtomicBool,
}
impl Service {
    pub(crate) fn clear_local(&self) -> Result<(), String> {
        self.stopped.store(true, Ordering::SeqCst);
        let mut inner = self.inner.lock().map_err(|_| "账号状态繁忙")?;
        crate::platform::leaderboard_credentials::remove(&self.path)?;
        inner.saved = Saved::default();
        inner.preview = None;
        inner.message = None;
        inner.loaded = true;
        inner.storage_error = None;
        Ok(())
    }
    pub(crate) fn new(dir: &Path) -> Self {
        let path = dir.join(if cfg!(debug_assertions) {
            "leaderboard-dev.protected"
        } else {
            "leaderboard.protected"
        });
        Self {
            path,
            inner: Mutex::new(Inner {
                saved: Saved::default(),
                preview: None,
                last_attempt: 0,
                message: None,
                loaded: false,
                storage_error: None,
            }),
            stopped: AtomicBool::new(false),
        }
    }
    fn restore(&self, inner: &mut Inner, retry: bool) -> Result<(), String> {
        if !inner.loaded || retry {
            inner.loaded = true;
            match crate::platform::leaderboard_credentials::load(&self.path)
                .map_err(String::from)
                .and_then(decode_saved)
            {
                Ok(saved) => {
                    inner.saved = saved;
                    inner.storage_error = None;
                }
                Err(error) => {
                    inner.storage_error = Some(error);
                }
            }
        }
        inner.storage_error.clone().map_or(Ok(()), Err)
    }
    fn persist(&self, inner: &Inner) -> Result<(), String> {
        let bytes = serde_json::to_vec(&inner.saved).map_err(|_| "保存账号失败")?;
        crate::platform::leaderboard_credentials::save(&self.path, &bytes).map_err(|_| {
            self.stopped.store(true, Ordering::SeqCst);
            "无法保存受保护的账号设置，自动上传已停止".into()
        })
    }
    fn ensure_trial(&self, inner: &mut Inner) -> Result<(), String> {
        if inner.saved.token.is_some() && !inner.saved.trial_pending {
            return Ok(());
        }
        if inner.saved.token.is_none() {
            inner.saved.token = Some(format!("td_trial_{}", random_id()?));
        }
        inner.saved.trial_pending = true;
        inner.saved.enabled = false;
        inner.preview = None;
        self.persist(inner)?;
        self.request(
            inner,
            "/api/trial/start",
            Some(json!({"device":inner.saved.device})),
        )?;
        inner.saved.trial_pending = false;
        inner.saved.epoch = None;
        inner.saved.acknowledged.clear();
        inner.saved.last_success = None;
        inner.saved.last_current_success = None;
        self.persist(inner)
    }
    fn request(&self, inner: &mut Inner, path: &str, body: Option<Value>) -> Result<Value, String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(25))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "无法建立安全连接")?;
        let url = format!("{}{path}", origin());
        let mut request = if let Some(body) = body {
            client.post(url).json(&body)
        } else {
            client.get(url)
        };
        if let Some(token) = &inner.saved.token {
            request = request.bearer_auth(token);
        }
        let response = request
            .header("Origin", origin())
            .send()
            .map_err(|_| "网络不可用，尚未同步；本地监测不受影响，稍后可重试")?;
        let status = response.status();
        let value: Value = response
            .json()
            .map_err(|_| "服务器尚未提供榜单接口或响应格式异常")?;
        if !status.is_success() {
            if status.as_u16() == 401 {
                inner.saved.enabled = false;
                inner.saved.account_sync = false;
                inner.saved.token = None;
                inner.saved.trial_pending = false;
                inner.saved.device = random_id()?;
                let _ = self.persist(inner);
            }
            return Err(value["message"]
                .as_str()
                .unwrap_or("服务请求失败，请稍后重试")
                .chars()
                .take(240)
                .collect());
        }
        Ok(value)
    }
    fn status(&self, inner: &mut Inner) -> Result<Value, String> {
        let remote = if inner.saved.token.is_some() && !inner.saved.trial_pending {
            match self.request(inner, "/api/member/status", None) {
                Ok(value) => {
                    if value["account"]["accountId"].as_str().is_some() && !inner.saved.account_sync
                    {
                        inner.saved.account_sync = true;
                        inner.saved.enabled = false;
                        inner.saved.epoch = None;
                        inner.saved.acknowledged.clear();
                        inner.saved.last_success = None;
                        inner.saved.last_current_success = None;
                        self.persist(inner)?;
                        self.stopped.store(false, Ordering::SeqCst);
                    }
                    if value["account"].is_null() && inner.saved.account_sync {
                        inner.saved.account_sync = false;
                        inner.saved.enabled = false;
                        inner.saved.acknowledged.clear();
                        inner.saved.last_success = None;
                        inner.saved.last_current_success = None;
                        self.persist(inner)?;
                    }
                    value
                }
                Err(error) => {
                    inner.message = Some(error);
                    Value::Null
                }
            }
        } else {
            Value::Null
        };
        Ok(
            json!({ "supported": true, "storageReady": true, "loggedIn": inner.saved.token.is_some() && !inner.saved.trial_pending, "enabled": inner.saved.enabled || inner.saved.account_sync, "accountSync": inner.saved.account_sync,
            "thisDevice": inner.saved.device, "lastSuccess": inner.saved.last_success.max(inner.saved.last_current_success), "message": inner.message, "remote": remote }),
        )
    }
    fn daily_status(&self, inner: &mut Inner) -> Result<Value, String> {
        // The authenticated status endpoint intentionally remains a weekly view
        // for the details window. Fetch the existing public daily projection for
        // the verified public identity; never relabel that weekly result.
        let mut status = self.status(inner)?;
        if !status["remote"].is_object() {
            return Ok(status);
        }
        let participant = floating_participant(&status).map(str::to_owned);
        let path = daily_board_path(&status);
        let day = local_usage::beijing_day(now());
        status["remote"]["board"] =
            json!({"period":"day","tool":"all","start":day,"end":day,"me":null});
        if let (Some(id), Some(path)) = (participant, path) {
            match self.request(inner, &path, None) {
                Ok(board) if is_current_daily_board(&board, &id, now()) => {
                    status["remote"]["board"] = board
                }
                result => {
                    status["remote"]["board"] = Value::Null;
                    status["message"] = json!(result
                        .err()
                        .unwrap_or_else(|| "当日榜单日期或身份不一致，请稍后刷新".into()));
                }
            }
        }
        Ok(status)
    }
    fn sync_events(
        &self,
        inner: &mut Inner,
        events: Vec<RankEvent>,
        enrollment: &Value,
    ) -> Result<(), String> {
        let epoch = enrollment["epoch"].as_str().ok_or("未绑定设备")?;
        if enrollment["device"].as_str() != Some(&inner.saved.device)
            || enrollment["active"].as_i64() != Some(1)
        {
            inner.saved.enabled = false;
            self.persist(inner)?;
            return Err("本机不是绑定设备，或上传已暂停".into());
        }
        if inner.saved.epoch.as_deref() != Some(epoch) {
            inner.saved.acknowledged.clear();
            inner.saved.epoch = Some(epoch.into());
        }
        let since = enrollment["since_ms"].as_i64().ok_or("绑定时间缺失")?;
        let events: Vec<_> = events
            .into_iter()
            .filter(|e| e.at >= since && !inner.saved.acknowledged.contains(&acknowledgment(e)))
            .collect();
        // Empty upload confirms source readability and updates last-sync time, not a fabricated zero.
        let batches: Vec<&[RankEvent]> = if events.is_empty() {
            vec![&[]]
        } else {
            events.chunks(200).collect()
        };
        for batch in batches {
            if self.stopped.load(Ordering::SeqCst) || !inner.saved.enabled {
                return Err("上传已停止".into());
            }
            self.request(
                inner,
                "/api/member/upload",
                Some(json!({ "device": inner.saved.device, "epoch": epoch, "events": batch })),
            )?;
            inner
                .saved
                .acknowledged
                .extend(batch.iter().map(acknowledgment));
            self.persist(inner)?;
        }
        inner.saved.last_success = Some(now());
        inner.message = None;
        self.persist(inner)
    }
    fn sync(&self, inner: &mut Inner, home: Result<PathBuf, &'static str>) -> Result<(), String> {
        if (!inner.saved.enabled && !inner.saved.account_sync) || inner.saved.token.is_none() {
            return Err("请先预览并同意参榜，或恢复上传".into());
        }
        inner.last_attempt = now();
        let remote = self.request(inner, "/api/member/status", None)?;
        if remote["account"]["accountId"].as_str().is_some() {
            if !inner.saved.account_sync {
                inner.saved.account_sync = true;
                inner.saved.enabled = false;
                inner.saved.acknowledged.clear();
                inner.saved.last_success = None;
                inner.saved.last_current_success = None;
                self.persist(inner)?;
            }
            let since = inner
                .saved
                .last_success
                .map_or(0, |time| time.saturating_sub(7 * DAY_MS));
            let (events, unread) = collect_events(home?, since, now())?;
            let current: HashSet<_> = events.iter().map(acknowledgment).collect();
            inner.saved.acknowledged.retain(|key| current.contains(key));
            let pending: Vec<_> = events
                .into_iter()
                .filter(|event| !inner.saved.acknowledged.contains(&acknowledgment(event)))
                .collect();
            if pending.is_empty() {
                self.request(
                    inner,
                    "/api/device/upload",
                    Some(json!({ "device": inner.saved.device, "epoch": "account", "events": [] })),
                )?;
            }
            for batch in account_batches(pending, now()) {
                if self.stopped.load(Ordering::SeqCst) || !inner.saved.account_sync {
                    return Err("同步已停止".into());
                }
                self.request(inner, "/api/device/upload", Some(json!({ "device": inner.saved.device, "epoch": "account", "events": batch })))?;
                inner
                    .saved
                    .acknowledged
                    .extend(batch.iter().map(acknowledgment));
                if (batch[0].at + 8 * 3600000).div_euclid(DAY_MS)
                    == (now() + 8 * 3600000).div_euclid(DAY_MS)
                {
                    inner.saved.last_current_success = Some(now());
                }
                self.persist(inner)?;
            }
            inner.saved.last_success = Some(now());
            inner.message = if unread.is_empty() {
                None
            } else {
                Some(format!(
                    "统计不完整：未读取 {}；已同步其他来源。",
                    unread.join("、")
                ))
            };
            return self.persist(inner);
        }
        if inner.saved.account_sync {
            inner.saved.account_sync = false;
            inner.saved.enabled = false;
            self.persist(inner)?;
            return Err("官网账号连接已失效，请重新连接".into());
        }
        let enrollment = &remote["enrollment"];
        let since = enrollment["since_ms"]
            .as_i64()
            .ok_or("尚未参榜")?
            .max(week_start(now()) - 7 * 86400000);
        let (events, unread) = collect_events(home?, since, now())?;
        // Bound checkpoint size to the source's accepted reporting window.
        let current: HashSet<_> = events.iter().map(acknowledgment).collect();
        inner.saved.acknowledged.retain(|id| current.contains(id));
        self.sync_events(inner, events, enrollment)?;
        inner.message = if unread.is_empty() {
            None
        } else {
            Some(format!(
                "统计不完整：未读取 {}；已同步其他来源。",
                unread.join("、")
            ))
        };
        Ok(())
    }
    fn action(
        &self,
        action: &str,
        body: Value,
        home: Result<PathBuf, &'static str>,
    ) -> Result<Value, String> {
        if action == "open" {
            let status = self.inner.lock().ok().and_then(|mut inner| {
                self.restore(&mut inner, false).ok()?;
                self.status(&mut inner).ok()
            });
            crate::platform::open_official_url(&public_board_url(&status.unwrap_or(Value::Null)))?;
            return Ok(Value::Null);
        }
        if action == "open-account" {
            crate::platform::open_official_url(&format!("{}/zh/account/", origin()))?;
            return Ok(Value::Null);
        }
        let mut inner = self.inner.lock().map_err(|_| "账号状态繁忙")?;
        if let Err(error) = self.restore(&mut inner, action == "retry-storage") {
            if action == "status" || action == "daily-status" {
                return Ok(
                    json!({"supported":true,"storageReady":false,"loggedIn":false,"enabled":false,"thisDevice":"","lastSuccess":null,"message":error,"remote":null}),
                );
            }
            return Err(error);
        }
        if inner.saved.device.is_empty() {
            inner.saved.device = random_id()?;
        }
        match action {
            "status" | "retry-storage" => return self.status(&mut inner),
            "daily-status" => return self.daily_status(&mut inner),
            "start" => {
                self.ensure_trial(&mut inner)?;
                return self.status(&mut inner); // No credential crosses the Rust/JS boundary.
            }
            "preview" => {
                if inner.saved.token.is_none() || inner.saved.trial_pending {
                    return Err("请先开始免注册试用".into());
                }
                let rebind = body["rebind"].as_bool().unwrap_or(false);
                if rebind {
                    return Err("试用版不支持跨设备重绑".into());
                }
                let timestamp = now();
                let since = if rebind {
                    timestamp
                } else {
                    week_start(timestamp)
                };
                let home = home?;
                let (events, unread) = collect_events(home.clone(), since, timestamp)?;
                let mut days = BTreeMap::<String, u64>::new();
                for e in &events {
                    let total = days.entry(local_usage::beijing_day(e.at)).or_default();
                    *total = total
                        .checked_add(e.tokens)
                        .ok_or("预览 Token 合计超出范围")?;
                }
                let preview_total = events
                    .iter()
                    .try_fold(0u64, |total, event| total.checked_add(event.tokens))
                    .ok_or("预览 Token 合计超出范围")?;
                let key = random_id()?;
                let result = json!({ "key": key, "days": days, "records": events.len(), "total": preview_total, "since": since, "until": timestamp, "rebind": rebind, "unread": unread });
                inner.preview = Some(Preview {
                    key,
                    events,
                    created: timestamp,
                    rebind,
                    home,
                });
                return Ok(result);
            }
            "enroll" => {
                let preview = inner.preview.take().ok_or("请先预览待上传内容")?;
                if body["key"].as_str() != Some(&preview.key)
                    || body["confirm"].as_bool() != Some(true)
                    || now() - preview.created > 600000
                    || week_start(now()) != week_start(preview.created)
                    || home.as_ref().ok() != Some(&preview.home)
                {
                    return Err("预览已过期，请重新预览并确认").map_err(String::from);
                }
                let remote = self.request(&mut inner, "/api/member/status", None)?;
                let action = if preview.rebind { "rebind" } else { "enroll" };
                let payload = json!({ "device": inner.saved.device, "nickname": body["nickname"], "consent": CONSENT, "confirm": true, "epoch": remote["enrollment"]["epoch"] });
                let enrollment =
                    self.request(&mut inner, &format!("/api/member/{action}"), Some(payload))?;
                inner.saved.enabled = true;
                inner.saved.acknowledged.clear();
                inner.last_attempt = now();
                self.persist(&inner)?;
                self.stopped.store(false, Ordering::SeqCst);
                self.sync_events(&mut inner, preview.events, &enrollment["enrollment"])?;
            }
            "sync" => {
                self.stopped.store(false, Ordering::SeqCst);
                self.sync(&mut inner, home)?;
            }
            "pair" => {
                self.ensure_trial(&mut inner)?;
                let pair = self.request(&mut inner, "/api/device/pair-start", Some(json!({})))?;
                let url = pair["url"].as_str().ok_or("设备连接地址无效")?;
                if !url.starts_with(&format!("{}/zh/account/#pair=", origin())) {
                    return Err("设备连接地址无效".into());
                }
                crate::platform::open_official_url(url)?;
                return self.status(&mut inner);
            }
            "pause" | "resume" | "leave" => {
                inner.saved.enabled = false;
                self.persist(&inner)?;
                let remote = self.request(&mut inner, "/api/member/status", None)?;
                if action == "leave" {
                    if body["confirm"].as_bool() != Some(true) {
                        return Err("请确认删除上传统计".into());
                    }
                    self.request(
                        &mut inner,
                        "/api/member/leave",
                        Some(json!({ "confirm": true, "epoch": remote["enrollment"]["epoch"] })),
                    )?;
                    inner.saved.epoch = None;
                    inner.saved.acknowledged.clear();
                    inner.saved.last_success = None;
                    inner.saved.last_current_success = None;
                } else {
                    if action == "resume"
                        && remote["enrollment"]["device"].as_str() != Some(&inner.saved.device)
                    {
                        return Err("本机不是原试用设备，不能恢复上传".into());
                    }
                    self.request(&mut inner, "/api/member/pause", Some(json!({ "active": action == "resume", "epoch": remote["enrollment"]["epoch"] })))?;
                    inner.saved.enabled = action == "resume";
                    if action == "resume" {
                        self.stopped.store(false, Ordering::SeqCst);
                    }
                }
                self.persist(&inner)?;
            }
            "delete" => {
                if body["confirm"].as_bool() != Some(true) {
                    return Err("请确认删除试用身份和统计".into());
                }
                inner.saved.enabled = false;
                self.persist(&inner)?;
                self.request(
                    &mut inner,
                    "/api/trial/delete",
                    Some(json!({ "confirm": true })),
                )?;
                inner.saved.token = None;
                inner.saved.trial_pending = false;
                inner.saved.device = random_id()?;
                inner.saved.epoch = None;
                inner.saved.acknowledged.clear();
                inner.saved.last_success = None;
                inner.saved.last_current_success = None;
                inner.preview = None;
                self.persist(&inner)?;
            }
            _ => return Err("未知账号操作".into()),
        }
        self.status(&mut inner)
    }
}
fn decode_saved(bytes: Option<Vec<u8>>) -> Result<Saved, String> {
    match bytes {
        None => Ok(Saved::default()),
        Some(bytes) => serde_json::from_slice(&bytes)
            .map_err(|_| "参榜凭据损坏，已停止恢复；不会覆盖原凭据".into()),
    }
}
#[tauri::command]
pub(crate) async fn leaderboard_action(
    app: tauri::AppHandle,
    action: String,
    body: Value,
) -> Result<Value, String> {
    if ["logout", "pause", "leave", "delete"].contains(&action.as_str()) {
        app.state::<Service>().stopped.store(true, Ordering::SeqCst);
    }
    tauri::async_runtime::spawn_blocking(move || {
        let home = app
            .state::<crate::AppState>()
            .connection
            .lock()
            .unwrap()
            .home();
        app.state::<Service>().action(&action, body, home)
    })
    .await
    .map_err(|_| "账号操作失败".to_string())?
}
pub(crate) fn start(app: tauri::AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(15));
        let service = app.state::<Service>();
        if service.stopped.load(Ordering::SeqCst) {
            continue;
        }
        if let Ok(mut inner) = service.inner.try_lock() {
            if service.restore(&mut inner, false).is_err() {
                continue;
            }
            if (inner.saved.enabled || inner.saved.account_sync)
                && inner.saved.token.is_some()
                && now() - inner.last_attempt >= INTERVAL
            {
                let home = app
                    .state::<crate::AppState>()
                    .connection
                    .lock()
                    .unwrap()
                    .home();
                if let Err(message) = service.sync(&mut inner, home) {
                    inner.message = Some(message);
                }
                let _ = app.emit("leaderboard-updated", ());
            }
        };
    });
}
#[cfg(test)]
#[path = "leaderboard_macos_tests.rs"]
#[cfg(target_os = "macos")]
mod macos_tests;
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_os = "windows")]
    use std::fs;
    #[test]
    fn daily_rank_uses_the_authenticated_public_identity_even_without_a_weekly_rank() {
        let id = "12345678-1234-4234-8234-123456789abc";
        let mut status = json!({"loggedIn":true,"thisDevice":"device","remote":{"enrollment":{"user_id":id,"device":"device","held":0},"board":{"me":null}}});
        assert_eq!(
            daily_board_path(&status),
            Some(format!(
                "/api/leaderboard?period=day&tool=all&participant={id}"
            ))
        );
        status["remote"]["enrollment"]["device"] = json!("other");
        assert_eq!(daily_board_path(&status), None);
        status["remote"]["account"] = json!({"publicId":id,"boardHeld":0});
        assert!(daily_board_path(&status).is_some());
        status["remote"]["account"]["boardHeld"] = json!(1);
        assert_eq!(daily_board_path(&status), None);
        status["remote"]["account"] = json!({"publicId":"td_trial_private","boardHeld":0});
        assert_eq!(daily_board_path(&status), None);
        status["loggedIn"] = json!(false);
        assert_eq!(daily_board_path(&status), None);
    }
    #[test]
    fn daily_rank_rejects_wrong_period_day_tool_or_participant() {
        let id = "12345678-1234-4234-8234-123456789abc";
        let timestamp = 1_792_267_200_000; // 2026-10-18 04:00 UTC+8
        let day = local_usage::beijing_day(timestamp);
        let board = json!({"period":"day","tool":"all","start":day,"end":day,"me":{"id":id,"rank":2,"tokens":123}});
        assert!(is_current_daily_board(&board, id, timestamp));
        for (key, value) in [
            ("period", "week"),
            ("start", "2000-01-01"),
            ("end", "2000-01-01"),
            ("tool", "codex"),
        ] {
            let mut invalid = board.clone();
            invalid[key] = json!(value);
            assert!(!is_current_daily_board(&invalid, id, timestamp));
        }
        let mut other = board.clone();
        other["me"]["id"] = json!("other");
        assert!(!is_current_daily_board(&other, id, timestamp));
        let mut no_rank = board.clone();
        no_rank["me"] = Value::Null;
        assert!(is_current_daily_board(&no_rank, id, timestamp));
        assert!(!is_current_daily_board(&board, id, timestamp + DAY_MS));
    }
    #[test]
    fn account_batches_send_current_usage_before_history_without_mixing_budgets() {
        let timestamp = 1_792_569_600_000i64;
        let events = vec![
            RankEvent {
                id: "old".into(),
                at: timestamp - 30 * DAY_MS,
                tokens: 1,
                tool: "codex".into(),
            },
            RankEvent {
                id: "today".into(),
                at: timestamp - 1000,
                tokens: 2,
                tool: "claude".into(),
            },
            RankEvent {
                id: "yesterday".into(),
                at: timestamp - DAY_MS,
                tokens: 3,
                tool: "workbuddy".into(),
            },
        ];
        let batches = account_batches(events, timestamp);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0][0].id, "today");
        assert_eq!(batches[1][0].id, "yesterday");
        assert_eq!(batches[1][1].id, "old");
        let many = (0..405)
            .map(|i| RankEvent {
                id: i.to_string(),
                at: timestamp - i,
                tokens: 1,
                tool: "codex".into(),
            })
            .collect();
        assert_eq!(
            account_batches(many, timestamp)
                .iter()
                .map(Vec::len)
                .collect::<Vec<_>>(),
            vec![200, 200, 5]
        );
    }
    #[test]
    fn corrupt_or_denied_storage_never_creates_a_replacement_identity() {
        assert!(decode_saved(Some(b"invalid".to_vec())).is_err());
        assert!(decode_saved(None).unwrap().token.is_none());
        let service = Service::new(&std::env::temp_dir().join("unused-rank-fixture"));
        {
            let mut inner = service.inner.lock().unwrap();
            assert!(
                !inner.loaded,
                "construction must not touch the system credential store"
            );
            inner.loaded = true;
            inner.storage_error = Some("synthetic denied".into());
        }
        let status = service.action("status", json!({}), Err("unused")).unwrap();
        assert_eq!(status["storageReady"], false);
        assert_eq!(status["enabled"], false);
        assert!(service.action("start", json!({}), Err("unused")).is_err());
        assert!(service.inner.lock().unwrap().saved.token.is_none());
        assert!(service.inner.lock().unwrap().saved.device.is_empty());
    }
    #[test]
    #[cfg(target_os = "windows")]
    fn explicit_restore_finds_an_existing_identity_after_an_initial_empty_read() {
        let root =
            std::env::temp_dir().join(format!("tokendaily-restore-{}", random_id().unwrap()));
        fs::create_dir_all(&root).unwrap();
        let service = Service::new(&root);
        let mut inner = service.inner.lock().unwrap();
        service.restore(&mut inner, false).unwrap();
        assert!(inner.saved.token.is_none());
        let mut saved = Saved::default();
        saved.device = "d".repeat(64);
        saved.token = Some("synthetic-own-token".into());
        saved.account_sync = true;
        crate::platform::leaderboard_credentials::save(
            &service.path,
            &serde_json::to_vec(&saved).unwrap(),
        )
        .unwrap();
        service.restore(&mut inner, true).unwrap();
        assert_eq!(inner.saved.token.as_deref(), Some("synthetic-own-token"));
        assert!(inner.saved.account_sync);
        drop(inner);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn enrollment_requires_preview_and_matching_source_before_any_network() {
        let service = Service::new(&std::env::temp_dir().join("unused-rank-preview"));
        {
            service.inner.lock().unwrap().loaded = true;
        }
        assert!(service
            .action("enroll", json!({"confirm":true}), Err("unused"))
            .is_err());
        {
            service.inner.lock().unwrap().preview = Some(Preview {
                key: "test".into(),
                events: vec![],
                created: now(),
                rebind: false,
                home: "/source/one".into(),
            });
        }
        assert!(service
            .action(
                "enroll",
                json!({"key":"test","confirm":true}),
                Ok("/source/two".into())
            )
            .is_err());
        assert!(!service.inner.lock().unwrap().saved.enabled);
    }
    #[test]
    fn public_board_link_contains_only_verified_public_identity() {
        let id = "12345678-1234-4234-8234-123456789abc";
        let mut status = json!({"loggedIn":true,"thisDevice":"device","remote":{"enrollment":{"user_id":id,"device":"device","held":0},"board":{"me":{"id":id}}}});
        assert_eq!(
            public_board_url(&status),
            format!("https://tokendaily.net/leaderboard/#participant={id}")
        );
        status["remote"]["enrollment"]["device"] = json!("other");
        assert_eq!(
            public_board_url(&status),
            "https://tokendaily.net/leaderboard/"
        );
        status["remote"]["enrollment"]["device"] = json!("device");
        status["remote"]["board"]["me"]["id"] = json!("td_trial_not_a_public_id");
        assert_eq!(
            public_board_url(&status),
            "https://tokendaily.net/leaderboard/"
        );
        status["remote"]["board"]["me"]["id"] = json!(id);
        status["remote"]["enrollment"]["held"] = json!(1);
        assert_eq!(
            public_board_url(&status),
            "https://tokendaily.net/leaderboard/"
        );
        assert_eq!(
            public_board_url(&Value::Null),
            "https://tokendaily.net/leaderboard/"
        );
    }

    #[test]
    fn monday_beijing_boundary() {
        let monday = 1792339200000; // 2026-10-19 00:00 UTC+8
        assert_eq!(week_start(monday), monday);
        assert_eq!(week_start(monday - 1), monday - 7 * 86400000);
    }
    #[test]
    fn device_ids_are_random_not_hardware() {
        let a = random_id().unwrap();
        assert_eq!(a.len(), 64);
        assert_ne!(a, random_id().unwrap());
    }
    #[test]
    #[cfg(target_os = "windows")]
    fn pending_trial_preserves_retry_identity_without_reporting_after_restart() {
        let root = std::env::temp_dir().join(format!("tokendaily-trial-{}", random_id().unwrap()));
        fs::create_dir_all(&root).unwrap();
        let service = Service::new(&root);
        let device = random_id().unwrap();
        let token = format!("td_trial_{}", random_id().unwrap());
        {
            let mut inner = service.inner.lock().unwrap();
            inner.saved.device = device.clone();
            inner.saved.token = Some(token.clone());
            inner.saved.trial_pending = true;
            service.persist(&inner).unwrap();
        }
        let restored = Service::new(&root);
        let status = restored.action("status", json!({}), Err("unused")).unwrap();
        assert_eq!(status["loggedIn"], false);
        assert_eq!(status["enabled"], false);
        assert!(status["remote"].is_null());
        assert_eq!(status["thisDevice"], device);
        assert_eq!(
            restored.inner.lock().unwrap().saved.token.as_deref(),
            Some(token.as_str())
        );
        assert!(!String::from_utf8_lossy(&fs::read(&restored.path).unwrap()).contains(&token));
        restored.clear_local().unwrap();
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    #[cfg(target_os = "windows")]
    fn secure_session_restarts_opt_in_and_clear_stops_upload_without_host_data() {
        let root =
            std::env::temp_dir().join(format!("tokendaily-rank-secret-{}", random_id().unwrap()));
        fs::create_dir_all(&root).unwrap();
        let service = Service::new(&root);
        assert!(!service.inner.lock().unwrap().saved.enabled);
        assert!(service
            .action("enroll", json!({"confirm":true}), Err("unused"))
            .is_err());
        {
            let mut state = service.inner.lock().unwrap();
            state.saved.device = random_id().unwrap();
            state.saved.token = Some("synthetic-session-not-a-real-credential".into());
            state.saved.enabled = true;
            service.persist(&state).unwrap();
        }
        let bytes = fs::read(&service.path).unwrap();
        assert!(!String::from_utf8_lossy(&bytes).contains("synthetic-session"));
        let restored = Service::new(&root);
        restored
            .restore(&mut restored.inner.lock().unwrap(), false)
            .unwrap();
        assert!(restored.inner.lock().unwrap().saved.enabled);
        assert!(restored.inner.lock().unwrap().saved.token.is_some());
        restored.clear_local().unwrap();
        assert!(restored.stopped.load(Ordering::SeqCst));
        assert!(restored.inner.lock().unwrap().saved.token.is_none());
        assert!(!restored.path.exists());
        fs::remove_dir_all(root).unwrap();
    }
}

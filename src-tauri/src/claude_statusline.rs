//! Optional Claude Code statusLine bridge. It records only rate-limit percentages and
//! reset times, then forwards the original input to the user's previous statusLine.
use crate::codex::{DesktopState, QuotaSnapshot, QuotaWindow};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Serialize,
};
use serde_json::{value::RawValue, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{ErrorKind, Read},
    path::{Path, PathBuf},
};

pub const FLAG: &str = "--claude-statusline";
const MAX_INPUT: u64 = 4 << 20;
const MAX_SETTINGS: u64 = 4 << 20;
const FRESH_MS: i64 = 600_000;
const MAX_AGE_MS: i64 = 86_400_000;
const BRIDGE: &str = "claude-statusline.json";
const BACKUP: &str = "claude-settings.backup.json";
const CAPTURE: &str = "claude-statusline-capture.json";
const WINDOWS: [(&str, &str, &str, i64); 2] = [
    ("five_hour", "claude:five_hour", "5 小时额度", 300),
    ("seven_day", "claude:seven_day", "7 天额度", 10_080),
];

/// Top-level keys in their original order; nested values keep their original bytes.
struct Ordered(Vec<(String, Box<RawValue>)>);
impl<'de> Deserialize<'de> for Ordered {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Entries;
        impl<'de> Visitor<'de> for Entries {
            type Value = Ordered;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Ordered, A::Error> {
                let mut entries = Vec::new();
                while let Some(entry) = map.next_entry::<String, Box<RawValue>>()? {
                    entries.push(entry);
                }
                Ok(Ordered(entries))
            }
        }
        deserializer.deserialize_map(Entries)
    }
}
impl Ordered {
    fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let ordered: Self = serde_json::from_slice(bytes)
            .map_err(|_| "Claude Code 设置文件不是有效的 JSON 对象，未做修改")?;
        if ordered.0.iter().filter(|(k, _)| k == "statusLine").count() > 1 {
            return Err("Claude Code 设置文件含重复的 statusLine，未做修改");
        }
        Ok(ordered)
    }
    fn position(&self) -> Option<usize> {
        self.0.iter().position(|(k, _)| k == "statusLine")
    }
    fn render(&self) -> Vec<u8> {
        if self.0.is_empty() {
            return b"{}\n".to_vec();
        }
        let body = self
            .0
            .iter()
            .map(|(k, v)| format!("  {}: {}", Value::String(k.clone()), v.get()))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("{{\n{body}\n}}\n").into_bytes()
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Bridge {
    version: u32,
    /// Original statusLine value, byte for byte, or None when there was none.
    previous: Option<String>,
    installed: String,
    installed_sha256: String,
    existed: bool,
}
pub(crate) struct Install {
    file: Vec<u8>,
    bridge: Bridge,
}
#[derive(Debug, PartialEq)]
pub(crate) enum Restore {
    Write(Vec<u8>),
    Remove,
    Changed,
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn install(existing: Option<&[u8]>, command: &str) -> Result<Install, &'static str> {
    let mut ordered = match existing {
        Some(bytes) => Ordered::parse(bytes)?,
        None => Ordered(Vec::new()),
    };
    let position = ordered.position();
    let previous = position.map(|i| ordered.0[i].1.get().to_owned());
    let mut value = match previous.as_deref() {
        Some(raw) => match serde_json::from_str::<Value>(raw) {
            Ok(Value::Object(map)) => map,
            _ => return Err("现有 statusLine 格式无法识别，未做修改"),
        },
        None => serde_json::Map::new(),
    };
    value.insert("type".into(), "command".into());
    value.insert("command".into(), command.into());
    let installed = serde_json::to_string_pretty(&Value::Object(value))
        .map_err(|_| "无法生成 statusLine 设置")?
        .replace('\n', "\n  ");
    let raw = RawValue::from_string(installed.clone()).map_err(|_| "无法生成 statusLine 设置")?;
    match position {
        Some(i) => ordered.0[i].1 = raw,
        None => ordered.0.push(("statusLine".into(), raw)),
    }
    let file = ordered.render();
    Ok(Install {
        bridge: Bridge {
            version: 1,
            previous,
            installed,
            installed_sha256: sha256_hex(&file),
            existed: existing.is_some(),
        },
        file,
    })
}

pub(crate) fn restore(
    current: Option<&[u8]>,
    bridge: &Bridge,
    backup: Option<&[u8]>,
) -> Result<Restore, &'static str> {
    let Some(current) = current else {
        return Ok(Restore::Changed);
    };
    if sha256_hex(current) == bridge.installed_sha256 {
        match (bridge.existed, backup) {
            (false, _) => return Ok(Restore::Remove),
            (true, Some(backup)) => return Ok(Restore::Write(backup.to_vec())),
            (true, None) => {}
        }
    }
    let mut ordered = Ordered::parse(current)?;
    let Some(i) = ordered.position() else {
        return Ok(Restore::Changed);
    };
    let ours = serde_json::from_str::<Value>(&bridge.installed).ok();
    if serde_json::from_str::<Value>(ordered.0[i].1.get()).ok() != ours {
        return Ok(Restore::Changed);
    }
    match bridge.previous.as_deref() {
        Some(raw) => {
            ordered.0[i].1 =
                RawValue::from_string(raw.to_owned()).map_err(|_| "备份的 statusLine 已损坏")?
        }
        None => {
            ordered.0.remove(i);
        }
    }
    Ok(Restore::Write(ordered.render()))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CaptureWindow {
    key: String,
    used_percent: f64,
    resets_at_ms: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Capture {
    version: u32,
    captured_at_ms: i64,
    /// SHA-256 of the Claude organization UUID; the raw identifier is never stored.
    org: Option<String>,
    windows: Vec<CaptureWindow>,
}

pub(crate) fn parse_input(input: &[u8], now: i64) -> Option<Capture> {
    let data: Value = serde_json::from_slice(input).ok()?;
    let limits = data.get("rate_limits")?;
    let windows: Vec<_> = WINDOWS
        .iter()
        .filter_map(|(key, ..)| {
            let window = limits.get(*key)?;
            let used = window["used_percentage"]
                .as_f64()
                .filter(|v| v.is_finite() && (0.0..=100.0).contains(v))?;
            let resets = window["resets_at"]
                .as_f64()
                .filter(|v| v.is_finite() && *v > 0.0 && *v < 1e11)?;
            Some(CaptureWindow {
                key: (*key).into(),
                used_percent: used,
                resets_at_ms: (resets * 1000.0) as i64,
            })
        })
        .collect();
    (!windows.is_empty()).then_some(Capture {
        version: 1,
        captured_at_ms: now,
        org: None,
        windows,
    })
}

pub(crate) fn org_hash(account: &[u8]) -> Option<String> {
    let data: Value = serde_json::from_slice(account).ok()?;
    data["oauthAccount"]["organizationUuid"]
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 128)
        .map(|org| sha256_hex(org.as_bytes()))
}

fn live_window(window: &CaptureWindow, now: i64) -> Option<QuotaWindow> {
    let (_, pool, label, minutes) = WINDOWS.iter().find(|(key, ..)| *key == window.key)?;
    (window.resets_at_ms > now).then(|| QuotaWindow {
        pool_id: (*pool).into(),
        window_id: "primary".into(),
        label: (*label).into(),
        used_percent: Some(window.used_percent),
        remaining_percent: Some(100.0 - window.used_percent),
        resets_at_ms: Some(window.resets_at_ms),
        duration_minutes: Some(*minutes),
        reached_type: (window.used_percent >= 100.0).then(|| "limit-reached".into()),
    })
}

/// Combines a statusLine capture with Claude Desktop's local history for the same
/// organization. Returns None when the capture must not change the current state.
pub(crate) fn merge(
    desktop: Option<&DesktopState>,
    capture: &Capture,
    now: i64,
) -> Option<DesktopState> {
    let age = now.saturating_sub(capture.captured_at_ms);
    if age > MAX_AGE_MS || capture.captured_at_ms > now.saturating_add(120_000) {
        return None;
    }
    // Same key shape as the shared Desktop record parser in `claude.rs`.
    let identity_key = format!("claude:desktop:{}", capture.org.as_deref()?);
    let live: Vec<_> = capture
        .windows
        .iter()
        .filter_map(|w| live_window(w, now))
        .collect();
    if live.is_empty() {
        return None;
    }
    if let Some((state, snapshot)) = desktop.and_then(|d| d.snapshot.as_ref().map(|s| (d, s))) {
        if snapshot.identity_key != identity_key {
            return None;
        }
        if snapshot.collected_at_ms >= capture.captured_at_ms {
            let mut next = state.clone();
            let mut attached = false;
            for window in next.snapshot.as_mut()?.windows.iter_mut() {
                if let Some(live) = live.iter().find(|l| l.pool_id == window.pool_id) {
                    window.resets_at_ms = live.resets_at_ms;
                    attached = true;
                }
            }
            if !attached {
                return None;
            }
            next.identity_label = Some("Claude Desktop 本机历史 · 重置时间来自 Claude Code".into());
            // The record's "reset times unknown" hint no longer applies.
            next.message = None;
            return Some(next);
        }
    }
    let mut state = DesktopState::disconnected();
    state.connected = true;
    // Like Desktop history, a capture cannot prove which account is signed in now.
    state.identity_verified = false;
    state.status = if age > FRESH_MS { "stale" } else { "available" }.into();
    state.identity_label = Some("Claude Code 状态栏 · 本机快照".into());
    state.last_success_at_ms = Some(capture.captured_at_ms);
    state.snapshot = Some(QuotaSnapshot {
        identity_key,
        status: "available".into(),
        collected_at_ms: capture.captured_at_ms,
        windows: live,
    });
    Some(state)
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r"'\''"))
}

pub(crate) fn command_for(executable: &Path, data_dir: &Path) -> String {
    format!(
        "{} {FLAG} {}",
        shell_quote(&executable.to_string_lossy()),
        shell_quote(&data_dir.to_string_lossy())
    )
}

fn read_bounded(path: &Path, limit: u64) -> std::io::Result<Option<Vec<u8>>> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(ErrorKind::InvalidData.into());
    }
    Ok(Some(bytes))
}

/// Replaces a file through a sibling temporary file so readers never see a partial write.
fn replace_file(path: &Path, bytes: &[u8], private: bool) -> std::io::Result<()> {
    let temp = path.with_file_name(format!(
        ".{}.tokendaily-{}.tmp",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("file"),
        std::process::id()
    ));
    let permissions = fs::metadata(path).ok().map(|m| m.permissions());
    let result = (|| {
        let mut file = fs::File::create(&temp)?;
        std::io::Write::write_all(&mut file, bytes)?;
        file.sync_all()?;
        #[cfg(unix)]
        if private {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&temp, fs::Permissions::from_mode(0o600))?;
        }
        if let (false, Some(permissions)) = (private, permissions) {
            fs::set_permissions(&temp, permissions)?;
        }
        fs::rename(&temp, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

fn settings_path() -> PathBuf {
    crate::platform::claude_home().join("settings.json")
}

fn read_bridge(dir: &Path) -> Option<Bridge> {
    read_bounded(&dir.join(BRIDGE), 1 << 20)
        .ok()
        .flatten()
        .and_then(|b| serde_json::from_slice(&b).ok())
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BridgeStatus {
    supported: bool,
    enabled: bool,
    captured_at_ms: Option<i64>,
    message: Option<String>,
}

pub(crate) fn status(dir: &Path, message: Option<&str>) -> BridgeStatus {
    BridgeStatus {
        supported: cfg!(target_os = "macos"),
        enabled: read_bridge(dir).is_some(),
        captured_at_ms: load_capture(dir).map(|c| c.captured_at_ms),
        message: message.map(str::to_owned),
    }
}

pub(crate) fn load_capture(dir: &Path) -> Option<Capture> {
    read_bridge(dir)?;
    let bytes = read_bounded(&dir.join(CAPTURE), 1 << 20).ok()??;
    serde_json::from_slice(&bytes).ok()
}

pub(crate) fn enable(dir: &Path, executable: &Path) -> Result<(), &'static str> {
    if !cfg!(target_os = "macos") {
        return Err("当前平台暂不支持 Claude Code 状态栏桥接");
    }
    if read_bridge(dir).is_some() {
        return Ok(());
    }
    let path = settings_path();
    if !path.parent().is_some_and(Path::is_dir) {
        return Err("未找到 Claude Code 配置目录，请先在终端使用一次 Claude Code");
    }
    let existing =
        read_bounded(&path, MAX_SETTINGS).map_err(|_| "无法读取 Claude Code 设置文件，未做修改")?;
    let install = install(existing.as_deref(), &command_for(executable, dir))?;
    fs::create_dir_all(dir).map_err(|_| "无法创建 TokenDaily 数据目录")?;
    if let Some(existing) = &existing {
        replace_file(&dir.join(BACKUP), existing, true)
            .map_err(|_| "无法备份 Claude Code 设置文件，未做修改")?;
    }
    let bridge = serde_json::to_vec(&install.bridge).map_err(|_| "无法保存桥接记录")?;
    // The bridge record must exist before Claude Code can invoke the helper.
    if replace_file(&dir.join(BRIDGE), &bridge, true).is_err()
        || replace_file(&path, &install.file, false).is_err()
    {
        let _ = fs::remove_file(dir.join(BRIDGE));
        let _ = fs::remove_file(dir.join(BACKUP));
        return Err("无法写入 Claude Code 设置文件，已保持原样");
    }
    Ok(())
}

/// Restores the previous statusLine. Returns a notice when another program changed it.
pub(crate) fn disable(dir: &Path, clearing: bool) -> Result<Option<&'static str>, &'static str> {
    let Some(bridge) = read_bridge(dir) else {
        let _ = fs::remove_file(dir.join(CAPTURE));
        return Ok(None);
    };
    let path = settings_path();
    let current =
        read_bounded(&path, MAX_SETTINGS).map_err(|_| "无法读取 Claude Code 设置文件，未还原")?;
    let backup = read_bounded(&dir.join(BACKUP), MAX_SETTINGS).ok().flatten();
    let outcome = restore(current.as_deref(), &bridge, backup.as_deref())?;
    match &outcome {
        Restore::Write(bytes) => {
            replace_file(&path, bytes, false).map_err(|_| "无法还原 Claude Code 设置文件")?
        }
        Restore::Remove => match fs::remove_file(&path) {
            Err(error) if error.kind() != ErrorKind::NotFound => {
                return Err("无法还原 Claude Code 设置文件")
            }
            _ => {}
        },
        Restore::Changed => {}
    }
    let _ = fs::remove_file(dir.join(BRIDGE));
    let _ = fs::remove_file(dir.join(CAPTURE));
    // Keep the original backup when someone else changed statusLine, unless clearing data.
    if outcome != Restore::Changed || clearing {
        let _ = fs::remove_file(dir.join(BACKUP));
    }
    Ok((outcome == Restore::Changed).then_some(
        "Claude Code 的 statusLine 已被其他程序修改，TokenDaily 未覆盖；原始设置备份仍保留在 TokenDaily 数据目录",
    ))
}

fn previous_command(bridge: &Bridge) -> Option<String> {
    let value: Value = serde_json::from_str(bridge.previous.as_deref()?).ok()?;
    (value["type"].as_str() == Some("command"))
        .then(|| value["command"].as_str().map(str::to_owned))
        .flatten()
}

/// Entry point for `TokenDaily --claude-statusline <data dir>`; never fails the status line.
pub fn run_helper(args: &[String]) -> i32 {
    let Some(dir) = args.get(2).map(PathBuf::from) else {
        return 0;
    };
    let mut input = Vec::new();
    let _ = std::io::stdin().take(MAX_INPUT).read_to_end(&mut input);
    let Some(bridge) = read_bridge(&dir) else {
        return 0;
    };
    if let Some(mut capture) = parse_input(&input, crate::now_ms()) {
        capture.org = fs::read(crate::platform::claude_account_file())
            .ok()
            .and_then(|b| org_hash(&b));
        if let Ok(bytes) = serde_json::to_vec(&capture) {
            let _ = replace_file(&dir.join(CAPTURE), &bytes, true);
        }
    }
    previous_command(&bridge).map_or(0, |command| forward(&command, input))
}

#[cfg(unix)]
fn forward(command: &str, input: Vec<u8>) -> i32 {
    use std::process::{Command, Stdio};
    let Ok(mut child) = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .spawn()
    else {
        return 1;
    };
    // Write from a thread so a child that ignores stdin cannot deadlock the helper.
    let writer = child.stdin.take().map(|mut stdin| {
        std::thread::spawn(move || {
            let _ = std::io::Write::write_all(&mut stdin, &input);
        })
    });
    let code = child.wait().ok().and_then(|s| s.code()).unwrap_or(1);
    if let Some(writer) = writer {
        let _ = writer.join();
    }
    code
}

#[cfg(not(unix))]
fn forward(_command: &str, _input: Vec<u8>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const NOW: i64 = 1_800_000_000_000;
    const ORIGINAL: &str = "{\n  \"hooks\": {\n    \"SessionEnd\": [\n      {\n        \"hooks\": [\n          {\n            \"type\": \"command\",\n            \"command\": \"opentoken upload --v2 >/dev/null 2>&1 &\"\n          }\n        ]\n      }\n    ]\n  },\n  \"statusLine\": {\n    \"type\": \"command\",\n    \"command\": \"/Users/example/.local/bin/opentoken statusline\"\n  },\n  \"theme\": \"dark\"\n}\n";
    const CMD: &str =
        "'/Applications/TokenDaily.app/Contents/MacOS/TokenDaily' --claude-statusline '/data'";

    fn top_keys(bytes: &[u8]) -> Vec<String> {
        Ordered::parse(bytes)
            .unwrap()
            .0
            .into_iter()
            .map(|(k, _)| k)
            .collect()
    }

    #[test]
    fn install_keeps_other_keys_and_unchanged_file_restores_byte_for_byte() {
        let installed = install(Some(ORIGINAL.as_bytes()), CMD).unwrap();
        let text = String::from_utf8(installed.file.clone()).unwrap();
        assert_eq!(top_keys(&installed.file), ["hooks", "statusLine", "theme"]);
        assert!(
            text.contains("\"SessionEnd\": [\n      {"),
            "nested bytes kept"
        );
        let parsed: Value = serde_json::from_slice(&installed.file).unwrap();
        assert_eq!(parsed["statusLine"]["command"], CMD);
        assert_eq!(parsed["statusLine"]["type"], "command");
        assert_eq!(
            previous_command(&installed.bridge).unwrap(),
            "/Users/example/.local/bin/opentoken statusline"
        );
        let restored = restore(
            Some(&installed.file),
            &installed.bridge,
            Some(ORIGINAL.as_bytes()),
        );
        assert_eq!(
            restored.unwrap(),
            Restore::Write(ORIGINAL.as_bytes().to_vec())
        );
    }

    #[test]
    fn restore_after_unrelated_edit_puts_back_only_status_line() {
        let installed = install(Some(ORIGINAL.as_bytes()), CMD).unwrap();
        let mut edited: Value = serde_json::from_slice(&installed.file).unwrap();
        edited["model"] = json!("opus");
        let edited = serde_json::to_vec_pretty(&edited).unwrap();
        let Restore::Write(bytes) =
            restore(Some(&edited), &installed.bridge, Some(ORIGINAL.as_bytes())).unwrap()
        else {
            panic!("expected a write")
        };
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["model"], "opus");
        assert_eq!(
            value["statusLine"]["command"],
            "/Users/example/.local/bin/opentoken statusline"
        );
    }

    #[test]
    fn restore_never_overwrites_a_status_line_changed_by_someone_else() {
        let installed = install(Some(ORIGINAL.as_bytes()), CMD).unwrap();
        let mut edited: Value = serde_json::from_slice(&installed.file).unwrap();
        edited["statusLine"]["command"] = json!("other-tool statusline");
        let edited = serde_json::to_vec(&edited).unwrap();
        let outcome = restore(Some(&edited), &installed.bridge, Some(ORIGINAL.as_bytes()));
        assert_eq!(outcome.unwrap(), Restore::Changed);
        assert_eq!(
            restore(None, &installed.bridge, None).unwrap(),
            Restore::Changed
        );
    }

    #[test]
    fn missing_file_or_status_line_is_removed_again_on_restore() {
        let created = install(None, CMD).unwrap();
        assert!(!created.bridge.existed);
        assert_eq!(
            restore(Some(&created.file), &created.bridge, None).unwrap(),
            Restore::Remove
        );
        assert!(previous_command(&created.bridge).is_none());
        let without = b"{\n  \"theme\": \"dark\",\n  \"hooks\": {}\n}\n";
        let added = install(Some(without), CMD).unwrap();
        assert_eq!(top_keys(&added.file), ["theme", "hooks", "statusLine"]);
        let mut edited: Value = serde_json::from_slice(&added.file).unwrap();
        edited["model"] = json!("x");
        let edited = serde_json::to_vec(&edited).unwrap();
        let Restore::Write(bytes) = restore(Some(&edited), &added.bridge, Some(without)).unwrap()
        else {
            panic!("expected a write")
        };
        assert!(!top_keys(&bytes).contains(&"statusLine".to_string()));
    }

    #[test]
    fn invalid_settings_are_never_modified() {
        assert!(install(Some(b"not json"), CMD).is_err());
        assert!(install(Some(b"[1,2]"), CMD).is_err());
        assert!(install(Some(b"{\"statusLine\":\"weird\"}"), CMD).is_err());
        assert!(install(Some(b"{\"statusLine\":{},\"statusLine\":{}}"), CMD).is_err());
    }

    #[test]
    fn input_keeps_only_valid_rate_limit_windows() {
        let input = json!({"session_id":"s","cwd":"/secret","rate_limits":{
            "five_hour":{"used_percentage":29,"resets_at":1_800_003_600},
            "seven_day":{"used_percentage":0,"resets_at":1_800_500_000},
            "spend_limit":{"used_percentage":120,"resets_at":1_800_900_000}}});
        let capture = parse_input(input.to_string().as_bytes(), NOW).unwrap();
        assert_eq!(capture.captured_at_ms, NOW);
        assert_eq!(capture.windows.len(), 2);
        assert_eq!(capture.windows[0].resets_at_ms, 1_800_003_600_000);
        assert_eq!(capture.windows[1].used_percent, 0.0);
        let stored = serde_json::to_string(&capture).unwrap();
        assert!(!stored.contains("secret") && !stored.contains("session"));
        assert!(parse_input(br#"{"model":{}}"#, NOW).is_none());
        assert!(parse_input(
            br#"{"rate_limits":{"five_hour":{"used_percentage":101,"resets_at":1}}}"#,
            NOW
        )
        .is_none());
        assert!(parse_input(
            br#"{"rate_limits":{"five_hour":{"used_percentage":5}}}"#,
            NOW
        )
        .is_none());
        assert!(parse_input(b"garbage", NOW).is_none());
    }

    #[test]
    fn org_is_hashed_like_desktop_history() {
        let hash = org_hash(br#"{"oauthAccount":{"organizationUuid":"org-1"}}"#).unwrap();
        assert_eq!(hash, sha256_hex(b"org-1"));
        assert!(org_hash(br#"{"oauthAccount":{}}"#).is_none());
    }

    fn capture(at: i64, org: Option<&str>, five_used: f64, five_reset: i64) -> Capture {
        Capture {
            version: 1,
            captured_at_ms: at,
            org: org.map(|o| sha256_hex(o.as_bytes())),
            windows: vec![
                CaptureWindow {
                    key: "five_hour".into(),
                    used_percent: five_used,
                    resets_at_ms: five_reset,
                },
                CaptureWindow {
                    key: "seven_day".into(),
                    used_percent: 6.0,
                    resets_at_ms: NOW + 400_000_000,
                },
            ],
        }
    }
    fn desktop(at: i64, org: &str, five_used: f64) -> DesktopState {
        let raw = json!({"version":2,"samples":[{"t":at,"org":org,"u":{"fh":five_used,"sd":7}}]});
        crate::claude::parse_desktop_history(raw.to_string().as_bytes(), NOW)
            .unwrap()
            .0
    }

    #[test]
    fn newer_desktop_history_keeps_its_percentages_and_gains_reset_times() {
        let merged = merge(
            Some(&desktop(NOW - 60_000, "org-1", 27.0)),
            &capture(NOW - 600_000, Some("org-1"), 29.0, NOW + 3_000_000),
            NOW,
        )
        .unwrap();
        let windows = &merged.snapshot.as_ref().unwrap().windows;
        assert_eq!(windows[0].remaining_percent, Some(73.0));
        assert_eq!(windows[0].resets_at_ms, Some(NOW + 3_000_000));
        assert_eq!(windows[1].resets_at_ms, Some(NOW + 400_000_000));
        assert!(!merged.identity_verified);
    }

    #[test]
    fn newer_capture_replaces_percentages_and_marks_staleness() {
        let merged = merge(
            Some(&desktop(NOW - 1_200_000, "org-1", 27.0)),
            &capture(NOW - 60_000, Some("org-1"), 29.0, NOW + 3_000_000),
            NOW,
        )
        .unwrap();
        let snapshot = merged.snapshot.as_ref().unwrap();
        assert_eq!(snapshot.collected_at_ms, NOW - 60_000);
        assert_eq!(snapshot.windows[0].remaining_percent, Some(71.0));
        assert_eq!(merged.status, "available");
        let alone = merge(
            None,
            &capture(NOW - 700_000, Some("org-1"), 0.0, NOW + 1),
            NOW,
        )
        .unwrap();
        assert_eq!(alone.status, "stale");
        assert_eq!(
            alone.snapshot.unwrap().windows[0].remaining_percent,
            Some(100.0)
        );
    }

    #[test]
    fn other_accounts_expired_windows_and_old_captures_change_nothing() {
        let state = desktop(NOW - 60_000, "org-1", 27.0);
        assert!(merge(
            Some(&state),
            &capture(NOW, Some("org-2"), 1.0, NOW + 5),
            NOW
        )
        .is_none());
        assert!(merge(Some(&state), &capture(NOW, None, 1.0, NOW + 5), NOW).is_none());
        assert!(merge(
            None,
            &capture(NOW - 86_400_001, Some("org-1"), 1.0, NOW + 5),
            NOW
        )
        .is_none());
        let mut expired = capture(NOW - 1, Some("org-1"), 50.0, NOW - 1);
        expired.windows.truncate(1);
        assert!(
            merge(None, &expired, NOW).is_none(),
            "a passed reset is unknown, not 0 or 100"
        );
        let mut half = capture(NOW - 1, Some("org-1"), 50.0, NOW - 1);
        let alone = merge(None, &half, NOW).unwrap();
        assert_eq!(alone.snapshot.unwrap().windows.len(), 1);
        half.org = Some("x".into());
        assert!(merge(Some(&state), &half, NOW).is_none());
    }

    #[test]
    fn command_quotes_paths_for_the_shell() {
        let command = command_for(Path::new("/A b/it's"), Path::new("/d"));
        assert_eq!(command, r"'/A b/it'\''s' --claude-statusline '/d'");
    }
}

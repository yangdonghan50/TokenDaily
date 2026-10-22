//! WorkBuddy transcript usage only. Content fields are ignored and never retained.
use crate::{
    local_usage::{beijing_day, RankEvent},
    usage::{UsageDay, UsageSnapshot, UsageState},
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    time::SystemTime,
};
const MAX: u64 = 9_007_199_254_740_991;
const EMPTY: &str = "未发现 WorkBuddy 有效 Token 记录，今日用量未知";
#[derive(Deserialize)]
struct Row {
    #[serde(rename = "type")]
    kind: String,
    role: Option<String>,
    id: Option<String>,
    uuid: Option<String>,
    #[serde(rename = "messageId")]
    message_id: Option<String>,
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    timestamp: Option<i64>,
    #[serde(rename = "providerData")]
    provider: Option<Data>,
}
#[derive(Deserialize)]
struct Data {
    usage: Option<Counts>,
    #[serde(rename = "rawUsage")]
    raw: Option<RawCounts>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Counts {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    total_tokens: Option<u64>,
}
#[derive(Deserialize)]
struct RawCounts {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
}
#[derive(Clone)]
struct Parsed {
    key: [u8; 32],
    timestamp: i64,
    counts: [u64; 2],
}

/// Reads a bounded historical window only when the user opens total history.
/// It shares the live collector's record parser and deduplication rules, while
/// keeping the frequent live scan incremental.
pub(crate) fn history_events(
    root: PathBuf,
    since: i64,
    now: i64,
) -> Result<Vec<RankEvent>, &'static str> {
    let mut paths = BTreeSet::new();
    discover(&root.join("projects"), &mut paths, 0)?;
    let mut records = BTreeMap::<[u8; 32], Parsed>::new();
    for path in paths {
        let file = fs::File::open(&path).map_err(|_| "WorkBuddy 日志读取失败")?;
        let mut reader = BufReader::new(file);
        loop {
            let mut bytes = Vec::new();
            let size = reader
                .by_ref()
                .take(16 * 1024 * 1024)
                .read_until(b'\n', &mut bytes)
                .map_err(|_| "WorkBuddy 日志读取中断")?;
            if size == 0 {
                break;
            }
            if bytes.last() != Some(&b'\n') {
                if size >= 16 * 1024 * 1024 {
                    return Err("WorkBuddy 单条记录过大，统计不完整");
                }
                break;
            }
            if bytes.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let Some(record) = parse_record(&bytes)? else {
                continue;
            };
            if record.timestamp < since || record.timestamp > now {
                continue;
            }
            let saved = records.entry(record.key).or_insert_with(|| record.clone());
            saved.timestamp = saved.timestamp.max(record.timestamp);
            for (target, value) in saved.counts.iter_mut().zip(record.counts) {
                *target = (*target).max(value);
            }
        }
    }
    let mut events = Vec::with_capacity(records.len());
    for (key, record) in records {
        let tokens = record.counts.iter().try_fold(0u64, |total, value| {
            total
                .checked_add(*value)
                .filter(|sum| *sum <= MAX)
                .ok_or("WorkBuddy Token 合计超出范围")
        })?;
        if tokens > 0 {
            events.push(RankEvent {
                id: format!("{:x}", Sha256::digest(key)),
                at: record.timestamp,
                tokens,
                tool: "workbuddy".into(),
            });
        }
    }
    events.sort_by(|a, b| (a.at, &a.id).cmp(&(b.at, &b.id)));
    Ok(events)
}

pub(crate) fn history_days(
    root: PathBuf,
    since: i64,
    now: i64,
) -> Result<BTreeMap<String, u64>, &'static str> {
    let mut days = BTreeMap::new();
    for event in history_events(root, since, now)? {
        let total = days.entry(beijing_day(event.at)).or_insert(0u64);
        *total = total
            .checked_add(event.tokens)
            .filter(|sum| *sum <= MAX)
            .ok_or("WorkBuddy Token 合计超出范围")?;
    }
    Ok(days)
}
fn parse_record(bytes: &[u8]) -> Result<Option<Parsed>, &'static str> {
    let row: Row =
        serde_json::from_slice(bytes).map_err(|_| "WorkBuddy 日志记录格式异常，统计不完整")?;
    if !(row.kind == "function_call"
        || row.kind == "message" && row.role.as_deref() == Some("assistant"))
    {
        return Ok(None);
    }
    let Some(data) = row.provider else {
        return Ok(None);
    };
    let (input, output, total) = if let Some(u) = data.usage {
        (u.input_tokens, u.output_tokens, u.total_tokens)
    } else if let Some(u) = data.raw {
        (u.prompt_tokens, u.completion_tokens, u.total_tokens)
    } else {
        return Ok(None);
    };
    let (input, output) = (
        input.ok_or("WorkBuddy 输入 Token 字段缺失")?,
        output.ok_or("WorkBuddy 输出 Token 字段缺失")?,
    );
    let sum = input
        .checked_add(output)
        .filter(|n| *n <= MAX)
        .ok_or("WorkBuddy Token 数值超出范围")?;
    if total.is_some_and(|t| t != sum) {
        return Err("WorkBuddy Token 字段口径不一致，统计不完整");
    }
    let timestamp = row
        .timestamp
        .filter(|t| *t > 0 && chrono::DateTime::from_timestamp_millis(*t).is_some())
        .ok_or("WorkBuddy 用量时间无效")?;
    let session = row
        .session_id
        .filter(|s| !s.is_empty())
        .ok_or("WorkBuddy 用量缺少会话标识")?;
    let id = row
        .uuid
        .or(row.message_id)
        .or(row.id)
        .filter(|s| !s.is_empty())
        .ok_or("WorkBuddy 用量缺少记录标识")?;
    Ok(Some(Parsed {
        key: Sha256::digest(serde_json::to_vec(&(session, id)).unwrap()).into(),
        timestamp,
        counts: [input, output],
    }))
}
#[derive(Default)]
struct Cursor {
    offset: u64,
    length: u64,
    modified: Option<SystemTime>,
    observed: i64,
    has_usage: bool,
    records: BTreeMap<[u8; 32], [u64; 2]>,
}
pub(crate) struct Collector {
    root: PathBuf,
    day: String,
    discovered: i64,
    files: BTreeMap<PathBuf, Cursor>,
    previous: UsageState,
    seen_usage: bool,
    dirty: bool,
    total: u64,
    #[cfg(test)]
    read_bytes: u64,
}
impl Collector {
    pub(crate) fn new(root: PathBuf) -> Self {
        Self {
            root,
            day: String::new(),
            discovered: 0,
            files: BTreeMap::new(),
            previous: UsageState::default(),
            seen_usage: false,
            dirty: true,
            total: 0,
            #[cfg(test)]
            read_bytes: 0,
        }
    }
    pub(crate) fn poll(&mut self, now: i64, force: bool) -> UsageState {
        let day = beijing_day(now);
        if self.day != day {
            self.day = day;
            self.files.clear();
            self.discovered = 0;
            self.previous = UsageState::default();
            self.total = 0;
            self.dirty = true;
        }
        let result = self.collect(now, force);
        if result.is_ok() {
            self.seen_usage = true;
        }
        self.previous = match result {
            Ok(tokens) => UsageState::available(UsageSnapshot {
                api_cost: None,
                identity_key: format!(
                    "workbuddy-local:{:x}",
                    Sha256::digest(self.root.to_string_lossy().as_bytes())
                ),
                collected_at_ms: now,
                days: vec![UsageDay {
                    date: self.day.clone(),
                    tokens,
                }],
            }),
            Err(EMPTY) => UsageState {
                source_unavailable: !self.seen_usage,
                message: Some(EMPTY.into()),
                ..UsageState::default()
            },
            Err(message) => {
                let mut state = UsageState::with_error(&self.previous, message);
                state.source_unavailable = !self.seen_usage
                    && matches!(self.root.join("projects").try_exists(), Ok(false));
                state
            }
        };
        self.previous.clone()
    }
    fn collect(&mut self, now: i64, force: bool) -> Result<u64, &'static str> {
        let reconcile = force || self.discovered == 0 || now - self.discovered >= 30_000;
        if reconcile {
            let mut paths = BTreeSet::new();
            discover(&self.root.join("projects"), &mut paths, 0)?;
            let before = self.files.len();
            self.files.retain(|p, _| paths.contains(p));
            self.dirty |= before != self.files.len();
            for path in paths {
                self.files.entry(path).or_default();
            }
            self.discovered = now;
        }
        for (path, c) in &mut self.files {
            if !reconcile && now - c.observed > 60_000 {
                continue;
            }
            let meta = fs::metadata(path).map_err(|_| "WorkBuddy 日志状态读取失败")?;
            let modified = meta.modified().ok();
            if meta.len() == c.length && modified == c.modified {
                continue;
            }
            self.dirty = true;
            if meta.len() < c.length || meta.len() == c.length && modified != c.modified {
                *c = Cursor::default();
            }
            let mut file = fs::File::open(path).map_err(|_| "WorkBuddy 日志读取失败")?;
            file.seek(SeekFrom::Start(c.offset))
                .map_err(|_| "WorkBuddy 日志定位失败")?;
            let mut reader = BufReader::new(file);
            loop {
                let mut bytes = Vec::new();
                let size = reader
                    .by_ref()
                    .take(16 * 1024 * 1024)
                    .read_until(b'\n', &mut bytes)
                    .map_err(|_| "WorkBuddy 日志读取中断")?;
                #[cfg(test)]
                {
                    self.read_bytes += size as u64;
                }
                if size == 0 {
                    break;
                }
                if bytes.last() != Some(&b'\n') {
                    if size >= 16 * 1024 * 1024 {
                        return Err("WorkBuddy 单条记录过大，统计不完整");
                    }
                    break;
                }
                if !bytes.iter().all(u8::is_ascii_whitespace) {
                    if let Some(record) = parse_record(&bytes)? {
                        c.has_usage = true;
                        if beijing_day(record.timestamp) == self.day {
                            let saved = c.records.entry(record.key).or_default();
                            for (target, value) in saved.iter_mut().zip(record.counts) {
                                *target = (*target).max(value);
                            }
                        }
                    }
                }
                c.offset += size as u64;
            }
            c.length = meta.len();
            c.modified = modified;
            c.observed = now;
        }
        if !self.files.values().any(|c| c.has_usage) {
            return Err(EMPTY);
        }
        if !self.dirty {
            return Ok(self.total);
        }
        let mut unique = BTreeMap::<[u8; 32], [u64; 2]>::new();
        for c in self.files.values() {
            for (id, counts) in &c.records {
                let saved = unique.entry(*id).or_default();
                for i in 0..2 {
                    saved[i] = saved[i].max(counts[i]);
                }
            }
        }
        self.total = unique.values().flatten().try_fold(0u64, |s, n| {
            s.checked_add(*n)
                .filter(|v| *v <= MAX)
                .ok_or("WorkBuddy Token 合计超出范围")
        })?;
        self.dirty = false;
        Ok(self.total)
    }
}
fn discover(dir: &Path, paths: &mut BTreeSet<PathBuf>, depth: usize) -> Result<(), &'static str> {
    if depth > 32 {
        return Err("WorkBuddy 日志目录层级异常");
    }
    for entry in
        fs::read_dir(dir).map_err(|_| "无法读取 WorkBuddy 日志目录，请确认本机已使用 WorkBuddy")?
    {
        let entry = entry.map_err(|_| "WorkBuddy 日志目录读取不完整")?;
        let kind = entry.file_type().map_err(|_| "WorkBuddy 文件类型未知")?;
        if kind.is_dir() {
            discover(&entry.path(), paths, depth + 1)?;
        } else if kind.is_file() && entry.path().extension().is_some_and(|e| e == "jsonl") {
            paths.insert(entry.path());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::{fs, io::Write, path::PathBuf};
    const NOW: i64 = 1792195200000;
    #[test]
    #[ignore = "read-only check of real WorkBuddy logs; no content or paths printed"]
    fn live_workbuddy_logs() {
        let mut collector = Collector::new(crate::platform::workbuddy_home());
        let started = std::time::Instant::now();
        let state = collector.poll(crate::now_ms(), true);
        assert_eq!(
            state.status,
            "available",
            "{}",
            state.message.unwrap_or_default()
        );
        let initial = started.elapsed();
        let bytes = collector.read_bytes;
        let started = std::time::Instant::now();
        let next = collector.poll(crate::now_ms(), false);
        println!(
            "files={} initial={initial:?} incremental={:?} reread_bytes={} today_tokens={}",
            collector.files.len(),
            started.elapsed(),
            collector.read_bytes - bytes,
            total(&next)
        );
        assert_eq!(next.status, "available");
    }
    fn fixture() -> (PathBuf, Collector) {
        let root = std::env::temp_dir().join(format!(
            "tokendaily-wb-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("projects/p")).unwrap();
        let collector = Collector::new(root.clone());
        (root, collector)
    }
    fn record(id: &str, time: i64, input: u64) -> String {
        format!(
            "{}\n",
            json!({"type":"function_call","id":id,"sessionId":"s","timestamp":time,"providerData":{"traceId":"shared-trace","usage":{"inputTokens":input,"outputTokens":20,"totalTokens":input+20,"inputTokensDetails":[{"cached_tokens":80}],"outputTokensDetails":[{"reasoning_tokens":10}]},"rawUsage":{"prompt_tokens":input,"completion_tokens":20,"credit":0.5}}})
        )
    }
    fn total(state: &crate::usage::UsageState) -> u64 {
        state.snapshot.as_ref().unwrap().days[0].tokens
    }
    #[test]
    fn incremental_dedup_tools_and_cache_semantics() {
        let (root, mut c) = fixture();
        let path = root.join("projects/p/a.jsonl");
        let a = record("a", NOW, 100);
        fs::write(&path, format!("{a}{a}{}", record("b", NOW, 200))).unwrap();
        assert_eq!(total(&c.poll(NOW, false)), 340);
        let bytes = c.read_bytes;
        assert_eq!(total(&c.poll(NOW + 2000, false)), 340);
        assert_eq!(c.read_bytes, bytes, "unchanged logs must not be reread");
        fs::write(root.join("projects/p/copy.jsonl"), &a).unwrap();
        assert_eq!(
            total(&c.poll(NOW + 3000, true)),
            340,
            "copied records must not double count"
        );
        let tail = record("c", NOW, 300);
        let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
        f.write_all(tail[..tail.len() - 1].as_bytes()).unwrap();
        assert_eq!(total(&c.poll(NOW + 4000, false)), 340);
        f.write_all(b"\n").unwrap();
        assert_eq!(total(&c.poll(NOW + 6000, false)), 660);
        drop(f);
        fs::write(&path, record("d", NOW, 400)).unwrap();
        assert_eq!(
            total(&c.poll(NOW + 8000, false)),
            540,
            "truncate replaces old cursor"
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn beijing_rollover_unknown_and_malformed_are_distinct() {
        let (root, mut c) = fixture();
        let absent = c.poll(NOW, false);
        assert!(absent.snapshot.is_none());
        assert!(absent.source_unavailable);
        let midnight = 1792166400000; // 2026-10-17 00:00 Beijing
        fs::write(
            root.join("projects/p/a.jsonl"),
            record("a", midnight - 1, 100),
        )
        .unwrap();
        assert_eq!(total(&c.poll(NOW, true)), 0);
        fs::write(root.join("projects/p/b.jsonl"), record("b", midnight, 200)).unwrap();
        assert_eq!(total(&c.poll(NOW, true)), 220);
        assert_eq!(total(&c.poll(midnight + 86_400_000, true)), 0);
        fs::write(root.join("projects/p/b.jsonl"), "{bad}\n").unwrap();
        let bad = c.poll(midnight + 86_402_000, true);
        assert_eq!(bad.status, "stale");
        assert!(!bad.source_unavailable);
        fs::remove_dir_all(root.join("projects")).unwrap();
        let missing = c.poll(midnight + 172_800_000, true);
        assert!(missing.snapshot.is_none());
        assert!(
            !missing.source_unavailable,
            "a previously observed source stays in scope after deletion"
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn missing_negative_inconsistent_and_overflow_counts_are_rejected() {
        let (root, mut c) = fixture();
        for usage in [
            json!({"inputTokens":100}),
            json!({"inputTokens":-1,"outputTokens":2}),
            json!({"inputTokens":1,"outputTokens":2,"totalTokens":99}),
            json!({"inputTokens":9007199254740991u64,"outputTokens":1}),
        ] {
            fs::write(root.join("projects/p/a.jsonl"),format!("{}\n",json!({"type":"message","role":"assistant","id":"a","sessionId":"s","timestamp":NOW,"providerData":{"usage":usage}}))).unwrap();
            assert!(c.poll(NOW, true).snapshot.is_none());
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn raw_usage_fallback_does_not_add_cache_or_reasoning_twice() {
        let line = json!({"type":"message","role":"assistant","id":"a","sessionId":"s","timestamp":NOW,"providerData":{"rawUsage":{"prompt_tokens":100,"completion_tokens":20,"total_tokens":120,"cache_read_input_tokens":80,"completion_thinking_tokens":10}}});
        assert_eq!(
            parse_record(line.to_string().as_bytes())
                .unwrap()
                .unwrap()
                .counts,
            [100, 20]
        );
    }
    #[test]
    fn history_keeps_beijing_days_and_deduplicates_copied_records() {
        let (root, _) = fixture();
        let midnight = 1792166400000; // 2026-10-17 00:00 Beijing
        let previous_day = record("old", midnight - 1, 100);
        let current_day = record("current", midnight, 100);
        let updated_current_day = record("current", midnight + 1, 200);
        fs::write(
            root.join("projects/p/a.jsonl"),
            format!("{previous_day}{current_day}{updated_current_day}"),
        )
        .unwrap();
        fs::write(root.join("projects/p/copy.jsonl"), current_day).unwrap();
        let events = history_events(root.clone(), NOW - 3 * 86_400_000, NOW).unwrap();
        assert_eq!(events.len(), 2);
        assert!(events
            .iter()
            .all(|event| event.tool == "workbuddy" && event.id.len() == 64));
        assert_eq!(events.iter().map(|event| event.tokens).sum::<u64>(), 340);
        let days = history_days(root.clone(), NOW - 3 * 86_400_000, NOW).unwrap();
        assert_eq!(days.get("2026-10-16"), Some(&120));
        assert_eq!(days.get("2026-10-17"), Some(&220));
        assert_eq!(days.len(), 2);
        fs::remove_dir_all(root).unwrap();
    }
}

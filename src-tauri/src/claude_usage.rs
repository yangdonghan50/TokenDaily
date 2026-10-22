//! Incremental, read-only Claude Code transcript accounting. No conversation fields retained.
use crate::local_usage::{beijing_day, RankEvent};
use crate::usage::{UsageDay, UsageSnapshot, UsageState};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    time::SystemTime,
};

const NO_USAGE_RECORDS: &str =
    "未发现 Claude Code 用量记录；网页 / 桌面聊天尚未接入，不能据此判断今日用量为 0";

#[derive(Default)]
struct Cursor {
    offset: u64,
    length: u64,
    modified: Option<SystemTime>,
    observed_at_ms: i64,
    has_usage: bool,
    records: BTreeMap<[u8; 32], CostRecord>,
}
#[derive(Clone)]
struct CostRecord {
    counts: [u64; 4],
    model: Option<String>,
    writes: Option<[u64; 2]>,
}
impl CostRecord {
    fn merge(&mut self, other: &Self) {
        for i in 0..4 {
            self.counts[i] = self.counts[i].max(other.counts[i]);
        }
        if self.model != other.model {
            self.model = None;
        }
        self.writes = match (self.writes, other.writes) {
            (Some(a), Some(b)) => Some([a[0].max(b[0]), a[1].max(b[1])]),
            _ => None,
        };
    }
}
#[derive(Deserialize)]
struct Record {
    timestamp: Option<String>,
    #[serde(rename = "requestId")]
    request_id: Option<String>,
    message: Option<Message>,
}
#[derive(Deserialize)]
struct Envelope {
    #[serde(rename = "type")]
    kind: String,
}
#[derive(Deserialize)]
struct Message {
    model: Option<serde_json::Value>,
    id: Option<String>,
    usage: Option<Counts>,
}
#[derive(Deserialize)]
struct Counts {
    cache_creation: Option<serde_json::Value>,
    input_tokens: u64,
    output_tokens: u64,
    #[serde(default)]
    cache_read_input_tokens: u64,
    #[serde(default)]
    cache_creation_input_tokens: u64,
}

/// Reads a bounded historical window on demand. This intentionally does not
/// share the hot collector: opening the history view must not add work to the
/// two-second live refresh loop.
pub(crate) fn history_events(
    root: PathBuf,
    since: i64,
    now: i64,
) -> Result<Vec<RankEvent>, &'static str> {
    let projects = root.join("projects");
    if !projects.is_dir() {
        return Err("未找到 Claude Code 日志目录，请先在本机使用 Claude Code");
    }
    let mut paths = Vec::new();
    discover(&projects, &mut paths)?;
    let mut records = BTreeMap::<[u8; 32], (i64, [u64; 4])>::new();
    for path in paths {
        let file = fs::File::open(&path).map_err(|_| "无法读取 Claude Code 日志")?;
        let mut reader = BufReader::new(file);
        loop {
            let mut bytes = Vec::new();
            let size = reader
                .by_ref()
                .take(16 * 1024 * 1024)
                .read_until(b'\n', &mut bytes)
                .map_err(|_| "Claude Code 日志读取失败")?;
            if size == 0 {
                break;
            }
            if bytes.last() != Some(&b'\n') {
                if size >= 16 * 1024 * 1024 {
                    return Err("Claude Code 日志单条记录过大，统计不完整");
                }
                break;
            }
            if bytes.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let envelope: Envelope = serde_json::from_slice(&bytes)
                .map_err(|_| "Claude Code 日志格式异常，统计不完整")?;
            if envelope.kind != "assistant" {
                continue;
            }
            let record: Record = serde_json::from_slice(&bytes)
                .map_err(|_| "Claude Code Token 记录格式异常，统计不完整")?;
            let Some(message) = record.message else {
                continue;
            };
            let Some(usage) = message.usage else { continue };
            let timestamp = record
                .timestamp
                .as_deref()
                .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
                .ok_or("Claude Code 用量时间缺失或无效")?
                .timestamp_millis();
            if timestamp < since || timestamp > now {
                continue;
            }
            let id = message
                .id
                .filter(|value| !value.is_empty())
                .ok_or("Claude Code 用量缺少消息标识")?;
            let key: [u8; 32] =
                Sha256::digest(serde_json::to_vec(&(record.request_id, id)).unwrap()).into();
            let counts = [
                usage.input_tokens,
                usage.output_tokens,
                usage.cache_read_input_tokens,
                usage.cache_creation_input_tokens,
            ];
            if counts.iter().any(|value| *value > 9_007_199_254_740_991) {
                return Err("Claude Code Token 数值无效");
            }
            let saved = records.entry(key).or_insert((timestamp, [0; 4]));
            saved.0 = saved.0.max(timestamp);
            for (target, value) in saved.1.iter_mut().zip(counts) {
                *target = (*target).max(value);
            }
        }
    }
    let mut events = Vec::with_capacity(records.len());
    for (key, (timestamp, counts)) in records {
        let tokens = counts.iter().try_fold(0u64, |total, value| {
            total
                .checked_add(*value)
                .filter(|sum| *sum <= 9_007_199_254_740_991)
                .ok_or("Claude Code Token 总量超出可表示范围")
        })?;
        if tokens > 0 {
            events.push(RankEvent {
                id: format!("{:x}", Sha256::digest(key)),
                at: timestamp,
                tokens,
                tool: "claude".into(),
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
            .filter(|sum| *sum <= 9_007_199_254_740_991)
            .ok_or("Claude Code Token 总量超出可表示范围")?;
    }
    Ok(days)
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
    cost: crate::api_cost::ApiCost,
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
            cost: Default::default(),
            #[cfg(test)]
            read_bytes: 0,
        }
    }
    pub(crate) fn poll(&mut self, now: i64) -> UsageState {
        let day = beijing_day(now);
        if self.day != day {
            self.day = day;
            self.files.clear();
            self.discovered = 0;
            self.previous = UsageState::default();
            self.dirty = true;
            self.total = 0;
            self.cost = Default::default();
        }
        let result = self.collect(now);
        if result.is_ok() {
            self.seen_usage = true;
        }
        self.previous = match result {
            Ok(tokens) => UsageState::available(UsageSnapshot {
                api_cost: Some(self.cost.clone()),
                identity_key: format!(
                    "claude-local:{:x}",
                    Sha256::digest(self.root.to_string_lossy().as_bytes())
                ),
                collected_at_ms: now,
                days: vec![UsageDay {
                    date: self.day.clone(),
                    tokens,
                }],
            }),
            Err(NO_USAGE_RECORDS) => UsageState {
                source_unavailable: !self.seen_usage,
                message: Some(NO_USAGE_RECORDS.into()),
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
    fn collect(&mut self, now: i64) -> Result<u64, &'static str> {
        if !self.root.join("projects").is_dir() {
            return Err("未找到 Claude Code 日志目录，请先在本机使用 Claude Code");
        }
        let reconcile = self.discovered == 0 || now - self.discovered >= 30_000;
        if reconcile {
            let mut paths = Vec::new();
            discover(&self.root.join("projects"), &mut paths)?;
            let paths: BTreeSet<_> = paths.into_iter().collect();
            let previous_count = self.files.len();
            self.files.retain(|path, _| paths.contains(path));
            self.dirty |= previous_count != self.files.len();
            for path in paths {
                self.files.entry(path).or_default();
            }
            self.discovered = now;
        }
        for (path, cursor) in &mut self.files {
            // Old inactive files are checked during reconciliation, not on every hot poll.
            if !reconcile && now - cursor.observed_at_ms > 60_000 {
                continue;
            }
            let metadata = fs::metadata(path).map_err(|_| "无法读取 Claude Code 日志状态")?;
            let modified = metadata.modified().ok();
            if metadata.len() == cursor.length && modified == cursor.modified {
                continue;
            }
            self.dirty = true;
            if metadata.len() < cursor.length
                || metadata.len() == cursor.length && modified != cursor.modified
            {
                *cursor = Cursor::default();
            }
            let mut file = fs::File::open(path).map_err(|_| "无法读取 Claude Code 日志")?;
            file.seek(SeekFrom::Start(cursor.offset))
                .map_err(|_| "无法定位 Claude Code 日志")?;
            let mut reader = BufReader::new(file);
            loop {
                let mut bytes = Vec::new();
                let size = reader
                    .by_ref()
                    .take(16 * 1024 * 1024)
                    .read_until(b'\n', &mut bytes)
                    .map_err(|_| "Claude Code 日志读取失败")?;
                #[cfg(test)]
                {
                    self.read_bytes += size as u64;
                }
                if size == 0 {
                    break;
                }
                if bytes.last() != Some(&b'\n') {
                    if size >= 16 * 1024 * 1024 {
                        return Err("Claude Code 日志单条记录过大，统计不完整");
                    }
                    break; // Only commit complete records; retry the partial tail after append.
                }
                if bytes.iter().all(u8::is_ascii_whitespace) {
                    cursor.offset += size as u64;
                    continue;
                }
                let envelope: Envelope = serde_json::from_slice(&bytes)
                    .map_err(|_| "Claude Code 日志格式异常，统计不完整")?;
                if envelope.kind == "assistant" {
                    let record: Record = serde_json::from_slice(&bytes)
                        .map_err(|_| "Claude Code Token 记录格式异常，统计不完整")?;
                    if let Some(message) = record.message {
                        if let Some(usage) = message.usage {
                            let timestamp = record
                                .timestamp
                                .as_deref()
                                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                                .ok_or("Claude Code 用量时间缺失或无效")?;
                            let id = message
                                .id
                                .filter(|s| !s.is_empty())
                                .ok_or("Claude Code 用量缺少消息标识")?;
                            let key: [u8; 32] = Sha256::digest(
                                serde_json::to_vec(&(record.request_id, id)).unwrap(),
                            )
                            .into();
                            let counts = [
                                usage.input_tokens,
                                usage.output_tokens,
                                usage.cache_read_input_tokens,
                                usage.cache_creation_input_tokens,
                            ];
                            if counts.iter().any(|v| *v > 9_007_199_254_740_991) {
                                return Err("Claude Code Token 数值无效");
                            }
                            cursor.has_usage = true;
                            if beijing_day(timestamp.timestamp_millis()) == self.day {
                                let writes = usage.cache_creation.as_ref().and_then(|w| {
                                    Some([
                                        w["ephemeral_5m_input_tokens"].as_u64()?,
                                        w["ephemeral_1h_input_tokens"].as_u64()?,
                                    ])
                                });
                                let record = CostRecord {
                                    counts,
                                    model: message.model.and_then(|v| {
                                        v.as_str().filter(|s| s.len() <= 128).map(str::to_owned)
                                    }),
                                    writes,
                                };
                                cursor
                                    .records
                                    .entry(key)
                                    .and_modify(|saved| saved.merge(&record))
                                    .or_insert(record);
                            }
                        }
                    }
                }
                cursor.offset += size as u64;
            }
            cursor.length = metadata.len();
            cursor.modified = modified;
            cursor.observed_at_ms = now;
        }
        if !self.files.values().any(|cursor| cursor.has_usage) {
            return Err(NO_USAGE_RECORDS);
        }
        if !self.dirty {
            return Ok(self.total);
        }
        let mut unique = BTreeMap::<[u8; 32], CostRecord>::new();
        for cursor in self.files.values() {
            for (key, counts) in &cursor.records {
                unique
                    .entry(*key)
                    .and_modify(|saved| saved.merge(counts))
                    .or_insert_with(|| counts.clone());
            }
        }
        self.total = unique
            .values()
            .flat_map(|record| record.counts.iter())
            .try_fold(0u64, |sum, count| {
                sum.checked_add(*count)
                    .filter(|v| *v <= 9_007_199_254_740_991)
                    .ok_or("Claude Code Token 总量超出可表示范围")
            })?;
        self.cost = Default::default();
        for record in unique.values() {
            self.cost.add(crate::api_cost::claude(
                record.model.as_deref(),
                record.counts,
                record.writes,
            ));
        }
        self.dirty = false;
        Ok(self.total)
    }
}
fn discover(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), &'static str> {
    for entry in fs::read_dir(directory).map_err(|_| "无法扫描 Claude Code 日志目录")? {
        let entry = entry.map_err(|_| "Claude Code 日志目录读取不完整")?;
        let kind = entry.file_type().map_err(|_| "Claude Code 日志类型未知")?;
        if kind.is_dir() {
            discover(&entry.path(), paths)?;
        } else if kind.is_file() && entry.path().extension().is_some_and(|ext| ext == "jsonl") {
            paths.push(entry.path());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    const NOW: i64 = 1792195200000; // 2026-10-17 00:00 UTC
    #[test]
    fn cost_follows_stream_dedup_and_unknown_models() {
        let (root, mut c) = fixture();
        let mut row: serde_json::Value =
            serde_json::from_str(&record("cost", "2026-10-17T00:00:00Z", 5)).unwrap();
        row["message"]["model"] = json!("claude-opus-5-5");
        row["message"]["usage"]["cache_creation"] =
            json!({"ephemeral_5m_input_tokens":20,"ephemeral_1h_input_tokens":0});
        let path = root.join("projects/p/cost.jsonl");
        fs::write(&path, format!("{row}\n{row}\n")).unwrap();
        let s = c.poll(NOW).snapshot.unwrap();
        assert_eq!(s.days[0].tokens, 135);
        let cost = s.api_cost.unwrap();
        assert_eq!(cost.priced_tokens, 135);
        assert_eq!(cost.unpriced_tokens, 0);
        assert!((cost.usd - 0.00026).abs() < 1e-12);
        row["message"]["usage"]["output_tokens"] = json!(15);
        fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap()
            .write_all(format!("{row}\n").as_bytes())
            .unwrap();
        let cost = c.poll(NOW + 2000).snapshot.unwrap().api_cost.unwrap();
        assert_eq!(cost.priced_tokens, 145);
        assert!((cost.usd - 0.00046).abs() < 1e-12);
        assert_eq!(
            c.poll(NOW + 86400000)
                .snapshot
                .unwrap()
                .api_cost
                .unwrap()
                .usd,
            0.
        );
        fs::remove_dir_all(root).unwrap();
    }
    fn fixture() -> (PathBuf, Collector) {
        // Windows clock resolution can give concurrent tests the same timestamp.
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "tokendaily-claude-{}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("projects/p/subagents")).unwrap();
        let collector = Collector::new(root.clone());
        (root, collector)
    }
    fn record(id: &str, time: &str, output: u64) -> String {
        json!({"type":"assistant","timestamp":time,"requestId":"r","message":{"id":id,"usage":{"input_tokens":10,"output_tokens":output,"cache_read_input_tokens":100,"cache_creation_input_tokens":20}}}).to_string()+"\n"
    }
    fn total(s: &UsageState) -> u64 {
        s.snapshot.as_ref().unwrap().days[0].tokens
    }
    #[test]
    fn incremental_dedup_stream_updates_subagents_and_cache() {
        let (root, mut c) = fixture();
        let path = root.join("projects/p/main.jsonl");
        let line = record("m1", "2026-10-17T00:00:00Z", 5);
        fs::write(&path, &line).unwrap();
        fs::write(root.join("projects/p/copy.jsonl"), &line).unwrap();
        fs::write(
            root.join("projects/p/subagents/sub.jsonl"),
            record("m2", "2026-10-17T00:00:00Z", 7),
        )
        .unwrap();
        assert_eq!(total(&c.poll(NOW)), 272);
        let bytes = c.read_bytes;
        assert_eq!(total(&c.poll(NOW + 2000)), 272);
        assert_eq!(c.read_bytes, bytes, "unchanged files must not be reread");
        let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
        file.write_all(record("m1", "2026-10-17T00:00:01Z", 15).as_bytes())
            .unwrap();
        assert_eq!(
            total(&c.poll(NOW + 4000)),
            282,
            "stream chunks update one response, not add twice"
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn beijing_midnight_partial_line_and_truncation() {
        let (root, mut c) = fixture();
        let path = root.join("projects/p/main.jsonl");
        fs::write(&path, record("old", "2026-10-16T15:59:59Z", 5)).unwrap();
        assert_eq!(total(&c.poll(NOW)), 0);
        let line = record("new", "2026-10-16T16:00:00Z", 5);
        let split = line.len() / 2;
        let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
        f.write_all(&line.as_bytes()[..split]).unwrap();
        assert_eq!(total(&c.poll(NOW + 2000)), 0);
        f.write_all(&line.as_bytes()[split..]).unwrap();
        assert_eq!(total(&c.poll(NOW + 4000)), 135);
        fs::write(&path, record("replacement", "2026-10-17T00:00:00Z", 8)).unwrap();
        assert_eq!(total(&c.poll(NOW + 6000)), 138);
        assert_eq!(total(&c.poll(NOW + 86400000)), 0);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn history_keeps_beijing_days_and_deduplicates_copied_records() {
        let (root, _) = fixture();
        let previous_day = record("old", "2026-10-16T15:59:59Z", 5);
        let current_day = record("current", "2026-10-16T16:00:00Z", 5);
        let updated_current_day = record("current", "2026-10-16T16:00:01Z", 8);
        fs::write(
            root.join("projects/p/main.jsonl"),
            format!("{previous_day}{current_day}{updated_current_day}"),
        )
        .unwrap();
        fs::write(root.join("projects/p/copy.jsonl"), current_day).unwrap();
        let events = history_events(root.clone(), NOW - 3 * 86_400_000, NOW).unwrap();
        assert_eq!(events.len(), 2);
        assert!(events
            .iter()
            .all(|event| event.tool == "claude" && event.id.len() == 64));
        assert_eq!(events.iter().map(|event| event.tokens).sum::<u64>(), 273);
        let days = history_days(root.clone(), NOW - 3 * 86_400_000, NOW).unwrap();
        assert_eq!(days.get("2026-10-16"), Some(&135));
        assert_eq!(days.get("2026-10-17"), Some(&138));
        assert_eq!(days.len(), 2);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn missing_and_corrupt_logs_are_not_zero() {
        let (root, mut c) = fixture();
        let empty = c.poll(NOW);
        assert_eq!(empty.status, "unknown");
        assert!(empty.source_unavailable);
        assert!(empty.snapshot.is_none());
        fs::write(root.join("projects/p/bad.jsonl"), "{invalid}\n").unwrap();
        let corrupt = c.poll(NOW + 31000);
        assert_ne!(corrupt.status, "available");
        assert!(
            !corrupt.source_unavailable,
            "corrupt logs must remain an error"
        );
        fs::remove_dir_all(&root).unwrap();
        let absent = Collector::new(root).poll(NOW);
        assert_eq!(absent.status, "error");
        assert!(absent.source_unavailable);
    }
    #[test]
    fn sessions_without_token_records_are_unknown_until_usage_is_observed() {
        let (root, mut c) = fixture();
        let path = root.join("projects/p/main.jsonl");
        fs::write(
            &path,
            "{\"type\":\"user\",\"message\":{\"role\":\"user\"}}\n",
        )
        .unwrap();
        assert!(c.poll(NOW).snapshot.is_none());
        let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
        file.write_all(record("known", "2026-10-16T15:59:59Z", 5).as_bytes())
            .unwrap();
        assert_eq!(
            total(&c.poll(NOW + 2000)),
            0,
            "verified historical source can report a healthy new-day zero"
        );
        file.write_all(record("today", "2026-10-17T00:00:00Z", 5).as_bytes())
            .unwrap();
        assert_eq!(total(&c.poll(NOW + 4000)), 135);
        fs::remove_file(&path).unwrap();
        assert!(
            c.poll(NOW + 31000).snapshot.is_none(),
            "removing all source logs must not invent zero"
        );
        assert!(!c.poll(NOW + 32000).source_unavailable);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn invalid_counts_keep_a_stale_subtotal_and_never_invent_today_zero() {
        let (root, mut c) = fixture();
        let path = root.join("projects/p/main.jsonl");
        fs::write(&path, record("m", "2026-10-17T00:00:00Z", 5)).unwrap();
        assert_eq!(total(&c.poll(NOW)), 135);
        fs::OpenOptions::new().append(true).open(&path).unwrap().write_all(b"{\"type\":\"assistant\",\"message\":{\"id\":\"bad\",\"usage\":{\"input_tokens\":-1,\"output_tokens\":1}}}\n").unwrap();
        let bad = c.poll(NOW + 2000);
        assert_eq!(bad.status, "stale");
        assert_eq!(total(&bad), 135);
        assert_eq!(c.poll(NOW + 86400000).status, "error");
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    #[ignore = "read-only check of the current user's real Claude Code logs"]
    fn live_claude_code_logs() {
        let mut collector = Collector::new(crate::platform::claude_home());
        let start = std::time::Instant::now();
        let first = collector.poll(crate::now_ms());
        println!(
            "first_scan_ms={} status={} day={:?} tokens={:?} files={}",
            start.elapsed().as_millis(),
            first.status,
            first.snapshot.as_ref().map(|s| s.days[0].date.clone()),
            first.snapshot.as_ref().map(|s| s.days[0].tokens),
            collector.files.len()
        );
        assert_eq!(first.status, "available");
        assert!(!first.source_unavailable);
        let start = std::time::Instant::now();
        let second = collector.poll(crate::now_ms());
        println!(
            "incremental_scan_ms={} status={} tokens={:?}",
            start.elapsed().as_millis(),
            second.status,
            second.snapshot.as_ref().map(|s| s.days[0].tokens)
        );
        assert_eq!(second.status, "available");
    }
    #[test]
    #[ignore = "read-only check of real Claude Code logs and Desktop usage record; no content or paths printed"]
    fn live_claude_sources() {
        let mut collector = Collector::new(crate::platform::claude_home());
        let started = std::time::Instant::now();
        let state = collector.poll(crate::now_ms());
        println!(
            "logs status={} message={:?} files={} initial={:?} today_tokens={}",
            state.status,
            state.message,
            collector.files.len(),
            started.elapsed(),
            state.snapshot.as_ref().map_or(0, |s| s.days[0].tokens)
        );
        match crate::claude::read_once() {
            Ok((quota, _)) => println!(
                "quota status={} verified={} windows={:?}",
                quota.status,
                quota.identity_verified,
                quota.snapshot.map(|s| s
                    .windows
                    .into_iter()
                    .map(|w| (w.label, w.remaining_percent, w.resets_at_ms))
                    .collect::<Vec<_>>())
            ),
            Err(error) => println!("quota error={:?}", error.message),
        }
        assert_eq!(
            state.status,
            "available",
            "{}",
            state.message.unwrap_or_default()
        );
    }
}

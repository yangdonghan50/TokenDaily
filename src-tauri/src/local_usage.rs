//! Read-only, incremental accounting of the effective CODEX_HOME's local logs.
//! Persists only normalized counts, offsets and hashes; never saves log text.
use crate::usage::{UsageDay, UsageSnapshot, UsageState};
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

const SAFE_MAX: u64 = 9_007_199_254_740_991;
const MAX_LINE: u64 = 16 * 1024 * 1024;

pub(crate) fn beijing_day(ms: i64) -> String {
    DateTime::<Utc>::from_timestamp_millis(ms)
        .expect("valid clock")
        .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
        .format("%Y-%m-%d")
        .to_string()
}

#[cfg(test)]
fn codex_home() -> PathBuf {
    crate::platform::codex_home()
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct Cursor {
    offset: u64,
    observed_len: u64,
    last_event_ms: i64,
    modified: Option<SystemTime>,
    prefix: Vec<u8>,
    prefix_len: usize,
    previous: Option<(u64, u64, u64)>,
    turn: String,
    #[serde(default)]
    model: Option<String>,
    invalid: bool,
    skipping: bool,
    #[serde(skip)]
    verify: bool,
}

#[derive(Deserialize)]
struct Record {
    #[serde(rename = "type")]
    kind: String,
    timestamp: Option<String>,
    payload: Payload,
}
#[derive(Deserialize)]
struct Payload {
    #[serde(rename = "type")]
    kind: Option<String>,
    turn_id: Option<String>,
    model: Option<serde_json::Value>,
    info: Option<Info>,
}
#[derive(Deserialize)]
struct Info {
    total_token_usage: Counts,
    last_token_usage: Counts,
}
#[derive(Deserialize)]
struct Counts {
    cached_input_tokens: Option<serde_json::Value>,
    cache_write_input_tokens: Option<serde_json::Value>,
    input_tokens: u64,
    output_tokens: u64,
    total_tokens: u64,
}
impl Counts {
    fn tuple(&self) -> Result<(u64, u64, u64), ()> {
        if self.input_tokens > SAFE_MAX
            || self.output_tokens > SAFE_MAX
            || self.total_tokens > SAFE_MAX
        {
            return Err(());
        }
        Ok((self.input_tokens, self.output_tokens, self.total_tokens))
    }
    fn actual(&self) -> Result<u64, ()> {
        self.tuple()?;
        self.input_tokens
            .checked_add(self.output_tokens)
            .filter(|v| *v <= SAFE_MAX)
            .ok_or(())
    }
}

pub(crate) struct Collector {
    range: Option<(i64, i64)>,
    events: Vec<RankEvent>,
    home: PathBuf,
    day: String,
    cursors: HashMap<PathBuf, Cursor>,
    seen: HashSet<[u8; 32]>,
    tokens: u64,
    cost: crate::api_cost::ApiCost,
    previous: UsageState,
    restored: HashMap<String, Cursor>,
    pub(crate) dirty: bool,
    generation: u64,
    #[cfg(test)]
    read_bytes: u64,
}
impl Collector {
    pub(crate) fn new(home: PathBuf) -> Self {
        Self {
            range: None,
            events: Vec::new(),
            home,
            day: String::new(),
            cursors: HashMap::new(),
            seen: HashSet::new(),
            tokens: 0,
            cost: Default::default(),
            previous: UsageState::default(),
            restored: HashMap::new(),
            dirty: false,
            generation: 0,
            #[cfg(test)]
            read_bytes: 0,
        }
    }
    pub(crate) fn reset(&mut self) {
        let generation = self.generation;
        *self = Self::new(self.home.clone());
        self.generation = generation;
    }
    #[cfg(test)]
    fn total(&self) -> u64 {
        self.tokens
    }

    fn consume(&mut self, cursor: &mut Cursor, line: &[u8]) -> Result<(), ()> {
        // Ignore all prompt/response/tool text without deserializing it.
        if memchr::memmem::find(line, b"token_count").is_none()
            && memchr::memmem::find(line, b"turn_context").is_none()
        {
            return Ok(());
        }
        let record: Record = serde_json::from_slice(line).map_err(|_| ())?;
        if record.kind == "turn_context" {
            cursor.model = record
                .payload
                .model
                .and_then(|v| v.as_str().filter(|s| s.len() <= 128).map(str::to_owned));
            cursor.turn = record
                .payload
                .turn_id
                .map(|id| digest(id.as_bytes()))
                .unwrap_or_default();
            return Ok(());
        }
        if record.kind != "event_msg" || record.payload.kind.as_deref() != Some("token_count") {
            return Ok(());
        }
        let Some(info) = record.payload.info else {
            return Ok(());
        }; // Rate-limit-only event.
        let total = info.total_token_usage.tuple()?;
        let amount = info.last_token_usage.actual()?;
        let timestamp = DateTime::parse_from_rfc3339(record.timestamp.as_deref().ok_or(())?)
            .map_err(|_| ())?
            .timestamp_millis();
        cursor.last_event_ms = cursor.last_event_ms.max(timestamp);
        let duplicate = cursor.previous == Some(total);
        cursor.previous = Some(total);
        // total_token_usage is cumulative; last_token_usage describes a response.
        // Context-window estimates have no input/output and must not be billed as use.
        if duplicate || amount == 0 {
            return Ok(());
        }
        if self.range.map_or_else(
            || beijing_day(timestamp) != self.day,
            |(since, until)| timestamp < since || timestamp > until,
        ) {
            return Ok(());
        }
        // Copied rollout prefixes (forks, archived/moved files) retain event timestamps
        // and turn IDs. Hash normalized fields, never file paths or conversation text.
        let key: [u8; 32] = Sha256::digest(format!(
            "{}|{timestamp}|{total:?}|{:?}",
            cursor.turn,
            info.last_token_usage.tuple()?
        ))
        .into();
        if self.seen.insert(key) {
            self.cost.add(crate::api_cost::codex(
                cursor.model.as_deref(),
                info.last_token_usage.input_tokens,
                info.last_token_usage.output_tokens,
                info.last_token_usage
                    .cached_input_tokens
                    .as_ref()
                    .and_then(serde_json::Value::as_u64),
                info.last_token_usage
                    .cache_write_input_tokens
                    .as_ref()
                    .and_then(serde_json::Value::as_u64),
            ));
            if self.range.is_some() {
                if self.events.len() >= 100_000 {
                    return Err(());
                }
                self.events.push(RankEvent {
                    id: key.iter().map(|b| format!("{b:02x}")).collect(),
                    at: timestamp,
                    tokens: amount,
                    tool: "codex".into(),
                });
            }
            self.tokens = self
                .tokens
                .checked_add(amount)
                .filter(|n| *n <= SAFE_MAX)
                .ok_or(())?;
        }
        Ok(())
    }

    fn read_file(&mut self, path: &Path) -> Result<(), ()> {
        let metadata = fs::metadata(path).map_err(|_| ())?;
        let modified = metadata.modified().ok();
        let key = path_key(path);
        let mut cursor = self
            .cursors
            .remove(path)
            .or_else(|| self.restored.remove(&key))
            .unwrap_or_default();
        let result = (|| {
            if metadata.len() == cursor.offset && modified == cursor.modified && !cursor.verify {
                return Ok(());
            }
            self.dirty = true;
            let mut file = File::open(path).map_err(|_| ())?;
            let mut prefix = vec![0; metadata.len().min(256) as usize];
            file.read_exact(&mut prefix).map_err(|_| ())?;
            if metadata.len() < cursor.offset
                || cursor.prefix_len > prefix.len()
                || cursor.prefix_len > 0
                    && Sha256::digest(&prefix[..cursor.prefix_len]).as_slice() != cursor.prefix
                || (metadata.len() == cursor.offset && modified != cursor.modified)
            {
                cursor = Cursor::default();
            }
            cursor.prefix_len = prefix.len();
            cursor.prefix = Sha256::digest(&prefix).to_vec();
            cursor.verify = false;
            file.seek(SeekFrom::Start(cursor.offset)).map_err(|_| ())?;
            // Snapshot EOF: an active producer cannot keep this scan running forever.
            let mut reader = BufReader::new(file.take(metadata.len() - cursor.offset));
            let mut line = Vec::new();
            loop {
                line.clear();
                let count = reader
                    .by_ref()
                    .take(MAX_LINE)
                    .read_until(b'\n', &mut line)
                    .map_err(|_| ())?;
                if count == 0 {
                    break;
                }
                #[cfg(test)]
                {
                    self.read_bytes += count as u64;
                }
                if cursor.skipping {
                    cursor.offset += count as u64;
                    cursor.skipping = line.last() != Some(&b'\n');
                    continue;
                }
                if line.last() != Some(&b'\n') {
                    if count as u64 == MAX_LINE {
                        cursor.invalid = true;
                        cursor.offset += count as u64;
                        cursor.skipping = true;
                        continue;
                    }
                    break; // Incomplete append: retry from the same offset next poll.
                }
                cursor.offset += count as u64;
                if self.consume(&mut cursor, &line).is_err() {
                    cursor.invalid = true;
                }
            }
            cursor.modified = modified;
            cursor.observed_len = metadata.len();
            Ok(())
        })();
        let invalid = cursor.invalid;
        self.cursors.insert(path.to_owned(), cursor);
        result.and(if invalid { Err(()) } else { Ok(()) })
    }

    pub(crate) fn poll(&mut self, now: i64) -> UsageState {
        let day = beijing_day(now);
        if self.day != day {
            if day < self.day {
                self.cursors.clear();
                self.restored.clear();
            }
            self.day = day;
            self.seen.clear();
            self.tokens = 0;
            self.cost = Default::default();
            self.previous = UsageState::default();
            self.dirty = true;
        }
        let mut files = Vec::new();
        let mut failed = discover(&self.home.join("sessions"), &mut files).is_err();
        let archive = self.home.join("archived_sessions");
        if archive.exists() {
            failed |= discover(&archive, &mut files).is_err();
        }
        // File dates are not event dates: Windows can defer mtime until the writer
        // closes its handle. Read every file once, then only appended bytes.
        for path in files {
            failed |= self.read_file(&path).is_err();
        }
        let next = if failed {
            let mut stale = if self.tokens > 0 {
                self.snapshot(now)
            } else {
                self.previous.clone()
            };
            stale =
                UsageState::with_error(&stale, "本机日志读取不完整，请检查 Codex 日志和目录权限");
            stale.last_success_at_ms = self.previous.last_success_at_ms;
            stale
        } else {
            self.snapshot(now)
        };
        self.previous = next.clone();
        next
    }

    fn snapshot(&self, now: i64) -> UsageState {
        UsageState::available(UsageSnapshot {
            api_cost: Some(self.cost.clone()),
            identity_key: "local-codex-beijing".into(),
            collected_at_ms: now,
            days: vec![UsageDay {
                date: self.day.clone(),
                tokens: self.tokens,
            }],
        })
    }
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Minimal opt-in export. Reuses the same parser/dedup keys as local daily totals.
/// Never exposes file paths, original turn IDs or conversation content.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct RankEvent {
    pub(crate) id: String,
    pub(crate) at: i64,
    pub(crate) tokens: u64,
    pub(crate) tool: String,
}
pub(crate) fn rank_events(
    home: PathBuf,
    since: i64,
    now: i64,
) -> Result<Vec<RankEvent>, &'static str> {
    let mut collector = Collector::new(home);
    collector.range = Some((since, now));
    let state = collector.poll(now);
    if state.status != "available" {
        return Err("本机日志读取不完整，未上传；请检查 Codex 日志权限后重试");
    }
    collector
        .events
        .sort_by(|a, b| (a.at, &a.id).cmp(&(b.at, &b.id)));
    Ok(collector.events)
}
fn path_key(path: &Path) -> String {
    digest(path.to_string_lossy().as_bytes())
}
#[derive(Serialize, Deserialize)]
struct Checkpoint {
    cost: crate::api_cost::ApiCost,
    version: u32,
    generation: u64,
    home: String,
    day: String,
    tokens: u64,
    seen: HashSet<[u8; 32]>,
    cursors: HashMap<String, Cursor>,
}
#[derive(Serialize, Deserialize)]
struct Envelope {
    payload: String,
    digest: String,
}
pub(crate) fn index_paths(path: &Path) -> [PathBuf; 2] {
    [path.with_extension("a.json"), path.with_extension("b.json")]
}
impl Collector {
    pub(crate) fn restore(home: PathBuf, path: &Path) -> Self {
        let mut fresh = Self::new(home);
        let restored = index_paths(path)
            .into_iter()
            .filter_map(|slot| {
                if fs::metadata(&slot).ok()?.len() > 8 * 1024 * 1024 {
                    return None;
                }
                let bytes = fs::read(&slot).ok()?;
                let envelope: Envelope = serde_json::from_slice(&bytes).ok()?;
                if digest(envelope.payload.as_bytes()) != envelope.digest {
                    return None;
                }
                let mut data: Checkpoint = serde_json::from_str(&envelope.payload).ok()?;
                if data.version != 3
                    || data.home != path_key(&fresh.home)
                    || data.tokens > SAFE_MAX
                    || !data.cost.usd.is_finite()
                    || data.cost.usd < 0.
                    || data
                        .cost
                        .priced_tokens
                        .checked_add(data.cost.unpriced_tokens)
                        != Some(data.tokens)
                    || chrono::NaiveDate::parse_from_str(&data.day, "%Y-%m-%d").is_err()
                    || data
                        .cursors
                        .values()
                        .any(|c| c.prefix_len > 256 || c.prefix.len() != 32)
                {
                    return None;
                }
                for cursor in data.cursors.values_mut() {
                    cursor.verify = true;
                }
                Some(data)
            })
            .max_by_key(|data| data.generation);
        if let Some(data) = restored {
            fresh.day = data.day;
            fresh.tokens = data.tokens;
            fresh.cost = data.cost;
            fresh.seen = data.seen;
            fresh.restored = data.cursors;
            fresh.generation = data.generation;
        }
        fresh
    }

    /// Probe a bounded set of recent writers when OS notifications are delayed
    /// by the filesystem cache. No directory traversal or content reads here.
    pub(crate) fn hot_changed(&self) -> bool {
        let mut recent: Vec<_> = self.cursors.iter().filter(|(_, c)| !c.invalid).collect();
        recent.sort_unstable_by_key(|(_, c)| std::cmp::Reverse(c.last_event_ms));
        recent
            .into_iter()
            .take(8)
            .any(|(path, c)| fs::metadata(path).is_ok_and(|m| m.len() != c.observed_len))
    }
    pub(crate) fn checkpoint(&mut self, path: &Path) -> Result<(), ()> {
        if !self.dirty {
            return Ok(());
        }
        let mut cursors = self.restored.clone();
        cursors.extend(self.cursors.iter().map(|(p, c)| (path_key(p), c.clone())));
        let data = Checkpoint {
            cost: self.cost.clone(),
            version: 3,
            generation: self.generation.checked_add(1).ok_or(())?,
            home: path_key(&self.home),
            day: self.day.clone(),
            tokens: self.tokens,
            seen: self.seen.clone(),
            cursors,
        };
        let payload = serde_json::to_string(&data).map_err(|_| ())?;
        let envelope = Envelope {
            digest: digest(payload.as_bytes()),
            payload,
        };
        let bytes = serde_json::to_vec(&envelope).map_err(|_| ())?;
        if bytes.len() > 8 * 1024 * 1024 {
            return Err(());
        }
        // Alternating checksummed slots survive interrupted writes without
        // relying on rename across virtualized Windows filesystem boundaries.
        let destination = index_paths(path)[data.generation as usize % 2].clone();
        let mut file = File::create(destination).map_err(|_| ())?;
        file.write_all(&bytes).map_err(|_| ())?;
        file.sync_all().map_err(|_| ())?;
        self.generation = data.generation;
        self.dirty = false;
        Ok(())
    }
}

fn discover(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), ()> {
    for entry in fs::read_dir(dir).map_err(|_| ())? {
        let entry = entry.map_err(|_| ())?;
        let kind = entry.file_type().map_err(|_| ())?;
        // Do not follow directory junctions or symlinks into unrelated roots.
        if kind.is_symlink() {
            continue;
        }
        if kind.is_dir() {
            discover(&entry.path(), files)?;
        } else if kind.is_file() && entry.path().extension().is_some_and(|ext| ext == "jsonl") {
            files.push(entry.path());
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "local_usage_tests.rs"]
mod tests;

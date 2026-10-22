//! Read-only TRAE local usage. No credentials or plaintext database files.
use crate::{
    local_usage::beijing_day,
    usage::{UsageDay, UsageSnapshot, UsageState},
};
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use hmac::{Hmac, Mac};
use rusqlite::{Connection, MAIN_DB};
use serde_json::Value;
use sha2::Sha512;
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
    time::SystemTime,
};
use zeroize::Zeroizing;

const MAX: u64 = 9_007_199_254_740_991;
const LIMIT: u64 = 64 * 1024 * 1024;
const PAGE: usize = 4096;
// Public application-level database key, NOT an account/login credential.
// Format reference: TokenTracker/src/lib/trae-snapshot.js (MIT).
const APPLICATION_KEY: [u8; 32] = [
    0x36, 0x05, 0xf6, 0x69, 0x10, 0x95, 0xa9, 0x93, 0xf0, 0x3d, 0x50, 0x09, 0xc9, 0x18, 0x35, 0x2e,
    0xf5, 0xbe, 0x31, 0xae, 0x31, 0xe8, 0xf0, 0x00, 0x21, 0x2b, 0x81, 0xff, 0x05, 0x8d, 0xa7, 0x73,
];
type Result<T> = std::result::Result<T, &'static str>;
#[derive(Clone)]
struct Record {
    key: String,
    at: i64,
    updated: i64,
    tokens: u64,
}
fn number(v: &Value, key: &str) -> Result<Option<u64>> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(n) => n
            .as_u64()
            .filter(|n| *n <= MAX)
            .map(Some)
            .ok_or("TRAE Token 字段无效"),
    }
}
fn counts(v: &Value) -> Result<u64> {
    let input = number(v, "prompt_tokens")?.ok_or("TRAE 输入计数缺失")?;
    let output = number(v, "completion_tokens")?.ok_or("TRAE 输出计数缺失")?;
    let sum = input
        .checked_add(output)
        .filter(|n| *n <= MAX)
        .ok_or("TRAE Token 超出范围")?;
    let p = number(v, "prompt_tokens_total")?.unwrap_or(0);
    let c = number(v, "completion_tokens_total")?.unwrap_or(0);
    let cumulative = p
        .checked_add(c)
        .filter(|n| *n <= MAX)
        .ok_or("TRAE Token 超出范围")?;
    if cumulative > 0 && (p < input || c < output) {
        return Err("TRAE 累计计数与单次计数不一致");
    }
    if number(v, "total_tokens")?.is_some_and(|n| n != sum && (cumulative == 0 || n != cumulative))
    {
        return Err("TRAE Token 总计不一致");
    }
    for key in [
        "cache_read_input_tokens",
        "cache_creation_input_tokens",
        "reasoning_tokens",
    ] {
        number(v, key)?;
    }
    Ok(if cumulative > 0 { cumulative } else { sum })
}
fn aggregate(rows: Vec<Record>, since: i64, now: i64) -> Result<BTreeMap<String, u64>> {
    let mut unique = BTreeMap::<String, Record>::new();
    for row in rows {
        if let Some(old) = unique.get(&row.key) {
            if old.updated == row.updated && (old.at != row.at || old.tokens != row.tokens) {
                return Err("TRAE 重复记录存在冲突");
            }
            if old.updated > row.updated {
                continue;
            }
        }
        unique.insert(row.key.clone(), row);
    }
    let mut days = BTreeMap::new();
    for row in unique.values().filter(|r| r.at >= since && r.at <= now) {
        let total = days.entry(beijing_day(row.at)).or_insert(0u64);
        *total = total
            .checked_add(row.tokens)
            .filter(|n| *n <= MAX)
            .ok_or("TRAE 合计超出范围")?;
    }
    Ok(days)
}
#[derive(PartialEq, Clone)]
struct Stamp {
    length: u64,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
}
fn stamp(p: &Path) -> Result<Option<Stamp>> {
    match fs::metadata(p) {
        Ok(s) if s.is_file() => Ok(Some(Stamp {
            length: s.len(),
            modified: s.modified().ok(),
            created: s.created().ok(),
        })),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        _ => Err("TRAE 数据库元数据读取失败"),
    }
}
fn paths(path: &Path) -> [PathBuf; 3] {
    [
        path.to_owned(),
        PathBuf::from(format!("{}-wal", path.display())),
        PathBuf::from(format!("{}-journal", path.display())),
    ]
}
fn fingerprint(path: &Path) -> Result<Vec<Option<Stamp>>> {
    paths(path).iter().map(|p| stamp(p)).collect()
}
fn read(p: &Path) -> Result<Zeroizing<Vec<u8>>> {
    let mut data = Zeroizing::new(Vec::new());
    fs::File::open(p)
        .map_err(|_| "TRAE 数据库读取失败")?
        .take(LIMIT + 1)
        .read_to_end(&mut data)
        .map_err(|_| "TRAE 数据库读取中断")?;
    if data.len() as u64 > LIMIT {
        return Err("TRAE 数据文件超过 64 MiB，本版本暂不读取，避免占用过多内存");
    }
    Ok(data)
}
fn be(b: &[u8], at: usize) -> u32 {
    u32::from_be_bytes(b[at..at + 4].try_into().unwrap())
}
fn checksum(data: &[u8], big: bool, mut state: [u32; 2]) -> [u32; 2] {
    for pair in data.chunks_exact(8) {
        let read = |b: &[u8]| {
            if big {
                u32::from_be_bytes(b.try_into().unwrap())
            } else {
                u32::from_le_bytes(b.try_into().unwrap())
            }
        };
        state[0] = state[0]
            .wrapping_add(read(&pair[..4]))
            .wrapping_add(state[1]);
        state[1] = state[1]
            .wrapping_add(read(&pair[4..]))
            .wrapping_add(state[0]);
    }
    state
}
fn wal_index(
    wal: &[u8],
    page_size: usize,
    db_pages: usize,
) -> Result<(usize, BTreeMap<usize, usize>)> {
    if wal.is_empty() {
        return Ok((db_pages, BTreeMap::new()));
    }
    if wal.len() < 32
        || ![0x377f0682, 0x377f0683].contains(&be(wal, 0))
        || be(wal, 4) != 3007000
        || be(wal, 8) as usize != page_size
    {
        return Err("TRAE 写入日志格式未知或未完成");
    }
    let big = be(wal, 0) == 0x377f0683;
    let mut sum = checksum(&wal[..24], big, [0, 0]);
    if sum != [be(wal, 24), be(wal, 28)] {
        return Err("TRAE 写入日志校验失败");
    }
    let mut committed = BTreeMap::new();
    let mut pending = BTreeMap::new();
    let mut count = db_pages;
    for (i, f) in wal[32..].chunks_exact(24 + page_size).enumerate() {
        if f[8..16] != wal[16..24] {
            break;
        }
        sum = checksum(&f[24..], big, checksum(&f[..8], big, sum));
        if sum != [be(f, 16), be(f, 20)] {
            return Err("TRAE 写入日志帧校验失败，请稍后刷新");
        }
        let page = be(f, 0) as usize;
        let commit = be(f, 4) as usize;
        if page == 0 || page > LIMIT as usize / page_size || commit > LIMIT as usize / page_size {
            return Err("TRAE 写入日志大小无效");
        }
        pending.insert(page, 32 + i * (24 + page_size) + 24);
        if commit > 0 {
            committed.append(&mut pending);
            count = commit;
            committed.retain(|n, _| *n <= count);
        }
    }
    Ok((count, committed))
}
fn snapshot(path: &Path) -> Result<Zeroizing<Vec<u8>>> {
    let before = fingerprint(path)?;
    let p = paths(path);
    if before[0].is_none() {
        return Err("未发现 TRAE CN 本地数据库");
    }
    if before.iter().flatten().any(|s| s.length > LIMIT) {
        return Err("TRAE 数据文件超过 64 MiB，本版本暂不读取");
    }
    if before[2].as_ref().is_some_and(|s| s.length >= 8) {
        let journal = read(&p[2])?;
        if journal.iter().take(8).any(|b| *b != 0) {
            return Err("TRAE 正在提交事务，请稍后刷新");
        }
    }
    let encrypted = read(path)?;
    if encrypted.len() < 100 {
        return Err("TRAE 数据库文件不完整");
    }
    let is_encrypted = &encrypted[..16] != b"SQLite format 3\0";
    let page_size = if is_encrypted {
        PAGE
    } else {
        match u16::from_be_bytes(encrypted[16..18].try_into().unwrap()) {
            1 => 65536,
            n => n as usize,
        }
    };
    if !(512..=65536).contains(&page_size)
        || !page_size.is_power_of_two()
        || encrypted.len() % page_size != 0
    {
        return Err("TRAE 数据库页格式未知");
    }
    let wal = if before[1].is_some() {
        read(&p[1])?
    } else {
        Zeroizing::new(Vec::new())
    };
    let (count, index) = wal_index(&wal, page_size, encrypted.len() / page_size)?;
    if count == 0 || count > LIMIT as usize / page_size {
        return Err("TRAE 数据库大小无效");
    }
    let mut result = Zeroizing::new(vec![0u8; count * page_size]);
    let mut mac_key = Zeroizing::new([0u8; 32]);
    let salt: Vec<u8> = encrypted[..16].iter().map(|b| b ^ 0x3a).collect();
    if is_encrypted {
        pbkdf2::pbkdf2_hmac::<Sha512>(&APPLICATION_KEY, &salt, 2, &mut *mac_key);
    }
    for n in 1..=count {
        let src = if let Some(offset) = index.get(&n) {
            &wal[*offset..*offset + page_size]
        } else {
            encrypted
                .get((n - 1) * page_size..n * page_size)
                .ok_or("TRAE 数据库页缺失")?
        };
        let dest = &mut result[(n - 1) * page_size..n * page_size];
        if is_encrypted {
            let start = if n == 1 { 16 } else { 0 };
            let mut mac = Hmac::<Sha512>::new_from_slice(&*mac_key).unwrap();
            mac.update(&src[start..4032]);
            mac.update(&(n as u32).to_le_bytes());
            mac.verify_slice(&src[4032..])
                .map_err(|_| "TRAE 数据库认证失败：格式变化、密钥不适用或文件损坏")?;
            dest[start..4016].copy_from_slice(&src[start..4016]);
            cbc::Decryptor::<aes::Aes256>::new((&APPLICATION_KEY).into(), src[4016..4032].into())
                .decrypt_padded_mut::<NoPadding>(&mut dest[start..4016])
                .map_err(|_| "TRAE 数据页解密失败")?;
            if n == 1 {
                dest[..16].copy_from_slice(b"SQLite format 3\0");
                if u16::from_be_bytes(dest[16..18].try_into().unwrap()) != 4096 || dest[20] != 80 {
                    return Err("TRAE 数据库加密格式不支持");
                }
            }
        } else {
            dest.copy_from_slice(src);
        }
    }
    result[18] = 1;
    result[19] = 1;
    result[28..32].copy_from_slice(&(count as u32).to_be_bytes());
    let counter = result[24..28].to_vec();
    result[92..96].copy_from_slice(&counter);
    if fingerprint(path)? != before {
        return Err("TRAE 正在写入数据库，请稍后刷新");
    }
    Ok(result)
}
fn records(path: &Path) -> Result<Vec<Record>> {
    let before = fingerprint(path)?;
    let data = snapshot(path)?;
    let mut connection = Connection::open_in_memory().map_err(|_| "TRAE 内存查询初始化失败")?;
    connection
        .deserialize_read_exact(MAIN_DB, &data[..], data.len(), true)
        .map_err(|_| "TRAE 数据库无法识别")?;
    connection.execute_batch("PRAGMA query_only=ON; PRAGMA trusted_schema=OFF; PRAGMA temp_store=MEMORY; PRAGMA cache_size=-2048;").map_err(|_|"TRAE 查询初始化失败")?;
    if connection.query_row("SELECT EXISTS(SELECT 1 FROM chat_turn WHERE context IS NOT NULL AND trim(context)!='' AND NOT json_valid(context))", [], |r| r.get::<_,bool>(0)).map_err(|_|"TRAE 数据结构无法识别")? {return Err("TRAE 本地记录损坏，统计不完整");}
    // Only count metadata crosses SQLite; no messages, titles or credentials.
    let mut statement=connection.prepare("SELECT session_id, turn_id, created_at, updated_at, json_object('prompt_tokens',json_extract(context,'$.token_usage.prompt_tokens'),'completion_tokens',json_extract(context,'$.token_usage.completion_tokens'),'total_tokens',json_extract(context,'$.token_usage.total_tokens'),'prompt_tokens_total',json_extract(context,'$.token_usage.prompt_tokens_total'),'completion_tokens_total',json_extract(context,'$.token_usage.completion_tokens_total'),'cache_read_input_tokens',json_extract(context,'$.token_usage.cache_read_input_tokens'),'cache_creation_input_tokens',json_extract(context,'$.token_usage.cache_creation_input_tokens'),'reasoning_tokens',json_extract(context,'$.token_usage.reasoning_tokens')) FROM chat_turn WHERE CASE WHEN json_valid(context) THEN json_type(context,'$.token_usage')='object' ELSE 0 END").map_err(|_|"TRAE 数据库版本尚不支持")?;
    let mut cursor = statement.query([]).map_err(|_| "TRAE 用量查询失败")?;
    let mut result = Vec::new();
    while let Some(row) = cursor.next().map_err(|_| "TRAE 用量读取失败")? {
        if result.len() >= 200_000 {
            return Err("TRAE 用量记录过多，本版本暂不读取");
        }
        let s: String = row.get(0).map_err(|_| "TRAE 会话标识缺失")?;
        let t: String = row.get(1).map_err(|_| "TRAE 轮次标识缺失")?;
        if s.is_empty() || t.is_empty() {
            return Err("TRAE 用量标识为空");
        }
        let at: i64 = row.get(2).map_err(|_| "TRAE 用量时间无效")?;
        let updated: i64 = row.get(3).map_err(|_| "TRAE 更新时间无效")?;
        let at = at
            .checked_mul(1000)
            .filter(|t| *t > 0 && chrono::DateTime::from_timestamp_millis(*t).is_some())
            .ok_or("TRAE 用量时间无效")?;
        let usage: String = row.get(4).map_err(|_| "TRAE 用量字段无效")?;
        let key = format!(
            "{:x}",
            sha2::Sha256::digest(serde_json::to_vec(&(s, t)).unwrap())
        );
        result.push(Record {
            key,
            at,
            updated,
            tokens: counts(&serde_json::from_str(&usage).map_err(|_| "TRAE 用量字段损坏")?)?,
        });
    }
    if fingerprint(path)? != before {
        return Err("TRAE 正在写入数据库，请稍后刷新");
    }
    Ok(result)
}
use sha2::Digest;
pub(crate) fn history_days(path: PathBuf, since: i64, now: i64) -> Result<BTreeMap<String, u64>> {
    aggregate(records(&path)?, since, now)
}
pub(crate) struct Collector {
    path: PathBuf,
    previous: UsageState,
    stamp: Vec<Option<Stamp>>,
    rows: Vec<Record>,
    last_read: i64,
    last_attempt: i64,
    seen: bool,
    #[cfg(test)]
    reads: u64,
}
impl Collector {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            previous: UsageState::default(),
            stamp: Vec::new(),
            rows: Vec::new(),
            last_read: 0,
            last_attempt: 0,
            seen: false,
            #[cfg(test)]
            reads: 0,
        }
    }
    pub(crate) fn poll(&mut self, now: i64, force: bool) -> UsageState {
        let today = beijing_day(now);
        if self
            .previous
            .snapshot
            .as_ref()
            .is_some_and(|s| s.days[0].date != today)
        {
            self.previous = UsageState::default();
        }
        let result = (|| {
            let current = fingerprint(&self.path)?;
            if current[0].is_none() {
                return Err("未发现 TRAE CN 本地数据库");
            }
            // A readable metadata stamp alone cannot confirm a new day's zero:
            // Windows may allow attributes while denying shared file reads.
            if self.previous.status != "available" {
                fs::File::open(&self.path).map_err(|_| "TRAE 数据库暂不可读，请稍后刷新")?;
            }
            if !force
                && current == self.stamp
                && self.previous.status == "available"
                && self.previous.snapshot.is_some()
            {
                let mut cached = self.previous.clone();
                cached.snapshot.as_mut().unwrap().collected_at_ms = now;
                cached.last_success_at_ms = Some(now);
                return Ok(cached);
            }
            if self.last_read == 0
                || current != self.stamp
                || force && now - self.last_attempt >= 10_000
            {
                let interval = if force { 10_000 } else { 30_000 };
                if now - self.last_attempt < interval && self.last_attempt != 0 {
                    return Err("TRAE 数据更新等待中");
                }
                self.last_attempt = now;
                #[cfg(test)]
                {
                    self.reads += 1;
                }
                self.rows = records(&self.path)?;
                self.stamp = current;
                self.last_read = now;
                self.seen |= !self.rows.is_empty();
            }
            if self.last_read == 0 {
                return Err("TRAE 数据库尚未读取");
            }
            if self.rows.is_empty() && !self.seen {
                return Err("未发现 TRAE 有效 Token 记录");
            }
            let days = aggregate(self.rows.clone(), 0, now)?;
            let mut state = UsageState::available(UsageSnapshot {
                identity_key: "trae-local-records-beijing".into(),
                collected_at_ms: now,
                api_cost: None,
                days: vec![UsageDay {
                    date: today.clone(),
                    tokens: *days.get(&today).unwrap_or(&0),
                }],
            });
            state.message = Some(
                "TRAE 本地记录；恢复或同步的会话可能来自其他设备，尚不能保证设备独占。".into(),
            );
            Ok(state)
        })();
        self.previous = match result {
            Ok(s) => s,
            Err(e) => {
                let mut s = UsageState::with_error(&self.previous, e);
                s.source_unavailable = !self.seen
                    && (e == "未发现 TRAE CN 本地数据库" || e == "未发现 TRAE 有效 Token 记录");
                s
            }
        };
        self.previous.clone()
    }
}
#[cfg(test)]
#[path = "trae_usage_tests.rs"]
mod tests;

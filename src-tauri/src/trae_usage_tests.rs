use super::*;
use serde_json::json;

#[test]
fn usage_counts_cache_and_reasoning_once_and_checks_totals() {
    assert_eq!(counts(&json!({"prompt_tokens":25903,"completion_tokens":997,"total_tokens":26900,"cache_read_input_tokens":2176,"reasoning_tokens":264,"prompt_tokens_total":0,"completion_tokens_total":0})).unwrap(),26900);
    assert_eq!(counts(&json!({"prompt_tokens":10,"completion_tokens":2,"total_tokens":12,"prompt_tokens_total":30,"completion_tokens_total":6})).unwrap(),36);
    for bad in [
        json!({"prompt_tokens":-1,"completion_tokens":2}),
        json!({"prompt_tokens":1.5,"completion_tokens":2}),
        json!({"prompt_tokens":10,"completion_tokens":2,"total_tokens":13}),
        json!({"prompt_tokens":MAX,"completion_tokens":1}),
        json!({"total_tokens":12}),
    ] {
        assert!(counts(&bad).is_err());
    }
}

#[test]
fn duplicates_and_corrections_are_replaced_and_beijing_day_is_used() {
    let at = chrono::DateTime::parse_from_rfc3339("2026-10-18T16:00:00Z")
        .unwrap()
        .timestamp_millis();
    let row = |updated, tokens| Record {
        key: "opaque-turn".into(),
        at,
        updated,
        tokens,
    };
    let days = aggregate(vec![row(1, 20), row(1, 20), row(2, 12)], at - 1, at).unwrap();
    assert_eq!(days.get("2026-10-19"), Some(&12));
    assert!(!days.contains_key("2026-10-18"));
    assert!(aggregate(vec![row(1, 20), row(1, 21)], at - 1, at).is_err());
    assert!(aggregate(vec![row(2, 12)], at + 1, at + 2)
        .unwrap()
        .is_empty());
}

#[test]
fn wal_exposes_only_committed_frames_and_rejects_corruption() {
    let mut wal = vec![0u8; 32];
    wal[0..4].copy_from_slice(&0x377f0682u32.to_be_bytes());
    wal[4..8].copy_from_slice(&3007000u32.to_be_bytes());
    wal[8..12].copy_from_slice(&512u32.to_be_bytes());
    let mut sum = checksum(&wal[..24], false, [0, 0]);
    wal[24..28].copy_from_slice(&sum[0].to_be_bytes());
    wal[28..32].copy_from_slice(&sum[1].to_be_bytes());
    for commit in [1u32, 0] {
        let mut frame = vec![0u8; 24 + 512];
        frame[..4].copy_from_slice(&1u32.to_be_bytes());
        frame[4..8].copy_from_slice(&commit.to_be_bytes());
        frame[24..].fill(if commit > 0 { 11 } else { 22 });
        sum = checksum(&frame[24..], false, checksum(&frame[..8], false, sum));
        frame[16..20].copy_from_slice(&sum[0].to_be_bytes());
        frame[20..24].copy_from_slice(&sum[1].to_be_bytes());
        wal.extend(frame);
    }
    let (count, index) = wal_index(&wal, 512, 1).unwrap();
    assert_eq!(count, 1);
    assert_eq!(wal[*index.get(&1).unwrap()], 11);
    wal[100] ^= 1;
    assert!(wal_index(&wal, 512, 1).is_err());
}

#[test]
fn encrypted_page_is_authenticated_before_use_and_no_plain_file_is_created() {
    use aes::cipher::BlockEncryptMut;
    let path = std::env::temp_dir().join(format!("tokendaily-trae-page-{}.db", crate::now_ms()));
    let mut encrypted = vec![0u8; PAGE];
    encrypted[..16].fill(7);
    encrypted[4016..4032].fill(9);
    let mut plain = vec![0u8; 4000];
    plain[..2].copy_from_slice(&4096u16.to_be_bytes());
    plain[4] = 80;
    let iv: [u8; 16] = encrypted[4016..4032].try_into().unwrap();
    cbc::Encryptor::<aes::Aes256>::new((&APPLICATION_KEY).into(), (&iv).into())
        .encrypt_padded_mut::<NoPadding>(&mut plain, 4000)
        .unwrap();
    encrypted[16..4016].copy_from_slice(&plain);
    let salt: Vec<u8> = encrypted[..16].iter().map(|b| b ^ 0x3a).collect();
    let mut key = [0; 32];
    pbkdf2::pbkdf2_hmac::<Sha512>(&APPLICATION_KEY, &salt, 2, &mut key);
    let mut mac = Hmac::<Sha512>::new_from_slice(&key).unwrap();
    mac.update(&encrypted[16..4032]);
    mac.update(&1u32.to_le_bytes());
    encrypted[4032..].copy_from_slice(&mac.finalize().into_bytes());
    std::fs::write(&path, &encrypted).unwrap();
    let result = snapshot(&path).unwrap();
    assert_eq!(&result[..16], b"SQLite format 3\0");
    assert_eq!(result[20], 80);
    assert_eq!(std::fs::read(&path).unwrap(), encrypted);
    encrypted[4095] ^= 1;
    std::fs::write(&path, &encrypted).unwrap();
    assert!(snapshot(&path).is_err());
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn unchanged_database_is_not_reopened_and_cross_day_failure_is_not_zero() {
    let root = std::env::temp_dir().join(format!("tokendaily-trae-{}", crate::now_ms()));
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join("database.db");
    let connection = rusqlite::Connection::open_in_memory().unwrap();
    connection.execute_batch("CREATE TABLE chat_turn(session_id TEXT, turn_id TEXT, created_at INTEGER, updated_at INTEGER,context TEXT);
        INSERT INTO chat_turn VALUES('s','t',1792378438,1792378438,'{\"token_usage\":{\"prompt_tokens\":25903,\"completion_tokens\":997,\"total_tokens\":26900}}');").unwrap();
    std::fs::write(&path, &*connection.serialize(rusqlite::MAIN_DB).unwrap()).unwrap();
    let now = 1792380000000;
    let mut collector = Collector::new(path.clone());
    let first = collector.poll(now, false);
    assert_eq!(first.snapshot.unwrap().days[0].tokens, 26900);
    let reads = collector.reads;
    assert_eq!(collector.poll(now + 2000, false).status, "available");
    assert_eq!(collector.reads, reads);
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::OpenOptionsExt;
        let held = std::fs::OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(&path)
            .unwrap();
        let blocked = collector.poll(now + 86400000, false);
        assert!(
            blocked.snapshot.is_none(),
            "a new-day zero needs a readable source, even when metadata is accessible"
        );
        assert_ne!(blocked.status, "available");
        drop(held);
    }
    assert_eq!(
        collector.poll(now + 86400000, false).snapshot.unwrap().days[0].tokens,
        0
    );
    std::fs::write(&path, b"corrupt").unwrap();
    let failed = collector.poll(now + 2 * 86400000, true);
    assert_ne!(failed.status, "available");
    assert!(failed.snapshot.is_none());
    std::fs::remove_file(&path).unwrap();
    assert!(
        !collector
            .poll(now + 2 * 86400000 + 2000, true)
            .source_unavailable
    );
    std::fs::remove_dir(&root).unwrap();
}

#[test]
#[ignore = "reads real local TRAE counts only; no conversation output"]
fn live_trae_database() {
    let start = std::time::Instant::now();
    let mut c = Collector::new(crate::platform::trae_database());
    let usage = c.poll(crate::now_ms(), true);
    assert_eq!(usage.status, "available", "{:?}", usage.message);
    let initial = start.elapsed();
    let reads = c.reads;
    c.poll(crate::now_ms(), false);
    assert_eq!(reads, c.reads);
    println!(
        "TRAE today Tokens={}, initial={:?}, unchanged body reads=0",
        usage.snapshot.unwrap().days[0].tokens,
        initial
    );
}

#[test]
#[ignore = "reads only aggregate metadata from the real local TRAE database"]
fn live_trae_database_record_shape() {
    let path = crate::platform::trae_database();
    let data = snapshot(&path).unwrap();
    let mut connection = rusqlite::Connection::open_in_memory().unwrap();
    connection
        .deserialize_read_exact(rusqlite::MAIN_DB, &data[..], data.len(), true)
        .unwrap();
    connection
        .execute_batch("PRAGMA query_only=ON; PRAGMA trusted_schema=OFF")
        .unwrap();
    let (turns, contexts, valid, usage): (i64, i64, i64, i64) = connection
        .query_row(
            "SELECT count(*),
                count(CASE WHEN context IS NOT NULL AND trim(context) != '' THEN 1 END),
                count(CASE WHEN context IS NOT NULL AND json_valid(context) THEN 1 END),
                count(CASE WHEN json_valid(context) AND json_type(context, '$.token_usage') = 'object' THEN 1 END)
             FROM chat_turn",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .unwrap();
    println!("TRAE chat turns={turns}, contexts={contexts}, valid JSON={valid}, token_usage objects={usage}");
}

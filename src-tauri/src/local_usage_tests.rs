use super::*;
use serde_json::json;

#[test]
fn api_cost_shares_dedup_checkpoint_and_midnight_without_altering_export() {
    let root = std::env::temp_dir().join(format!(
        "tokendaily-cost-{}-{}",
        std::process::id(),
        crate::now_ms()
    ));
    fs::create_dir_all(root.join("sessions")).unwrap();
    let context =
        json!({"type":"turn_context","payload":{"turn_id":"cost-test","model":"gpt-6.1-sol"}});
    let mut line: serde_json::Value =
        serde_json::from_slice(&event("2026-10-18T01:00:00Z", 1100, 1000)).unwrap();
    let usage = &mut line["payload"]["info"]["last_token_usage"];
    usage["output_tokens"] = json!(100);
    usage["cached_input_tokens"] = json!(500);
    usage["cache_write_input_tokens"] = json!(100);
    let bytes = format!("{context}\n{line}\n{line}\n");
    fs::write(root.join("sessions/a.jsonl"), &bytes).unwrap();
    fs::write(root.join("sessions/b.jsonl"), &bytes).unwrap();
    let now = instant("2026-10-18T02:00:00Z");
    let mut c = Collector::new(root.clone());
    let s = c.poll(now).snapshot.unwrap();
    assert_eq!(s.days[0].tokens, 1100);
    assert!((s.api_cost.unwrap().usd - 0.0021).abs() < 1e-12);
    c.checkpoint(&root.join("index")).unwrap();
    let mut restored = Collector::restore(root.clone(), &root.join("index"));
    assert!((restored.poll(now).snapshot.unwrap().api_cost.unwrap().usd - 0.0021).abs() < 1e-12);
    let next = restored
        .poll(instant("2026-10-18T16:00:00Z"))
        .snapshot
        .unwrap();
    assert_eq!(next.days[0].tokens, 0);
    assert_eq!(next.api_cost.unwrap().usd, 0.);
    let export = rank_events(root.clone(), now - 86400000, now).unwrap();
    assert_eq!(export.len(), 1);
    assert_eq!(export[0].tokens, 1100);
    assert!(!serde_json::to_string(&export).unwrap().contains("model"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn optional_pricing_metadata_cannot_break_token_accounting() {
    let mut c = Collector::new(PathBuf::new());
    c.day = "2026-10-18".into();
    let mut cursor = Cursor::default();
    let mut line: serde_json::Value =
        serde_json::from_slice(&event("2026-10-18T01:00:00Z", 100, 100)).unwrap();
    line["payload"]["info"]["last_token_usage"]["cached_input_tokens"] = json!("bad");
    c.consume(&mut cursor, &serde_json::to_vec(&line).unwrap())
        .unwrap();
    assert_eq!(c.total(), 100);
    assert_eq!(c.cost.unpriced_tokens, 100);
}

#[test]
fn rank_export_matches_daily_parser_deduplicates_and_honors_exact_rebind_time() {
    let root = std::env::temp_dir().join(format!("tokendaily-rank-{}", crate::now_ms()));
    fs::create_dir_all(root.join("sessions")).unwrap();
    let since = instant("2026-10-12T00:00:00Z");
    let until = instant("2026-10-17T04:00:00Z");
    let mut bytes =
        b"{\"type\":\"response_item\",\"payload\":{\"text\":\"private-conversation\"}}\n".to_vec();
    bytes.extend(event("2026-10-11T15:59:59Z", 40, 40));
    bytes.push(b'\n');
    bytes.extend(event("2026-10-12T00:00:00Z", 140, 100));
    bytes.push(b'\n');
    bytes.extend(event("2026-10-17T03:00:00Z", 340, 200));
    bytes.push(b'\n');
    fs::write(root.join("sessions/private-project.jsonl"), &bytes).unwrap();
    fs::write(root.join("sessions/copied.jsonl"), &bytes).unwrap();
    let exported = rank_events(root.clone(), since, until).unwrap();
    assert_eq!(exported.len(), 2);
    assert_eq!(exported.iter().map(|e| e.tokens).sum::<u64>(), 300);
    let safe = serde_json::to_string(&exported).unwrap();
    assert!(!safe.contains("private"));
    assert!(!safe.contains("sessions"));
    let rebound = rank_events(root.clone(), instant("2026-10-17T03:00:00.001Z"), until).unwrap();
    assert!(rebound.is_empty());
    let mut today = Collector::new(root.clone());
    assert_eq!(today.poll(until).snapshot.unwrap().days[0].tokens, 200);
    assert_eq!(
        exported
            .iter()
            .filter(|e| beijing_day(e.at) == beijing_day(until))
            .map(|e| e.tokens)
            .sum::<u64>(),
        200
    );
    assert!(rank_events(root.join("missing"), since, until).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "read-only real log export check; no data is uploaded"]
fn live_rank_export_matches_today() {
    let now = crate::now_ms();
    let exported = rank_events(codex_home(), now - 7 * 86400000, now).unwrap();
    let mut local = Collector::new(codex_home());
    // Active writers may append while scanning: freeze both scans at the same
    // event-time cutoff instead of comparing different live snapshots.
    let day_start = (now + 8 * 3600000).div_euclid(86400000) * 86400000 - 8 * 3600000;
    local.range = Some((day_start, now));
    let state = local.poll(now);
    assert_eq!(state.status, "available");
    let total = exported
        .iter()
        .filter(|e| beijing_day(e.at) == beijing_day(now))
        .map(|e| e.tokens)
        .sum::<u64>();
    assert_eq!(total, state.snapshot.unwrap().days[0].tokens);
    println!(
        "Real local export matches daily collector; records={}; no upload",
        exported.len()
    );
}

#[test]
fn checkpoint_reuses_offsets_validates_root_and_recovers_corruption() {
    use std::io::Write;
    let root = std::env::temp_dir().join(format!("tokendaily-index-{}", crate::now_ms()));
    fs::create_dir_all(root.join("sessions")).unwrap();
    let file = root.join("sessions/private-name.jsonl");
    let index = root.join("index.json");
    let now = instant("2026-10-16T01:00:00Z");
    let mut bytes =
        b"{\"type\":\"response_item\",\"payload\":{\"secret\":\"private-conversation\"}}\n"
            .to_vec();
    bytes.extend(event("2026-10-16T00:00:00Z", 100, 100));
    bytes.push(b'\n');
    fs::write(&file, &bytes).unwrap();
    let mut c = Collector::new(root.clone());
    assert_eq!(c.poll(now).snapshot.unwrap().days[0].tokens, 100);
    c.checkpoint(&index).unwrap();
    let saved = fs::read_to_string(&index_paths(&index)[1]).unwrap();
    assert!(!saved.contains("private-conversation"));
    assert!(!saved.contains("private-name"));
    let mut warm = Collector::restore(root.clone(), &index);
    assert_eq!(warm.poll(now).snapshot.unwrap().days[0].tokens, 100);
    assert_eq!(warm.read_bytes, 0);
    let mut extra = event("2026-10-16T00:01:00Z", 130, 30);
    extra.push(b'\n');
    File::options()
        .append(true)
        .open(&file)
        .unwrap()
        .write_all(&extra)
        .unwrap();
    assert!(warm.hot_changed());
    assert_eq!(warm.poll(now).snapshot.unwrap().days[0].tokens, 130);
    assert!(!warm.hot_changed());
    assert_eq!(warm.read_bytes, extra.len() as u64);
    warm.checkpoint(&index).unwrap(); // Preserve the last valid slot while writing the next.
    let mut next = Collector::restore(root.clone(), &index);
    assert_eq!(
        next.poll(instant("2026-10-16T16:00:00Z"))
            .snapshot
            .unwrap()
            .days[0]
            .tokens,
        0
    );
    assert_eq!(next.read_bytes, 0);
    let wrong = Collector::restore(root.join("other"), &index);
    assert!(wrong.restored.is_empty());
    fs::write(&index_paths(&index)[0], b"truncated-cache").unwrap();
    let mut recovered = Collector::restore(root.clone(), &index);
    assert_eq!(recovered.poll(now).snapshot.unwrap().days[0].tokens, 130);
    assert!(recovered.read_bytes > 0);
    let generation = recovered.generation;
    recovered.reset();
    assert_eq!(recovered.generation, generation);
    recovered.poll(now);
    recovered.checkpoint(&index).unwrap();
    assert!(Collector::restore(root.clone(), &index).generation > generation);
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "read-only check of the current user's real Codex logs"]
fn live_local_logs() {
    let mut collector = Collector::new(codex_home());
    let start = std::time::Instant::now();
    let first = collector.poll(crate::now_ms());
    println!(
        "first_scan_ms={} status={} tokens={:?} files={}",
        start.elapsed().as_millis(),
        first.status,
        first.snapshot.as_ref().map(|s| s.days[0].tokens),
        collector.cursors.len()
    );
    assert_eq!(first.status, "available");
    let start = std::time::Instant::now();
    let second = collector.poll(crate::now_ms());
    println!(
        "incremental_scan_ms={} status={} tokens={:?}",
        start.elapsed().as_millis(),
        second.status,
        second.snapshot.as_ref().map(|s| s.days[0].tokens)
    );
    assert_eq!(second.status, "available");
    assert!(second.snapshot.unwrap().days[0].tokens >= first.snapshot.unwrap().days[0].tokens);
    let index = std::env::temp_dir().join("tokendaily-benchmark-index.json");
    collector.checkpoint(&index).unwrap();
    let index_size = index_paths(&index)
        .iter()
        .filter_map(|p| fs::metadata(p).ok())
        .map(|m| m.len())
        .sum::<u64>();
    let prior = collector.total();
    let start = std::time::Instant::now();
    let mut warm = Collector::restore(codex_home(), &index);
    let result = warm.poll(crate::now_ms());
    assert_eq!(result.status, "available");
    assert!(warm.total() >= prior);
    println!(
        "warm_scan_ms={} log_bytes_read={} index_bytes={}",
        start.elapsed().as_millis(),
        warm.read_bytes,
        index_size
    );
    let start = std::time::Instant::now();
    for _ in 0..100 {
        warm.hot_changed();
    }
    println!("hot_probe_100_ms={}", start.elapsed().as_millis());
    for path in index_paths(&index) {
        let _ = fs::remove_file(path);
    }
}

fn event(time: &str, total: u64, last: u64) -> Vec<u8> {
    serde_json::to_vec(&json!({"type":"event_msg","timestamp":time,"payload":{
        "type":"token_count","info":{
            "total_token_usage":{"input_tokens":total,"output_tokens":0,"total_tokens":total},
            "last_token_usage":{"input_tokens":last,"cached_input_tokens":last/2,"output_tokens":0,"total_tokens":last}
        }}})).unwrap()
}
fn instant(s: &str) -> i64 {
    DateTime::parse_from_rfc3339(s).unwrap().timestamp_millis()
}

#[test]
fn beijing_midnight_and_year_boundary() {
    assert_eq!(beijing_day(instant("2026-10-15T15:59:59Z")), "2026-10-15");
    assert_eq!(beijing_day(instant("2026-10-15T16:00:00Z")), "2026-10-16");
    assert_eq!(beijing_day(instant("2026-12-31T16:00:00Z")), "2027-01-01");
}

#[test]
fn cumulative_repeats_cache_reset_and_copied_forks_are_not_added_twice() {
    let mut collector = Collector::new(PathBuf::new());
    collector.day = "2026-10-16".into();
    let mut file = Cursor::default();
    for line in [
        event("2026-10-15T15:59:00Z", 100, 100),
        event("2026-10-15T16:00:00Z", 160, 60),
        event("2026-10-15T16:00:01Z", 160, 60),
        event("2026-10-15T16:01:00Z", 30, 30),
    ] {
        collector.consume(&mut file, &line).unwrap();
    }
    assert_eq!(collector.total(), 90);
    let mut copied = Cursor::default();
    collector
        .consume(&mut copied, &event("2026-10-15T16:00:00Z", 160, 60))
        .unwrap();
    assert_eq!(collector.total(), 90);
    collector
        .consume(&mut copied, &event("2026-10-15T16:02:00Z", 200, 40))
        .unwrap();
    assert_eq!(collector.total(), 130);
}

#[test]
fn synthetic_context_fill_and_invalid_numbers_do_not_create_usage() {
    let mut c = Collector::new(PathBuf::new());
    c.day = "2026-10-16".into();
    let mut f = Cursor::default();
    let synthetic = json!({"type":"event_msg","timestamp":"2026-10-16T00:00:00Z","payload":{"type":"token_count","info":{
        "total_token_usage":{"input_tokens":0,"output_tokens":0,"total_tokens":1000},
        "last_token_usage":{"input_tokens":0,"output_tokens":0,"total_tokens":1000}}}});
    c.consume(&mut f, &serde_json::to_vec(&synthetic).unwrap())
        .unwrap();
    assert_eq!(c.total(), 0);
    let mut invalid = synthetic;
    invalid["payload"]["info"]["last_token_usage"]["input_tokens"] = json!(-1);
    assert!(c
        .consume(&mut f, &serde_json::to_vec(&invalid).unwrap())
        .is_err());
}

#[test]
fn incremental_files_partial_lines_restart_archive_and_midnight() {
    let root = std::env::temp_dir().join(format!(
        "tokendaily-local-{}-{}",
        std::process::id(),
        crate::now_ms()
    ));
    fs::create_dir_all(root.join("sessions")).unwrap();
    let path = root.join("sessions/test.jsonl");
    let now = instant("2026-10-16T01:00:00Z");
    let line = event("2026-10-16T00:00:00Z", 123, 123);
    let mut c = Collector::new(root.clone());
    assert_eq!(c.poll(now).snapshot.unwrap().days[0].tokens, 0);
    fs::write(&path, &line[..20]).unwrap();
    assert_eq!(c.poll(now).snapshot.unwrap().days[0].tokens, 0);
    let mut complete = line;
    complete.push(b'\n');
    fs::write(&path, &complete).unwrap();
    // Windows modification time can lag an open writer, or survive a file copy.
    File::options()
        .write(true)
        .open(&path)
        .unwrap()
        .set_times(
            std::fs::FileTimes::new()
                .set_modified(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1)),
        )
        .unwrap();
    assert_eq!(c.poll(now).snapshot.unwrap().days[0].tokens, 123);
    assert_eq!(c.poll(now).snapshot.unwrap().days[0].tokens, 123);
    assert_eq!(
        Collector::new(root.clone())
            .poll(now)
            .snapshot
            .unwrap()
            .days[0]
            .tokens,
        123
    );
    fs::create_dir_all(root.join("archived_sessions")).unwrap();
    fs::rename(&path, root.join("archived_sessions/test.jsonl")).unwrap();
    assert_eq!(c.poll(now).snapshot.unwrap().days[0].tokens, 123);
    assert_eq!(
        c.poll(instant("2026-10-16T16:00:00Z"))
            .snapshot
            .unwrap()
            .days[0]
            .tokens,
        0
    );
    // Test-created directory only, under the configured temporary directory.
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn unreadable_source_is_unknown_not_zero() {
    let mut c = Collector::new(PathBuf::from("Z:/tokendaily-nonexistent-test-source"));
    let result = c.poll(instant("2026-10-16T00:00:00Z"));
    assert_eq!(result.status, "error");
    assert!(result.snapshot.is_none());
}

#[test]
fn distinct_turns_with_identical_counts_are_separate_and_output_is_included() {
    let mut c = Collector::new(PathBuf::new());
    c.day = "2026-10-16".into();
    let mut a = Cursor {
        turn: "a".into(),
        ..Cursor::default()
    };
    let mut b = Cursor {
        turn: "b".into(),
        ..Cursor::default()
    };
    let mut value: serde_json::Value =
        serde_json::from_slice(&event("2026-10-16T00:00:00Z", 110, 100)).unwrap();
    value["payload"]["info"]["last_token_usage"]["output_tokens"] = json!(10);
    value["payload"]["info"]["last_token_usage"]["reasoning_output_tokens"] = json!(5);
    let line = serde_json::to_vec(&value).unwrap();
    c.consume(&mut a, &line).unwrap();
    c.consume(&mut b, &line).unwrap();
    assert_eq!(c.total(), 220);
}

#[test]
fn new_day_resumed_session_counts_only_new_response_and_errors_do_not_become_zero() {
    use std::io::Write;
    let root = std::env::temp_dir().join(format!(
        "tokendaily-rollover-{}-{}",
        std::process::id(),
        crate::now_ms()
    ));
    fs::create_dir_all(root.join("sessions")).unwrap();
    let path = root.join("sessions/test.jsonl");
    let mut old = event("2026-10-15T15:59:00Z", 1000, 1000);
    old.push(b'\n');
    fs::write(&path, old).unwrap();
    let mut c = Collector::new(root.clone());
    assert_eq!(
        c.poll(instant("2026-10-15T15:59:59Z"))
            .snapshot
            .unwrap()
            .days[0]
            .tokens,
        1000
    );
    assert_eq!(
        c.poll(instant("2026-10-15T16:00:00Z"))
            .snapshot
            .unwrap()
            .days[0]
            .tokens,
        0
    );
    let mut next = event("2026-10-15T16:00:01Z", 1025, 25);
    next.push(b'\n');
    File::options()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(&next)
        .unwrap();
    assert_eq!(
        c.poll(instant("2026-10-15T16:00:02Z"))
            .snapshot
            .unwrap()
            .days[0]
            .tokens,
        25
    );
    // A malformed usage event makes the source incomplete, never a healthy zero.
    fs::write(&path, b"{\"type\":\"token_count\",invalid}\n").unwrap();
    let bad = Collector::new(root.clone()).poll(instant("2026-10-16T00:00:00Z"));
    assert_eq!(bad.status, "error");
    assert!(bad.snapshot.is_none());
    fs::remove_dir_all(root).unwrap();
}

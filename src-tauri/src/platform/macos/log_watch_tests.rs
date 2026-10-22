use super::*;
use std::{
    fs,
    io::Write,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "tokendaily-watch-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path.canonicalize().unwrap())
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn changed(watch: &mut Watch) {
    assert_eq!(watch.wait(Duration::from_secs(5)), Signal::Changed);
}
fn drain(watch: &mut Watch) {
    while watch.wait(Duration::from_millis(500)) == Signal::Changed {}
}
fn append(path: &Path, text: &[u8]) {
    let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
    file.write_all(text).unwrap();
    file.sync_all().unwrap();
}

#[test]
fn native_events_cover_creation_append_archive_replace_and_rebind() {
    let a = Fixture::new();
    let b = Fixture::new();
    let wake = Wake::new().unwrap();
    let mut watch = Watch::new(wake.clone());
    watch.reset(&a.0);
    drain(&mut watch);
    // Both log roots are initially absent; their creation must wake the worker.
    fs::create_dir_all(a.0.join("sessions/2026/10/17")).unwrap();
    let file = a.0.join("sessions/2026/10/17/session.jsonl");
    fs::write(&file, b"first\n").unwrap();
    changed(&mut watch);
    drain(&mut watch);
    append(&file, b"second\n");
    changed(&mut watch);
    drain(&mut watch);
    fs::create_dir_all(a.0.join("archived_sessions")).unwrap();
    let archived = a.0.join("archived_sessions/session.jsonl");
    fs::rename(&file, &archived).unwrap();
    changed(&mut watch);
    drain(&mut watch);
    fs::write(a.0.join("replacement"), b"replacement\n").unwrap();
    fs::rename(a.0.join("replacement"), &archived).unwrap();
    changed(&mut watch);
    drain(&mut watch);
    // Unrelated source files must not force a log scan.
    fs::write(a.0.join("settings.json"), b"{}\n").unwrap();
    assert_eq!(watch.wait(Duration::from_secs(2)), Signal::Timeout);
    // Queued manual work survives stop/reset, stale source changes do not.
    wake.unpark();
    watch.reset(&b.0);
    assert_eq!(watch.wait(Duration::ZERO), Signal::Manual);
    drain(&mut watch);
    append(&archived, b"old-source\n");
    assert_eq!(watch.wait(Duration::from_secs(2)), Signal::Timeout);
    fs::create_dir_all(b.0.join("sessions")).unwrap();
    changed(&mut watch);
    drain(&mut watch);
    fs::rename(b.0.join("sessions"), b.0.join("old-sessions")).unwrap();
    fs::create_dir_all(b.0.join("sessions")).unwrap();
    changed(&mut watch);
    drain(&mut watch);
    fs::write(b.0.join("sessions/recreated.jsonl"), b"new inode\n").unwrap();
    changed(&mut watch);
    watch.stop();
    fs::write(b.0.join("sessions/after-stop.jsonl"), b"stopped\n").unwrap();
    assert_eq!(watch.wait(Duration::from_millis(700)), Signal::Timeout);
    wake.unpark();
    assert_eq!(watch.wait(Duration::ZERO), Signal::Manual);
}

#[test]
fn coalesces_events_prioritizes_manual_and_rejects_late_callbacks() {
    let root = Fixture::new();
    let wake = Wake::new().unwrap();
    let mut watch = Watch::new(wake.clone());
    watch.reset(&root.0);
    drain(&mut watch);
    let generation = wake.pending.lock().unwrap().generation;
    let event =
        || Ok(notify::Event::new(notify::EventKind::Any).add_path(root.0.join("sessions/a.jsonl")));
    for _ in 0..1000 {
        wake.receive(generation, &root.0, event());
    }
    wake.unpark();
    assert_eq!(watch.wait(Duration::ZERO), Signal::Manual);
    assert_eq!(watch.wait(Duration::ZERO), Signal::Changed);
    assert_eq!(watch.wait(Duration::ZERO), Signal::Timeout);
    wake.receive(
        generation,
        &root.0,
        Ok(notify::Event::new(notify::EventKind::Other).set_flag(notify::event::Flag::Rescan)),
    );
    assert_eq!(watch.wait(Duration::ZERO), Signal::Changed);
    wake.receive(
        generation,
        &root.0,
        Err(notify::Error::generic("synthetic failure")),
    );
    assert!(watch.warning().is_some());
    assert_eq!(watch.wait(Duration::ZERO), Signal::Changed);
    watch.stop();
    wake.receive(generation, &root.0, event());
    wake.receive(
        generation,
        &root.0,
        Err(notify::Error::generic("late failure")),
    );
    assert!(watch.warning().is_none());
    assert_eq!(watch.wait(Duration::ZERO), Signal::Timeout);
    watch.reset(&root.0.join("missing"));
    assert!(watch.warning().is_some());
    assert_eq!(watch.wait(Duration::from_millis(20)), Signal::Timeout);
    watch.reset(&root.0);
    assert!(watch.warning().is_none());
}

#[test]
fn native_event_drives_shared_collector_without_archive_double_count() {
    let root = Fixture::new();
    fs::create_dir_all(root.0.join("sessions")).unwrap();
    let wake = Wake::new().unwrap();
    let mut watch = Watch::new(wake);
    watch.reset(&root.0);
    drain(&mut watch);
    let now = chrono::DateTime::parse_from_rfc3339("2026-10-17T01:00:00Z")
        .unwrap()
        .timestamp_millis();
    let mut collector = crate::local_usage::Collector::new(root.0.clone());
    assert_eq!(collector.poll(now).snapshot.unwrap().days[0].tokens, 0);
    let event = |total: u64, last: u64| {
        format!(
            "{}\n",
            serde_json::json!({"type":"event_msg","timestamp":"2026-10-17T00:00:00Z","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":total,"output_tokens":0,"total_tokens":total},"last_token_usage":{"input_tokens":last,"output_tokens":0,"total_tokens":last}}}})
        )
    };
    let file = root.0.join("sessions/usage.jsonl");
    fs::write(&file, event(100, 100)).unwrap();
    changed(&mut watch);
    assert_eq!(collector.poll(now).snapshot.unwrap().days[0].tokens, 100);
    drain(&mut watch);
    append(&file, event(150, 50).as_bytes());
    changed(&mut watch);
    assert_eq!(collector.poll(now).snapshot.unwrap().days[0].tokens, 150);
    drain(&mut watch);
    fs::create_dir_all(root.0.join("archived_sessions")).unwrap();
    fs::rename(&file, root.0.join("archived_sessions/usage.jsonl")).unwrap();
    changed(&mut watch);
    assert_eq!(collector.poll(now).snapshot.unwrap().days[0].tokens, 150);
}

#[test]
fn does_not_follow_log_root_symlinks() {
    let root = Fixture::new();
    let outside = Fixture::new();
    std::os::unix::fs::symlink(&outside.0, root.0.join("sessions")).unwrap();
    let mut watch = Watch::new(Wake::new().unwrap());
    watch.reset(&root.0);
    assert_eq!(watch.roots.len(), 1);
    drain(&mut watch);
    fs::write(outside.0.join("outside.jsonl"), b"outside\n").unwrap();
    assert_eq!(watch.wait(Duration::from_secs(2)), Signal::Timeout);
}

#[test]
fn unchanged_reset_reuses_stream_and_drop_releases_callback() {
    let root = Fixture::new();
    let wake = Wake::new().unwrap();
    let weak = Arc::downgrade(&wake);
    let mut watch = Watch::new(wake.clone());
    watch.reset(&root.0);
    let generation = wake.pending.lock().unwrap().generation;
    watch.reset(&root.0);
    assert_eq!(wake.pending.lock().unwrap().generation, generation);
    let manual = wake.clone();
    let thread = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(50));
        manual.unpark();
    });
    assert_eq!(watch.wait(Duration::from_secs(3)), Signal::Manual);
    thread.join().unwrap();
    drop(wake);
    drop(watch);
    assert!(
        weak.upgrade().is_none(),
        "native callbacks must be released on drop"
    );
}

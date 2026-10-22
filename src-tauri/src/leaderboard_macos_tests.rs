//! Explicit, local-only lifecycle test. Never sends real usage or touches user state.
use super::*;
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    sync::Arc,
};

#[test]
#[ignore = "requires TOKENDAILY_RANK_LOCAL=http://127.0.0.1:8787; synthetic server and isolated Keychain"]
fn local_trial_consent_restart_sync_pause_delete() {
    assert_eq!(origin(), "http://127.0.0.1:8787", "local-only guard");
    let listener = TcpListener::bind("127.0.0.1:8787").unwrap();
    listener.set_nonblocking(true).unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let server_stop = stop.clone();
    let calls = Arc::new(Mutex::new(Vec::<(String, Value)>::new()));
    let server_calls = calls.clone();
    let server = std::thread::spawn(move || {
        let mut enrollment = Value::Null;
        let mut device = String::new();
        while !server_stop.load(Ordering::SeqCst) {
            let Ok((mut stream, _)) = listener.accept() else {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            };
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut bytes = Vec::new();
            let (end, length) = loop {
                let mut block = [0; 4096];
                let n = stream.read(&mut block).unwrap();
                assert!(n > 0);
                bytes.extend_from_slice(&block[..n]);
                if let Some(end) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&bytes[..end]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            line.to_lowercase()
                                .strip_prefix("content-length: ")
                                .and_then(|s| s.parse::<usize>().ok())
                        })
                        .unwrap_or(0);
                    break (end + 4, length);
                }
            };
            while bytes.len() < end + length {
                let mut block = [0; 4096];
                let n = stream.read(&mut block).unwrap();
                assert!(n > 0);
                bytes.extend_from_slice(&block[..n]);
            }
            let headers = String::from_utf8_lossy(&bytes[..end]);
            let path = headers
                .lines()
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_owned();
            let body: Value = if length == 0 {
                Value::Null
            } else {
                serde_json::from_slice(&bytes[end..end + length]).unwrap()
            };
            assert!(!String::from_utf8_lossy(&bytes[end..]).contains("private-conversation"));
            match path.as_str() {
                "/api/trial/start" => device = body["device"].as_str().unwrap().into(),
                "/api/member/enroll" => {
                    assert_eq!(body["confirm"], true);
                    enrollment = json!({"device":device,"epoch":"synthetic-epoch","active":1,"since_ms":week_start(now())});
                }
                "/api/member/pause" => {
                    enrollment["active"] = json!(if body["active"] == true { 1 } else { 0 })
                }
                "/api/trial/delete" => enrollment = Value::Null,
                "/api/member/status" | "/api/member/upload" => (),
                _ => panic!("unexpected endpoint"),
            }
            server_calls.lock().unwrap().push((path, body));
            let response = json!({"enrollment":enrollment}).to_string();
            write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",response.len(),response).unwrap();
        }
    });
    let root = std::env::temp_dir().join(format!("tokendaily-local-rank-{}", random_id().unwrap()));
    fs::create_dir_all(root.join("sessions")).unwrap();
    let at = Utc.timestamp_millis_opt(now() - 1000).unwrap().to_rfc3339();
    let record = json!({"timestamp":at,"type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":100,"output_tokens":23,"total_tokens":123},"last_token_usage":{"total_tokens":123,"input_tokens":100,"output_tokens":23}}}});
    fs::write(root.join("sessions/test.jsonl"), format!("{{\"type\":\"response_item\",\"payload\":{{\"text\":\"private-conversation\"}}}}\n{record}\n")).unwrap();
    let service = Service::new(&root);
    // Always clean up even if an assertion fails.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let act = |svc: &Service, action, body| svc.action(action, body, Ok(root.clone())).unwrap();
        assert_eq!(act(&service, "status", json!({}))["loggedIn"], false);
        assert!(calls.lock().unwrap().is_empty());
        let started = act(&service, "start", json!({}));
        assert_eq!(started["loggedIn"], true);
        assert_eq!(started["enabled"], false);
        let preview = act(&service, "preview", json!({}));
        assert_eq!(preview["total"], 123);
        assert!(!calls
            .lock()
            .unwrap()
            .iter()
            .any(|(path, _)| path.ends_with("upload")));
        let enrolled = act(
            &service,
            "enroll",
            json!({"key":preview["key"],"confirm":true,"nickname":"synthetic-local"}),
        );
        assert_eq!(enrolled["enabled"], true);
        let restored = Service::new(&root);
        let status = act(&restored, "status", json!({}));
        assert_eq!(status["thisDevice"], started["thisDevice"]);
        assert_eq!(status["enabled"], true);
        act(&restored, "sync", json!({}));
        let uploads: Vec<_> = calls
            .lock()
            .unwrap()
            .iter()
            .filter(|(path, _)| path.ends_with("upload"))
            .map(|(_, b)| b["events"].as_array().unwrap().len())
            .collect();
        assert_eq!(uploads, vec![1, 0], "restart retains dedup checkpoints");
        assert_eq!(act(&restored, "pause", json!({}))["enabled"], false);
        assert!(restored
            .action("sync", json!({}), Ok(root.clone()))
            .is_err());
        assert_eq!(act(&restored, "resume", json!({}))["enabled"], true);
        assert_eq!(
            act(&restored, "delete", json!({"confirm":true}))["loggedIn"],
            false
        );
        assert!(
            !service.path.exists(),
            "Mac must not write plaintext credentials"
        );
    }));
    let cleaned = service.clear_local();
    stop.store(true, Ordering::SeqCst);
    let joined = server.join();
    fs::remove_dir_all(root).unwrap();
    cleaned.unwrap();
    joined.unwrap();
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}

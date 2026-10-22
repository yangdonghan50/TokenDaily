use super::*;
use serde_json::json;
#[cfg(target_os = "windows")]
use std::fs;

#[test]
fn stored_authorization_distinguishes_missing_denied_corrupt_and_valid() {
    let (missing, credential) = restore_authorization(Ok(None));
    assert!(missing.supported);
    assert_eq!(missing.status, "auth-required");
    assert!(credential.is_none());
    for input in [
        Err("安全存储拒绝访问"),
        Ok(Some(b"not-json".to_vec())),
        Ok(Some(
            br#"{"token":"test","domain":"evil.test","identity":"test","expires_at_ms":null}"#
                .to_vec(),
        )),
    ] {
        let (state, credential) = restore_authorization(input);
        assert_eq!(state.status, "error");
        assert!(!state.authorized);
        assert!(state.balance.is_none());
        assert!(credential.is_none());
    }
    let bytes = serde_json::to_vec(&Credential {
        token: "synthetic-only".into(),
        domain: "www.codebuddy.cn".into(),
        identity: "test".into(),
        expires_at_ms: None,
    })
    .unwrap();
    let (state, credential) = restore_authorization(Ok(Some(bytes)));
    assert_eq!(state.status, "loading");
    assert!(state.authorized);
    assert!(state.balance.is_none());
    assert!(credential.is_some());
}

#[test]
fn startup_restore_is_visible_before_keychain_returns() {
    let state = restoring_state();
    assert_eq!(state.status, "loading");
    assert!(state.refreshing);
    assert!(!state.authorized);
    assert!(state.balance.is_none());
    assert_eq!(state.message, "正在恢复本机已保存的积分授权");
}

#[test]
fn automatic_restore_runs_once_but_manual_retry_remains_available() {
    assert!(should_restore(false, false, false, false));
    assert!(!should_restore(false, true, false, false));
    assert!(should_restore(true, true, false, false));
    assert!(!should_restore(true, false, true, false));
    assert!(!should_restore(true, false, false, true));
}

#[test]
#[cfg(target_os = "macos")]
fn mac_last_balance_is_scoped_to_credential_without_keychain_access() {
    let path = std::env::temp_dir().join(format!("tokendaily-wb-cache-{}", crate::now_ms()));
    let credential = Credential {
        token: "synthetic-token-a".into(),
        domain: "www.codebuddy.cn".into(),
        identity: "synthetic-account".into(),
        expires_at_ms: None,
    };
    let balance = parse_balance(
        &payload(json!([{"CapacityRemainPrecise":"12.05"}])),
        crate::now_ms(),
    )
    .unwrap();
    crate::credit_cache::save_last(&path, &cache_scope(&credential), &balance).unwrap();
    assert_eq!(
        cached_balance(&path, &credential).unwrap().remaining,
        "12.05"
    );
    assert!(cached_balance(
        &path,
        &Credential {
            token: "other".into(),
            ..credential.clone()
        }
    )
    .is_none());
    assert!(cached_balance(
        &path,
        &Credential {
            expires_at_ms: Some(crate::now_ms() - 1),
            ..credential
        }
    )
    .is_none());
    crate::credit_cache::clear_last(&path).unwrap();
}

#[test]
#[cfg(target_os = "windows")]
fn last_success_restores_as_stale_only_for_same_authorization() {
    let root = std::env::temp_dir().join(format!("tokendaily-workbuddy-last-{}", crate::now_ms()));
    fs::create_dir_all(&root).unwrap();
    let credential = Credential {
        token: "synthetic-token-a".into(),
        domain: "www.codebuddy.cn".into(),
        identity: "synthetic-account".into(),
        expires_at_ms: None,
    };
    fs::write(
        root.join("workbuddy-credits.protected"),
        crate::platform::protect_secret(&serde_json::to_vec(&credential).unwrap()).unwrap(),
    )
    .unwrap();
    let balance = parse_balance(
        &payload(json!([{"CapacityRemainPrecise":"12.05"}])),
        crate::now_ms() - 60_000,
    )
    .unwrap();
    crate::credit_cache::save_last(
        &root.join("workbuddy-last-balance.protected"),
        &cache_scope(&credential),
        &balance,
    )
    .unwrap();
    let restored = Credits::new(&root);
    let state = restored.inner.lock().unwrap().state.clone();
    assert_eq!(state.status, "stale");
    assert_eq!(state.balance.as_ref().unwrap().remaining, "12.05");
    drop(restored);
    let other = Credential {
        token: "synthetic-token-b".into(),
        ..credential
    };
    fs::write(
        root.join("workbuddy-credits.protected"),
        crate::platform::protect_secret(&serde_json::to_vec(&other).unwrap()).unwrap(),
    )
    .unwrap();
    assert!(Credits::new(&root)
        .inner
        .lock()
        .unwrap()
        .state
        .balance
        .is_none());
    let expired = Credential {
        token: "synthetic-token-a".into(),
        expires_at_ms: Some(crate::now_ms() - 1),
        ..other
    };
    fs::write(
        root.join("workbuddy-credits.protected"),
        crate::platform::protect_secret(&serde_json::to_vec(&expired).unwrap()).unwrap(),
    )
    .unwrap();
    assert!(Credits::new(&root)
        .inner
        .lock()
        .unwrap()
        .state
        .balance
        .is_none());
    let current = Credential {
        expires_at_ms: None,
        ..expired
    };
    fs::write(
        root.join("workbuddy-credits.protected"),
        crate::platform::protect_secret(&serde_json::to_vec(&current).unwrap()).unwrap(),
    )
    .unwrap();
    let mut future = balance;
    future.collected_at_ms = crate::now_ms() + 120_000;
    crate::credit_cache::save_last(
        &root.join("workbuddy-last-balance.protected"),
        &cache_scope(&current),
        &future,
    )
    .unwrap();
    assert!(Credits::new(&root)
        .inner
        .lock()
        .unwrap()
        .state
        .balance
        .is_none());
    fs::remove_dir_all(root).unwrap();
}

fn payload(rows: Value) -> Value {
    json!({"data":{"Packages":rows}})
}
#[test]
fn precise_balance_expiry_and_unknown_are_distinct() {
    let v = payload(json!([
        {"ResourceId":"a","CapacityRemainPrecise":"0.1","CapacityRemain":999},
        {"ResourceId":"b","CapacityRemainPrecise":"0.2"},
        {"ResourceId":"old","CapacityRemainPrecise":"50","ExpiredTime":1700000000}
    ]));
    let result = parse_balance(&v, 1800000000000).unwrap();
    assert_eq!(result.remaining, "0.3");
    assert_eq!(result.packages.len(), 2);
    assert_eq!(
        parse_balance(&payload(json!([])), 0).unwrap().remaining,
        "0"
    );
    assert!(parse_balance(&json!({"data":{}}), 0).is_err());
    for bad in [
        json!(null),
        json!(""),
        json!(-1),
        json!(true),
        json!("NaN"),
        json!("0.1234567"),
    ] {
        assert!(parse_balance(&payload(json!([{"CapacityRemainPrecise":bad}])), 0).is_err());
    }
}
#[test]
fn conflicting_duplicates_and_incomplete_pages_are_rejected() {
    let row = json!({"ResourceId":"a","CapacityRemainPrecise":"2.25"});
    assert_eq!(
        parse_balance(&payload(json!([row, row])), 0)
            .unwrap()
            .remaining,
        "2.25"
    );
    assert!(parse_balance(
        &payload(json!([row,{"ResourceId":"a","CapacityRemain":3}])),
        0
    )
    .is_err());
    assert!(parse_balance(&json!({"data":{"Packages":[],"TotalCount":2}}), 0).is_err());
    assert!(parse_balance(
        &payload(json!([{"CapacityRemain":null,"CapacityUsed":0,"CapacitySize":100}])),
        0
    )
    .is_err());
}
#[test]
fn dates_are_expiry_not_reset_and_no_ambiguous_timezone() {
    assert_eq!(timestamp(&json!(1800000000)).unwrap(), Some(1800000000000));
    assert_eq!(
        timestamp(&json!("2026-10-17T00:00:00+08:00")).unwrap(),
        Some(1792166400000)
    );
    assert!(timestamp(&json!("2026-10-17 00:00:00")).is_err());
    let v = payload(
        json!([{"CapacityRemain":1,"CycleEndTime":1800000001,"DeductionEndTime":1900000000}]),
    );
    assert_eq!(
        parse_balance(&v, 1800000000000).unwrap().next_expiry_at_ms,
        Some(1800000001000)
    );
}
#[test]
fn auth_urls_allow_only_official_https_hosts() {
    assert!(login_url("https://www.codebuddy.cn/login?state=test").is_ok());
    for url in [
        "http://www.codebuddy.cn/login",
        "https://www.codebuddy.cn.evil.test/",
        "https://user@www.codebuddy.cn/",
        "https://www.codebuddy.cn:444/",
        "file:///test",
    ] {
        assert!(login_url(url).is_err());
    }
}
#[test]
fn throttling_and_epoch_protect_state() {
    assert!(!due(1000, 1000, false, false));
    assert!(due(301000, 1000, false, false));
    assert!(!due(10999, 1000, true, false));
    assert!(due(11000, 1000, true, false));
    assert!(!due(10999, 1000, true, true));
    assert!(due(11000, 1000, true, true));
    assert!(!due(60999, 1000, false, true));
    assert!(due(61000, 1000, false, true));
    assert!(!accept(3, 2));
    assert!(accept(3, 3));
}

#[test]
fn failures_retain_only_safe_history_and_auth_failure_clears_identity_data() {
    let mut state = CreditState::default();
    state.authorized = true;
    state.status = "available".into();
    state.balance =
        Some(parse_balance(&payload(json!([{"CapacityRemain":"12.05"}])), 1000).unwrap());
    let mut inner = Inner {
        state,
        credential: Some(Credential {
            token: "test-only".into(),
            domain: "www.codebuddy.cn".into(),
            identity: "test".into(),
            expires_at_ms: None,
        }),
        ticket: None,
        restore_attempted: true,
        restoring: false,
        generation: 1,
        last_attempt: 1000,
        failed: false,
        auth_failures: 0,
    };
    assert!(!fail(&mut inner, "网络失败", false));
    assert_eq!(inner.state.status, "stale");
    assert_eq!(inner.state.balance.as_ref().unwrap().remaining, "12.05");
    assert_eq!(inner.state.balance.as_ref().unwrap().collected_at_ms, 1000);
    assert!(!fail(&mut inner, "授权失效，请重新登录", false));
    assert_eq!(inner.state.status, "stale");
    assert!(inner.credential.is_some());
    assert!(inner.state.message.contains("复核"));
    assert!(fail(&mut inner, "授权失效，请重新登录", false));
    assert!(inner.state.balance.is_none());
    assert!(inner.credential.is_none());
    assert!(!inner.state.authorized);
    assert_eq!(inner.state.status, "auth-required");
}

#[test]
fn network_failure_breaks_auth_failure_confirmation_chain() {
    let mut inner = Inner {
        state: CreditState::default(),
        credential: None,
        ticket: None,
        restore_attempted: false,
        restoring: false,
        generation: 0,
        last_attempt: 0,
        failed: false,
        auth_failures: 0,
    };
    assert!(!fail(&mut inner, "授权失效，请重新登录", false));
    assert_eq!(inner.auth_failures, 1);
    assert!(!fail(&mut inner, "网络失败", false));
    assert_eq!(inner.auth_failures, 0);
    assert!(!fail(&mut inner, "授权失效，请重新登录", false));
    assert!(fail(&mut inner, "授权失效，请重新登录", true));
}

#[test]
#[cfg(target_os = "windows")]
fn confirmed_invalid_authorization_removes_own_saved_files() {
    let root =
        std::env::temp_dir().join(format!("tokendaily-workbuddy-invalid-{}", crate::now_ms()));
    fs::create_dir_all(&root).unwrap();
    let s = Credits::new(&root);
    fs::write(&s.path, b"synthetic-credential").unwrap();
    fs::write(&s.cache_path, b"synthetic-balance").unwrap();
    let mut i = s.inner.lock().unwrap();
    i.state.identity_label = Some("synthetic-account".into());
    forget_invalid_credential(&s, &mut i);
    assert!(i.state.identity_label.is_none());
    assert!(!s.path.exists());
    assert!(!s.cache_path.exists());
    drop(i);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn summary_groups_are_not_mistaken_for_individual_packages_or_pages() {
    // Synthetic values, matching the field structure observed in the live summary.
    let v = payload(json!([
        {"PackageCode":"group-a","CycleRemainCapacity":"8.5","TotalCount":6},
        {"PackageCode":"group-b","CycleRemainCapacity":"12.75","TotalCount":1}
    ]));
    let result = parse_balance(&v, 1000).unwrap();
    assert_eq!(result.remaining, "21.25");
    assert_eq!(result.packages[0].name, "积分汇总（6 项）");
    assert_eq!(result.next_expiry_at_ms, None);
}

use super::*;
use std::time::{Duration, Instant};

#[test]
fn checks_are_single_flight_and_manual_checks_are_throttled() {
    let now = Instant::now();
    let mut state = UpdateState::new("0.1.23");
    assert!(state.begin_check(now, false));
    assert!(!state.begin_check(now + Duration::from_secs(61), false));
    state.finish_check(None, None);
    assert!(!state.begin_check(now + Duration::from_secs(59), false));
    assert!(state.begin_check(now + Duration::from_secs(60), false));
    state.finish_check(None, None);
    assert!(!state.begin_check(now + Duration::from_secs(61), true));
    assert!(state.begin_check(now + Duration::from_secs(60 + 6 * 3600), true));
}

#[test]
fn failures_preserve_an_available_update_but_never_claim_current() {
    let mut state = UpdateState::new("0.1.23");
    let now = Instant::now();
    state.begin_check(now, false);
    state.finish_check(None, Some("检查失败"));
    assert_eq!(state.snapshot.phase, Phase::CheckError);
    assert!(!state.begin_install());
    state.begin_check(now + Duration::from_secs(60), false);
    state.finish_check(Some("0.1.24"), None);
    assert!(state.begin_install());
    assert!(!state.begin_install());
    assert!(!state.begin_check(now + Duration::from_secs(120), false));
    state.progress(50, Some(100));
    assert_eq!(state.snapshot.downloaded, 50);
    state.fail_install("校验失败");
    assert_eq!(state.snapshot.phase, Phase::InstallError);
    assert_eq!(state.snapshot.latest_version.as_deref(), Some("0.1.24"));
    assert!(state.begin_install());
    assert_eq!(state.snapshot.downloaded, 0);
}

#[test]
fn check_failure_does_not_drop_an_existing_retry_and_progress_cannot_overflow() {
    let mut state = UpdateState::new("0.1.23");
    let now = Instant::now();
    state.begin_check(now, false);
    state.finish_check(Some("0.1.24"), None);
    state.begin_check(now + Duration::from_secs(60), false);
    state.finish_check(None, Some("网络失败"));
    assert_eq!(state.snapshot.phase, Phase::Available);
    assert_eq!(state.snapshot.latest_version.as_deref(), Some("0.1.24"));
    assert!(state.snapshot.message.is_some());
    state.begin_install();
    state.progress(usize::MAX, None);
    state.progress(usize::MAX, Some(0));
    assert_eq!(state.snapshot.downloaded, u64::MAX);
    assert_eq!(state.snapshot.total, None);
    state.ready_to_install();
    assert_eq!(state.snapshot.phase, Phase::Installing);
    assert!(!state.begin_install());
}

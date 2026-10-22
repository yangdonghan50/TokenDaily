//! Shared notification contracts; no operating-system types or inferred delivery.
use crate::alerts::SubmissionError;

pub(crate) fn matches_current(
    current: &crate::codex::DesktopState,
    expected: &crate::codex::QuotaSnapshot,
) -> bool {
    current.connected
        && current.identity_verified
        && current.status == "available"
        && current.snapshot.as_ref().is_some_and(|snapshot| {
            snapshot.status == "available"
                && snapshot.identity_key == expected.identity_key
                && snapshot.collected_at_ms == expected.collected_at_ms
        })
}
use serde::Serialize;

#[derive(Default)]
pub(crate) struct WorkerGate {
    running: bool,
    next_epoch: Option<u32>,
}
impl WorkerGate {
    pub fn request(&mut self, epoch: u32) -> bool {
        self.next_epoch = Some(epoch);
        if self.running {
            false
        } else {
            self.running = true;
            true
        }
    }
    pub fn take(&mut self) -> Option<u32> {
        let next = self.next_epoch.take();
        if next.is_none() {
            self.running = false;
        }
        next
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Permission {
    pub supported: bool,
    pub authorization: &'static str,
    pub alerts: Option<bool>,
    pub center: Option<bool>,
}
impl Permission {
    #[cfg(any(target_os = "windows", test))]
    pub fn unknown(supported: bool) -> Self {
        Self {
            supported,
            authorization: "unknown",
            alerts: None,
            center: None,
        }
    }
    pub fn can_submit(&self) -> bool {
        self.supported
            && matches!(self.authorization, "authorized" | "provisional")
            && (self.alerts == Some(true) || self.center == Some(true))
    }
}

#[derive(Debug)]
pub(crate) struct Failure {
    pub kind: SubmissionError,
    pub message: &'static str,
}
impl Failure {
    pub fn rejected(message: &'static str) -> Self {
        Self {
            kind: SubmissionError::Rejected,
            message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queued_alert_cannot_use_a_failed_disconnected_or_superseded_snapshot() {
        let snapshot = crate::codex::QuotaSnapshot {
            identity_key: "a".into(),
            status: "available".into(),
            collected_at_ms: 10,
            windows: vec![],
        };
        let mut current = crate::codex::DesktopState::disconnected();
        current.status = "available".into();
        current.connected = true;
        current.identity_verified = true;
        current.snapshot = Some(snapshot.clone());
        assert!(matches_current(&current, &snapshot));
        current.status = "stale".into();
        assert!(!matches_current(&current, &snapshot));
        current.status = "available".into();
        current.connected = false;
        assert!(!matches_current(&current, &snapshot));
        current.connected = true;
        current.snapshot.as_mut().unwrap().collected_at_ms = 11;
        assert!(!matches_current(&current, &snapshot));
        current.snapshot = Some(snapshot.clone());
        current.snapshot.as_mut().unwrap().identity_key = "b".into();
        assert!(!matches_current(&current, &snapshot));
    }
    #[test]
    fn one_worker_coalesces_pending_refreshes_and_can_restart_after_draining() {
        let mut gate = WorkerGate::default();
        assert!(gate.request(1));
        assert_eq!(gate.take(), Some(1));
        assert!(!gate.request(2));
        assert!(!gate.request(3));
        assert_eq!(gate.take(), Some(3));
        assert_eq!(gate.take(), None);
        assert!(gate.request(4));
        assert_eq!(gate.take(), Some(4));
        assert_eq!(gate.take(), None);
    }
    #[test]
    fn permission_requires_observed_authorization_and_a_visible_channel() {
        let mut p = Permission::unknown(true);
        p.alerts = Some(true);
        for status in [
            "unknown",
            "not-determined",
            "denied",
            "unsupported",
            "future-value",
        ] {
            p.authorization = status;
            assert!(!p.can_submit());
        }
        p.authorization = "authorized";
        assert!(p.can_submit());
        p.alerts = Some(false);
        assert!(!p.can_submit());
        p.center = Some(true);
        assert!(p.can_submit());
        p.authorization = "provisional";
        assert!(p.can_submit());
        p.supported = false;
        assert!(!p.can_submit());
    }
}

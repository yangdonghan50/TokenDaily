use crate::codex::QuotaSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlertSettings {
    pub low_enabled: bool,
    pub recovery_enabled: bool,
    pub low_threshold: f64,
}

impl Default for AlertSettings {
    fn default() -> Self {
        Self {
            low_enabled: true,
            recovery_enabled: true,
            low_threshold: 20.0,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlertLedger {
    identity_key: String,
    windows: BTreeMap<String, SeenWindow>,
    #[serde(default)]
    history: BTreeMap<String, BTreeMap<String, SeenWindow>>,
    #[serde(default)]
    retry_after_ms: i64,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SeenWindow {
    low_sent: bool, // Legacy wire name; means reserved/submitted, not delivered.
    #[serde(default)]
    recovery_pending: bool,
    was_restricted: bool,
    last_seen_at_ms: i64,
    last_reset_at_ms: Option<i64>,
}

pub(crate) struct AlertEvent {
    pub title: String,
    pub body: String,
}

pub(crate) fn evaluate(
    previous: &AlertLedger,
    snapshot: &QuotaSnapshot,
    settings: &AlertSettings,
    now_ms: i64,
) -> (AlertLedger, Option<AlertEvent>) {
    let age = now_ms.saturating_sub(snapshot.collected_at_ms);
    if snapshot.status != "available" || !(-5000..=600_000).contains(&age) {
        return (previous.clone(), None);
    }
    let mut ledger = previous.clone();
    if ledger.identity_key != snapshot.identity_key {
        if !ledger.identity_key.is_empty() {
            ledger.history.insert(
                ledger.identity_key.clone(),
                std::mem::take(&mut ledger.windows),
            );
        }
        ledger.windows = ledger
            .history
            .remove(&snapshot.identity_key)
            .unwrap_or_default();
        ledger.identity_key = snapshot.identity_key.clone();
        ledger.retry_after_ms = 0;
    }
    let mut low_count = 0;
    let mut recovery_count = 0;
    for window in &snapshot.windows {
        let Some(remaining) = window.remaining_percent else {
            continue;
        };
        if !remaining.is_finite() || !(0.0..=100.0).contains(&remaining) {
            continue;
        }
        if window.resets_at_ms.is_some_and(|reset| reset <= now_ms) {
            continue;
        }
        let key = serde_json::to_string(&(&window.pool_id, &window.window_id)).expect("window key");
        let old = ledger.windows.get(&key);
        if old.is_some_and(|seen| snapshot.collected_at_ms <= seen.last_seen_at_ms) {
            continue;
        }
        let restricted = remaining == 0.0 || window.reached_type.is_some();
        let mut low_sent = old.is_some_and(|seen| seen.low_sent);
        let cycle_advanced = old
            .and_then(|seen| seen.last_reset_at_ms)
            .zip(window.resets_at_ms)
            .is_some_and(|(old_reset, new_reset)| {
                new_reset.saturating_sub(old_reset)
                    >= window
                        .duration_minutes
                        .unwrap_or(60)
                        .saturating_mul(30_000)
                        .max(300_000)
            });
        if cycle_advanced && remaining > settings.low_threshold + 5.0 {
            low_sent = false;
        }
        let recovered = old.is_some_and(|seen| seen.was_restricted || seen.recovery_pending)
            && !restricted
            && remaining > 0.0;
        if recovered && settings.recovery_enabled {
            recovery_count += 1;
        }
        if settings.low_enabled && remaining <= settings.low_threshold && !low_sent {
            low_count += 1;
            low_sent = true;
        }
        ledger.windows.insert(
            key,
            SeenWindow {
                low_sent,
                recovery_pending: false,
                was_restricted: restricted,
                last_seen_at_ms: snapshot.collected_at_ms,
                last_reset_at_ms: window.resets_at_ms,
            },
        );
    }
    let still_restricted = snapshot
        .windows
        .iter()
        .any(|window| window.remaining_percent == Some(0.0) || window.reached_type.is_some());
    let event = match (recovery_count, low_count) {
        (0, 0) => None,
        (recovered, low) if recovered > 0 && low > 0 => Some(AlertEvent {
            title: "Codex 额度状态有变化".into(),
            body: format!("{recovered} 个窗口恢复，{low} 个窗口额度偏低；请查看详情。"),
        }),
        (recovered, _) if recovered > 0 => Some(AlertEvent {
            title: "Codex 额度已确认恢复".into(),
            body: if still_restricted {
                format!("{recovered} 个窗口恢复，其他窗口仍可能受限。")
            } else {
                format!("{recovered} 个窗口恢复，请查看当前额度。")
            },
        }),
        (_, low) => Some(AlertEvent {
            title: "Codex 额度偏低".into(),
            body: format!("{low} 个窗口低于剩余 {}% 阈值。", settings.low_threshold),
        }),
    };
    (ledger, event)
}

/// Durable reservation precedes the external side effect. A pending record after
/// a crash is ambiguous and is conservatively treated as submitted, never replayed.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Record {
    ledger: AlertLedger,
    pending: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SubmissionError {
    Rejected,
    Uncertain,
}

pub(crate) struct Delivery {
    ledger: AlertLedger,
    pub status: String,
    blocked: bool,
}
impl Default for Delivery {
    fn default() -> Self {
        Self {
            ledger: AlertLedger::default(),
            status: "idle".into(),
            blocked: false,
        }
    }
}
impl Delivery {
    pub(crate) fn restore(record: Record) -> Self {
        let status = if record.pending {
            "uncertain"
        } else if record.ledger.retry_after_ms > 0 {
            "retry"
        } else {
            "idle"
        };
        Self {
            ledger: record.ledger,
            status: status.into(),
            blocked: false,
        }
    }
    pub(crate) fn blocked() -> Self {
        Self {
            status: "storage-error".into(),
            blocked: true,
            ..Self::default()
        }
    }
    pub(crate) fn identity_unverified(&mut self) {
        if !self.blocked {
            self.status = "identity-unverified".into();
        }
    }
    pub(crate) fn process(
        &mut self,
        snapshot: &QuotaSnapshot,
        settings: &AlertSettings,
        now: i64,
        enabled: bool,
        mut persist: impl FnMut(&Record) -> Result<(), ()>,
        mut submit: impl FnMut(&AlertEvent) -> Result<(), SubmissionError>,
    ) {
        if self.blocked {
            return;
        }
        let (mut submitted, event) = evaluate(&self.ledger, snapshot, settings, now);
        let mut observed = submitted.clone();
        // Separate observed restrictions from notification submission, so a failed
        // low alert cannot erase the evidence required by a later recovery.
        let old_windows = if self.ledger.identity_key == snapshot.identity_key {
            Some(&self.ledger.windows)
        } else {
            self.ledger.history.get(&snapshot.identity_key)
        };
        if event.is_some() {
            for (key, window) in &mut observed.windows {
                if window.last_seen_at_ms != snapshot.collected_at_ms {
                    continue;
                }
                let old = old_windows.and_then(|windows| windows.get(key));
                if !old.is_some_and(|w| w.low_sent) {
                    window.low_sent = false;
                }
                if settings.recovery_enabled
                    && !window.was_restricted
                    && old.is_some_and(|w| w.was_restricted || w.recovery_pending)
                {
                    window.recovery_pending = true;
                }
            }
        }
        if !enabled || event.is_none() || now < submitted.retry_after_ms {
            self.ledger = observed;
            if persist(&Record {
                ledger: self.ledger.clone(),
                pending: false,
            })
            .is_err()
            {
                self.status = "storage-error".into();
            } else if !enabled {
                self.status = "disabled".into();
            } else if ["identity-unverified", "disabled", "storage-error"]
                .contains(&self.status.as_str())
            {
                self.status = "idle".into();
            }
            return;
        }
        let event = event.expect("event");
        submitted.retry_after_ms = 0;
        self.ledger = observed;
        if persist(&Record {
            ledger: submitted.clone(),
            pending: true,
        })
        .is_err()
        {
            self.status = "storage-error".into();
            return;
        }
        let outcome = submit(&event);
        if outcome == Err(SubmissionError::Uncertain) {
            // A timeout is not evidence of rejection. Preserve the durable pending
            // reservation and consume it in memory too, even if a late success arrives.
            self.ledger = submitted;
            self.status = "uncertain".into();
            return;
        }
        if outcome == Err(SubmissionError::Rejected) {
            self.ledger.retry_after_ms = now.saturating_add(60_000);
            self.status = if persist(&Record {
                ledger: self.ledger.clone(),
                pending: false,
            })
            .is_ok()
            {
                "retry"
            } else {
                "uncertain"
            }
            .into();
            return;
        }
        self.ledger = submitted;
        self.status = if persist(&Record {
            ledger: self.ledger.clone(),
            pending: false,
        })
        .is_ok()
        {
            "submitted"
        } else {
            "uncertain"
        }
        .into();
    }
}

#[derive(Serialize, Deserialize)]
struct Envelope {
    version: u32,
    sequence: u64,
    payload: String,
    digest: String,
}
pub(crate) fn ledger_paths(path: &std::path::Path) -> [std::path::PathBuf; 2] {
    [path.with_extension("a.json"), path.with_extension("b.json")]
}
fn slots(path: &std::path::Path) -> Result<Vec<(u64, Record)>, ()> {
    use sha2::{Digest, Sha256};
    let mut result = Vec::new();
    let mut exists = false;
    for slot in ledger_paths(path) {
        let metadata = match std::fs::metadata(&slot) {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => return Err(()),
        };
        exists = true;
        if metadata.len() > 1_048_576 {
            continue;
        }
        let record = (|| {
            let bytes = std::fs::read(slot).ok()?;
            let envelope: Envelope = serde_json::from_slice(&bytes).ok()?;
            if envelope.version != 1
                || envelope.digest != format!("{:x}", Sha256::digest(envelope.payload.as_bytes()))
            {
                return None;
            }
            Some((
                envelope.sequence,
                serde_json::from_str(&envelope.payload).ok()?,
            ))
        })();
        if let Some(record) = record {
            result.push(record);
        }
    }
    if exists && result.is_empty() {
        Err(())
    } else {
        Ok(result)
    }
}
pub(crate) fn load_delivery(path: &std::path::Path) -> Delivery {
    match slots(path) {
        Ok(records) if !records.is_empty() => {
            Delivery::restore(records.into_iter().max_by_key(|(seq, _)| *seq).unwrap().1)
        }
        Ok(_) => match std::fs::read(path) {
            Ok(bytes) => match serde_json::from_slice::<AlertLedger>(&bytes) {
                Ok(ledger) => Delivery::restore(Record {
                    ledger,
                    pending: false,
                }),
                Err(_) => Delivery::blocked(),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Delivery::default(),
            Err(_) => Delivery::blocked(),
        },
        Err(_) => Delivery::blocked(),
    }
}
pub(crate) fn save_record(path: &std::path::Path, record: &Record) -> Result<(), ()> {
    use sha2::{Digest, Sha256};
    use std::io::Write;
    let sequence = slots(path)?
        .into_iter()
        .map(|(seq, _)| seq)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(())?;
    let payload = serde_json::to_string(record).map_err(|_| ())?;
    let digest = format!("{:x}", Sha256::digest(payload.as_bytes()));
    let bytes = serde_json::to_vec(&Envelope {
        version: 1,
        sequence,
        payload,
        digest,
    })
    .map_err(|_| ())?;
    if bytes.len() > 1_048_576 {
        return Err(());
    }
    let mut file =
        std::fs::File::create(&ledger_paths(path)[(sequence % 2) as usize]).map_err(|_| ())?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codex::QuotaWindow;
    const NOW: i64 = 1792051200000;

    fn snapshot(remaining: f64, time: i64, reset: i64, identity: &str) -> QuotaSnapshot {
        QuotaSnapshot {
            identity_key: identity.into(),
            status: "available".into(),
            collected_at_ms: time,
            windows: vec![QuotaWindow {
                pool_id: "codex".into(),
                window_id: "primary".into(),
                label: "额度窗口".into(),
                used_percent: Some(100.0 - remaining),
                remaining_percent: Some(remaining),
                resets_at_ms: Some(reset),
                duration_minutes: Some(60),
                reached_type: None,
            }],
        }
    }

    #[test]
    fn window_cycles_and_returning_identity_keep_deduplication() {
        let settings = AlertSettings::default();
        let (a, _) = evaluate(
            &AlertLedger::default(),
            &snapshot(10.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
        );
        let (b, _) = evaluate(
            &a,
            &snapshot(10.0, NOW + 1000, NOW + 3_600_000, "b"),
            &settings,
            NOW + 1000,
        );
        let (a, again) = evaluate(
            &b,
            &snapshot(9.0, NOW + 2000, NOW + 3_605_000, "a"),
            &settings,
            NOW + 2000,
        );
        assert!(again.is_none());
        let (renewed, _) = evaluate(
            &a,
            &snapshot(90.0, NOW + 3000, NOW + 7_200_000, "a"),
            &settings,
            NOW + 3000,
        );
        assert!(evaluate(
            &renewed,
            &snapshot(10.0, NOW + 4000, NOW + 7_200_000, "a"),
            &settings,
            NOW + 4000
        )
        .1
        .is_some());
    }
    #[test]
    fn partial_recovery_and_expired_windows_never_claim_everything_recovered() {
        let settings = AlertSettings::default();
        let mut first = snapshot(0.0, NOW, NOW + 3_600_000, "a");
        let mut other = first.windows[0].clone();
        other.window_id = "secondary".into();
        first.windows.push(other);
        let (ledger, _) = evaluate(&AlertLedger::default(), &first, &settings, NOW);
        first.collected_at_ms += 1000;
        first.windows[0].remaining_percent = Some(80.0);
        let (_, event) = evaluate(&ledger, &first, &settings, NOW + 1000);
        assert!(event.unwrap().body.contains("其他窗口"));
        first.windows[0].resets_at_ms = Some(NOW);
        assert!(evaluate(&ledger, &first, &settings, NOW + 1000).1.is_none());
    }
    #[test]
    fn uncertain_submission_keeps_reservation_and_never_retries_after_restart() {
        let settings = AlertSettings::default();
        let mut runtime = Delivery::default();
        let mut records = Vec::new();
        runtime.process(
            &snapshot(10.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
            true,
            |record| {
                records.push(record.clone());
                Ok(())
            },
            |_| Err(SubmissionError::Uncertain),
        );
        assert_eq!(runtime.status, "uncertain");
        assert_eq!(records.len(), 1);
        assert!(records[0].pending);
        let mut restarted = Delivery::restore(records[0].clone());
        for delivery in [&mut runtime, &mut restarted] {
            delivery.process(
                &snapshot(9.0, NOW + 61_000, NOW + 3_600_000, "a"),
                &settings,
                NOW + 61_000,
                true,
                |_| Ok(()),
                |_| panic!("ambiguous submission must not repeat"),
            );
        }
    }

    #[test]
    fn failed_low_submission_has_a_cooldown_and_does_not_retry_stale_data() {
        let settings = AlertSettings::default();
        let mut runtime = Delivery::default();
        let mut sends = 0;
        runtime.process(
            &snapshot(10.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
            true,
            |_| Ok(()),
            |_| {
                sends += 1;
                Err(SubmissionError::Rejected)
            },
        );
        runtime.process(
            &snapshot(9.0, NOW + 1000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 1000,
            true,
            |_| Ok(()),
            |_| {
                sends += 1;
                Ok(())
            },
        );
        assert_eq!(sends, 1);
        runtime.process(
            &snapshot(8.0, NOW + 61_000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 61_000,
            true,
            |_| Ok(()),
            |_| {
                sends += 1;
                Ok(())
            },
        );
        runtime.process(
            &snapshot(7.0, NOW + 62_000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 62_000,
            true,
            |_| Ok(()),
            |_| {
                sends += 1;
                Ok(())
            },
        );
        assert_eq!(sends, 2);
    }
    #[test]
    fn durable_slots_restore_pending_and_fail_closed_when_both_corrupt() {
        use std::{
            fs,
            time::{SystemTime, UNIX_EPOCH},
        };
        let directory = std::env::temp_dir().join(format!(
            "tokendaily-alerts-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        let path = directory.join("alerts.json");
        let settings = AlertSettings::default();
        let mut runtime = Delivery::default();
        runtime.process(
            &snapshot(10.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
            true,
            |record| save_record(&path, record),
            |_| Ok(()),
        );
        let records = slots(&path).unwrap();
        assert_eq!(records.len(), 2);
        let latest = records.iter().map(|(seq, _)| *seq).max().unwrap();
        fs::write(&ledger_paths(&path)[(latest % 2) as usize], b"truncated").unwrap();
        let mut restored = load_delivery(&path);
        assert_eq!(restored.status, "uncertain");
        restored.process(
            &snapshot(9.0, NOW + 1000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 1000,
            true,
            |record| save_record(&path, record),
            |_| panic!("must not repeat pending notification"),
        );
        for slot in ledger_paths(&path) {
            fs::write(slot, b"corrupt").unwrap();
        }
        let mut blocked = load_delivery(&path);
        assert!(blocked.blocked);
        blocked.process(
            &snapshot(8.0, NOW + 2000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 2000,
            true,
            |_| panic!("must preserve corrupt evidence"),
            |_| panic!("must not send"),
        );
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn failed_submission_retries_only_with_new_data_and_preserves_recovery() {
        let settings = AlertSettings::default();
        let mut runtime = Delivery::default();
        let mut records = Vec::new();
        runtime.process(
            &snapshot(0.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
            true,
            |record| {
                records.push(record.clone());
                Ok(())
            },
            |_| Err(SubmissionError::Rejected),
        );
        assert_eq!(runtime.status, "retry");
        let restored = records.last().unwrap().clone();
        let mut runtime = Delivery::restore(restored);
        let mut sends = 0;
        runtime.process(
            &snapshot(0.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW + 60_000,
            true,
            |_| Ok(()),
            |_| {
                sends += 1;
                Ok(())
            },
        );
        assert_eq!(sends, 0);
        runtime.process(
            &snapshot(80.0, NOW + 61_000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 61_000,
            true,
            |_| Ok(()),
            |event| {
                assert!(event.title.contains("恢复"));
                sends += 1;
                Ok(())
            },
        );
        assert_eq!(sends, 1);
        assert_eq!(runtime.status, "submitted");
    }
    #[test]
    fn reservation_failure_never_submits_and_pending_restart_never_replays() {
        let settings = AlertSettings::default();
        let low = snapshot(10.0, NOW, NOW + 3_600_000, "a");
        let mut runtime = Delivery::default();
        let mut sends = 0;
        runtime.process(
            &low,
            &settings,
            NOW,
            true,
            |_| Err(()),
            |_| {
                sends += 1;
                Ok(())
            },
        );
        assert_eq!(sends, 0);
        assert_eq!(runtime.status, "storage-error");
        let mut runtime = Delivery::default();
        let mut records = Vec::new();
        runtime.process(
            &low,
            &settings,
            NOW,
            true,
            |record| {
                if records.is_empty() {
                    records.push(record.clone());
                    Ok(())
                } else {
                    Err(())
                }
            },
            |_| {
                sends += 1;
                Ok(())
            },
        );
        assert_eq!(sends, 1);
        assert_eq!(runtime.status, "uncertain");
        let mut restored = Delivery::restore(records[0].clone());
        restored.process(
            &snapshot(9.0, NOW + 1000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 1000,
            true,
            |_| Ok(()),
            |_| {
                sends += 1;
                Ok(())
            },
        );
        assert_eq!(sends, 1);
    }
    #[test]
    fn failed_recovery_remains_pending_until_new_recovery_evidence() {
        let settings = AlertSettings::default();
        let mut runtime = Delivery::default();
        runtime.process(
            &snapshot(0.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
            true,
            |_| Ok(()),
            |_| Ok(()),
        );
        runtime.process(
            &snapshot(80.0, NOW + 1000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 1000,
            true,
            |_| Ok(()),
            |_| Err(SubmissionError::Rejected),
        );
        let mut sent = 0;
        runtime.process(
            &snapshot(80.0, NOW + 62_000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 62_000,
            true,
            |_| Ok(()),
            |event| {
                assert!(event.title.contains("恢复"));
                sent += 1;
                Ok(())
            },
        );
        assert_eq!(sent, 1);
    }

    #[test]
    fn low_alert_fires_once_and_survives_serialization() {
        let settings = AlertSettings::default();
        let (first, event) = evaluate(
            &AlertLedger::default(),
            &snapshot(18.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
        );
        assert_eq!(event.unwrap().title, "Codex 额度偏低");
        let restored: AlertLedger =
            serde_json::from_slice(&serde_json::to_vec(&first).unwrap()).unwrap();
        let (_, again) = evaluate(
            &restored,
            &snapshot(18.0, NOW + 1000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 1000,
        );
        assert!(again.is_none());
    }

    #[test]
    fn stale_or_elapsed_data_cannot_confirm_recovery() {
        let settings = AlertSettings::default();
        let (old, _) = evaluate(
            &AlertLedger::default(),
            &snapshot(0.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
        );
        let (_, stale) = evaluate(
            &old,
            &snapshot(80.0, NOW + 1000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 900_000,
        );
        let (_, elapsed) = evaluate(
            &old,
            &snapshot(80.0, NOW + 1000, NOW - 1, "a"),
            &settings,
            NOW + 1000,
        );
        assert!(stale.is_none() && elapsed.is_none());
    }

    #[test]
    fn fresh_recovery_is_once_and_identity_change_isolated() {
        let settings = AlertSettings::default();
        let (old, _) = evaluate(
            &AlertLedger::default(),
            &snapshot(0.0, NOW, NOW + 3_600_000, "a"),
            &settings,
            NOW,
        );
        let (new, recovered) = evaluate(
            &old,
            &snapshot(80.0, NOW + 1000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 1000,
        );
        assert!(recovered.unwrap().title.contains("恢复"));
        assert!(evaluate(
            &new,
            &snapshot(80.0, NOW + 2000, NOW + 3_600_000, "a"),
            &settings,
            NOW + 2000
        )
        .1
        .is_none());
        assert!(evaluate(
            &old,
            &snapshot(80.0, NOW + 1000, NOW + 3_600_000, "b"),
            &settings,
            NOW + 1000
        )
        .1
        .is_none());
    }
}

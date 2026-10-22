use std::sync::Arc;

#[cfg(target_os = "macos")]
pub(crate) use super::macos::log_watch::{Wake, Watch};
#[cfg(target_os = "windows")]
pub(crate) use super::windows::log_watch::{Wake, Watch};

#[derive(Debug, PartialEq)]
pub(crate) enum Signal {
    Manual,
    Changed,
    Timeout,
}

pub(crate) const RECONCILE_MS: i64 = 30_000;
pub(crate) const COALESCE_MS: i64 = 500;

pub(crate) fn wait_ms(now: i64, periodic: i64, dirty: Option<i64>) -> u64 {
    let midnight =
        (now + 8 * 3600_000).div_euclid(86_400_000) * 86_400_000 + 86_400_000 - 8 * 3600_000;
    periodic
        .min(midnight)
        .min(dirty.unwrap_or(i64::MAX))
        .saturating_sub(now)
        .max(0) as u64
}

// Keep the platform worker's wake handle type visible through one shared interface.
pub(crate) type SharedWake = Arc<Wake>;

// A native watcher is an optimization, not the authority for metric validity.
// Keep platform-specific failure reporting out of the shared collector.
pub(crate) fn warning(watch: &Watch) -> Option<&'static str> {
    #[cfg(target_os = "macos")]
    {
        watch.warning()
    }
    #[cfg(target_os = "windows")]
    {
        let _ = watch;
        None
    } // Windows watcher health reporting is unchanged.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waits_for_idle_deadline_or_first_change_without_starving_midnight() {
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-15T15:59:59Z")
            .unwrap()
            .timestamp_millis();
        assert_eq!(wait_ms(now, now + 30_000, None), 1000);
        assert_eq!(wait_ms(now, now + 30_000, Some(now + 500)), 500);
        assert_eq!(wait_ms(now, now - 1, None), 0);
        assert_eq!(wait_ms(now - 60_000, now - 30_000, None), 30_000);
    }
}

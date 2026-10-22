//! Read-only, on-demand total Token history from local tool logs.
//!
//! This intentionally never merges the Codex account's daily buckets. Each
//! displayed day is derived from locally readable logs and grouped in Beijing
//! time. Missing source data remains explicit instead of becoming a zero.
use crate::{
    claude_usage, local_usage, platform,
    usage::{UsageDay, UsageSnapshot, UsageState},
    workbuddy_usage,
};
use std::collections::BTreeMap;

const DAY_MS: i64 = 24 * 60 * 60 * 1000;
const HISTORY_DAYS: i64 = 20 * 7;
const SAFE_MAX: u64 = 9_007_199_254_740_991;

fn add_days(
    target: &mut BTreeMap<String, u64>,
    source: BTreeMap<String, u64>,
) -> Result<(), &'static str> {
    for (date, tokens) in source {
        let total = target.entry(date).or_insert(0);
        *total = total
            .checked_add(tokens)
            .filter(|sum| *sum <= SAFE_MAX)
            .ok_or("总 Token 数值超出可表示范围")?;
    }
    Ok(())
}

fn codex_days(events: Vec<local_usage::RankEvent>) -> Result<BTreeMap<String, u64>, &'static str> {
    let mut days = BTreeMap::new();
    for event in events {
        let total = days
            .entry(local_usage::beijing_day(event.at))
            .or_insert(0u64);
        *total = total
            .checked_add(event.tokens)
            .filter(|sum| *sum <= SAFE_MAX)
            .ok_or("Codex Token 总量超出可表示范围")?;
    }
    Ok(days)
}

fn codex_history(
    now: i64,
    settings: &crate::connection::Settings,
) -> Result<BTreeMap<String, u64>, &'static str> {
    local_usage::rank_events(
        settings.home()?,
        now.saturating_sub(HISTORY_DAYS * DAY_MS),
        now,
    )
    .and_then(codex_days)
}

pub(crate) fn collect(
    now: i64,
    settings: crate::connection::Settings,
    trae_enabled: bool,
) -> UsageState {
    let since = now.saturating_sub(HISTORY_DAYS * DAY_MS);
    let mut sources = vec![
        ("Codex", codex_history(now, &settings)),
        (
            "Claude Code",
            claude_usage::history_days(platform::claude_home(), since, now),
        ),
        (
            "WorkBuddy",
            workbuddy_usage::history_days(platform::workbuddy_home(), since, now),
        ),
    ];
    if trae_enabled {
        sources.push((
            "TRAE 本地记录",
            crate::trae_usage::history_days(platform::trae_database(), since, now),
        ));
    }
    let mut days = BTreeMap::new();
    let mut unread = Vec::new();
    let mut read = Vec::new();
    for (name, result) in sources {
        match result.and_then(|source| add_days(&mut days, source)) {
            Ok(()) => read.push(name),
            Err(_) => unread.push(name),
        }
    }
    if read.is_empty() {
        return UsageState {
            source_unavailable: false,
            status: "unknown".into(),
            snapshot: None,
            last_success_at_ms: None,
            message: Some("未能读取本机总 Token 历史；请确认至少一个本机日志目录可访问。".into()),
        };
    }
    // A source scan that completed with no matching event confirms a genuine
    // zero for today. Historical missing dates remain absent and render blank.
    days.entry(local_usage::beijing_day(now)).or_insert(0);
    let message = if unread.is_empty() {
        None
    } else {
        Some(format!(
            "统计不完整：未读取 {}；仅汇总 {} 的本机日志。",
            unread.join("、"),
            read.join("、")
        ))
    };
    UsageState {
        source_unavailable: false,
        status: "available".into(),
        snapshot: Some(UsageSnapshot {
            api_cost: None,
            identity_key: "local-total-history-beijing".into(),
            collected_at_ms: now,
            days: days
                .into_iter()
                .map(|(date, tokens)| UsageDay { date, tokens })
                .collect(),
        }),
        last_success_at_ms: Some(now),
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn history_respects_explicit_missing_home_without_falling_back() {
        let settings = crate::connection::Settings {
            home: Some(std::env::temp_dir().join("tokendaily-missing-history-source")),
            executable: None,
        };
        assert!(codex_history(1_800_000_000_000, &settings).is_err());
    }

    #[test]
    fn history_range_covers_twenty_week_grid_from_monday() {
        let monday = 4 * DAY_MS - 8 * 3_600_000; // 1970-01-05 Beijing midnight.
        let first_visible = monday - 19 * 7 * DAY_MS;
        for weekday in 0..7 {
            let now = monday + weekday * DAY_MS + DAY_MS - 1;
            let since = now - HISTORY_DAYS * DAY_MS;
            assert!(
                since <= first_visible,
                "must include the entire first visible day"
            );
            assert!(since >= first_visible - 7 * DAY_MS, "scan remains bounded");
        }
    }

    #[test]
    fn merges_sources_by_date_without_creating_missing_days() {
        let mut target = BTreeMap::from([("2026-10-16".into(), 7)]);
        add_days(
            &mut target,
            BTreeMap::from([("2026-10-16".into(), 11), ("2026-10-18".into(), 3)]),
        )
        .unwrap();
        assert_eq!(target.get("2026-10-16"), Some(&18));
        assert_eq!(target.get("2026-10-17"), None);
        assert_eq!(target.get("2026-10-18"), Some(&3));
    }

    #[test]
    fn refuses_overflow_instead_of_wrapping_total_usage() {
        let mut target = BTreeMap::from([("2026-10-16".into(), SAFE_MAX)]);
        assert!(add_days(&mut target, BTreeMap::from([("2026-10-16".into(), 1)])).is_err());
        assert!(codex_days(vec![
            local_usage::RankEvent {
                id: "a".into(),
                at: 1,
                tokens: SAFE_MAX,
                tool: "codex".into()
            },
            local_usage::RankEvent {
                id: "b".into(),
                at: 2,
                tokens: 1,
                tool: "codex".into()
            },
        ])
        .is_err());
    }
}

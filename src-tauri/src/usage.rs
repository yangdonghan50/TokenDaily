use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UsageDay {
    pub(crate) date: String,
    pub(crate) tokens: u64,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UsageSnapshot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) api_cost: Option<crate::api_cost::ApiCost>,
    pub(crate) identity_key: String,
    pub(crate) collected_at_ms: i64,
    pub(crate) days: Vec<UsageDay>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UsageState {
    #[serde(default)]
    pub(crate) source_unavailable: bool,
    pub(crate) status: String,
    pub(crate) snapshot: Option<UsageSnapshot>,
    pub(crate) last_success_at_ms: Option<i64>,
    pub(crate) message: Option<String>,
}

impl Default for UsageState {
    fn default() -> Self {
        Self {
            source_unavailable: false,
            status: "unknown".into(),
            snapshot: None,
            last_success_at_ms: None,
            message: None,
        }
    }
}

impl UsageState {
    pub(crate) fn available(snapshot: UsageSnapshot) -> Self {
        Self {
            source_unavailable: false,
            status: if snapshot.days.is_empty() {
                "unknown"
            } else {
                "available"
            }
            .into(),
            last_success_at_ms: Some(snapshot.collected_at_ms),
            snapshot: Some(snapshot),
            message: None,
        }
    }

    pub(crate) fn with_error(previous: &Self, message: &'static str) -> Self {
        let mut next = previous.clone();
        next.source_unavailable = false;
        next.status = if next.snapshot.is_some() {
            "stale"
        } else {
            "error"
        }
        .into();
        next.message = Some(message.into());
        next
    }
}

fn valid_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let (year, month, day) = (&date[..4], &date[5..7], &date[8..10]);
    let (Ok(year), Ok(month), Ok(day)) = (
        year.parse::<u32>(),
        month.parse::<u32>(),
        day.parse::<u32>(),
    ) else {
        return false;
    };
    if year < 1970 || !(1..=12).contains(&month) {
        return false;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let max_day = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    (1..=max_day).contains(&day)
}

pub(crate) fn normalize(
    raw: &Value,
    identity_key: String,
    collected_at_ms: i64,
) -> Result<UsageSnapshot, &'static str> {
    let buckets = raw
        .get("dailyUsageBuckets")
        .and_then(Value::as_array)
        .ok_or("来源未提供日用量记录")?;
    let mut dates = BTreeSet::new();
    let mut days = Vec::with_capacity(buckets.len());
    for bucket in buckets {
        let date = bucket
            .get("startDate")
            .and_then(Value::as_str)
            .ok_or("日用量日期缺失")?;
        let tokens = bucket
            .get("tokens")
            .and_then(Value::as_u64)
            .filter(|count| *count <= 9_007_199_254_740_991)
            .ok_or("日用量数值无效")?;
        if !valid_date(date) || !dates.insert(date.to_owned()) {
            return Err("日用量日期无效或重复");
        }
        days.push(UsageDay {
            date: date.to_owned(),
            tokens,
        });
    }
    days.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(UsageSnapshot {
        api_cost: None,
        identity_key,
        collected_at_ms,
        days,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_source_dates_without_converting_them_to_local_today() {
        let usage = normalize(
            &json!({"dailyUsageBuckets":[
                {"startDate":"2026-10-15","tokens":26847},
                {"startDate":"2026-10-14","tokens":0}
            ]}),
            "codex:account-a:".into(),
            12,
        )
        .unwrap();
        assert_eq!(usage.days[0].date, "2026-10-14");
        assert_eq!(usage.days[0].tokens, 0);
        assert_eq!(usage.days[1].tokens, 26847);
        assert_eq!(usage.identity_key, "codex:account-a:");
    }

    #[test]
    fn rejects_invalid_or_duplicate_buckets_instead_of_showing_partial_history() {
        for raw in [
            json!({"dailyUsageBuckets":null}),
            json!({"dailyUsageBuckets":[{"startDate":"2026-02-30","tokens":1}]}),
            json!({"dailyUsageBuckets":[{"startDate":"2026-10-15","tokens":-1}]}),
            json!({"dailyUsageBuckets":[{"startDate":"2026-10-15","tokens":1},{"startDate":"2026-10-15","tokens":2}]}),
        ] {
            assert!(normalize(&raw, "identity".into(), 1).is_err());
        }
        assert!(valid_date("2024-02-29"));
        assert!(!valid_date("2025-02-29"));
    }

    #[test]
    fn failed_refresh_marks_previous_usage_stale_without_changing_count() {
        let prior = UsageState::available(
            normalize(
                &json!({"dailyUsageBuckets":[{"startDate":"2026-10-15","tokens":7}]}),
                "a".into(),
                123,
            )
            .unwrap(),
        );
        let failed = UsageState::with_error(&prior, "失败");
        assert_eq!(failed.status, "stale");
        assert_eq!(failed.last_success_at_ms, Some(123));
        assert_eq!(failed.snapshot.unwrap().days[0].tokens, 7);
    }
}

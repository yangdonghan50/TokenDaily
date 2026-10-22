//! Local equivalent Standard/global API token cost, NOT a subscription bill.
//! Verified 2026-10-18. Sources and exclusions: docs/API_COST.md.
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiCost {
    pub(crate) usd: f64,
    pub(crate) priced_tokens: u64,
    pub(crate) unpriced_tokens: u64,
}
impl ApiCost {
    pub(crate) fn add(&mut self, other: Self) {
        self.usd += other.usd;
        self.priced_tokens += other.priced_tokens;
        self.unpriced_tokens += other.unpriced_tokens;
    }
    fn unknown(tokens: u64) -> Self {
        Self {
            unpriced_tokens: tokens,
            ..Self::default()
        }
    }
    fn charge(&mut self, tokens: u64, rate: f64) {
        self.priced_tokens += tokens;
        self.usd += tokens as f64 * rate / 1_000_000.;
    }
}
pub(crate) fn codex(
    model: Option<&str>,
    input: u64,
    output: u64,
    cached: Option<u64>,
    write: Option<u64>,
) -> ApiCost {
    let (base, read, out) = match model {
        Some("gpt-6.1-sol") => (2., 0.1, 10.),
        Some("gpt-6-sol") => (2., 0.2, 10.),
        Some("gpt-5.6-sol" | "gpt-5.6") => (4., 0.4, 20.),
        Some("gpt-6-astra") => (10., 1., 50.),
        Some("gpt-6-luna") => (0.1, 0.01, 0.5),
        _ => return ApiCost::unknown(input + output),
    };
    if cached.is_some_and(|c| c > input) || write.is_some_and(|w| w > input - cached.unwrap_or(0)) {
        return ApiCost::unknown(input + output);
    }
    let (in_mult, out_mult) = if input > 272_000 { (2., 1.5) } else { (1., 1.) };
    let mut cost = ApiCost::default();
    cost.charge(output, out * out_mult);
    if let Some(cached) = cached {
        cost.charge(cached, read * in_mult);
        if let Some(write) = write {
            cost.charge(write, base * 1.25 * in_mult);
            cost.charge(input - cached - write, base * in_mult);
        } else {
            cost.unpriced_tokens += input - cached;
        }
    } else {
        cost.unpriced_tokens += input;
    }
    cost
}
pub(crate) fn claude(model: Option<&str>, counts: [u64; 4], writes: Option<[u64; 2]>) -> ApiCost {
    let (base, out, read) = match model {
        Some("claude-opus-5-5") => (4., 20., 0.2),
        Some("claude-sonnet-5-5") => (2., 10., 0.1),
        Some("claude-opus-5" | "claude-opus-4-8" | "claude-opus-4-7" | "claude-opus-4-6") => {
            (5., 25., 0.5)
        }
        _ => return ApiCost::unknown(counts.iter().sum()),
    };
    let mut cost = ApiCost::default();
    cost.charge(counts[0], base);
    cost.charge(counts[1], out);
    cost.charge(counts[2], read);
    if let Some([short, long]) = writes.filter(|w| w[0].checked_add(w[1]) == Some(counts[3])) {
        cost.charge(short, base * 1.25);
        cost.charge(long, base * 2.);
    } else {
        cost.unpriced_tokens += counts[3];
    }
    cost
}
#[cfg(test)]
#[path = "api_cost_tests.rs"]
mod tests;

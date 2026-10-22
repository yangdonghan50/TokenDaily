use super::*;
#[test]
fn codex_cache_is_subset_and_long_context_is_per_request() {
    let cost = codex(Some("gpt-6.1-sol"), 1000, 100, Some(500), Some(100));
    assert!((cost.usd - 0.0021).abs() < 1e-12);
    assert_eq!(cost.priced_tokens, 1100);
    assert_eq!(cost.unpriced_tokens, 0);
    let cost = codex(Some("gpt-6.1-sol"), 300000, 100, Some(200000), Some(0));
    assert!((cost.usd - 0.4415).abs() < 1e-12);
}
#[test]
fn unknown_model_and_inconsistent_breakdown_never_become_free() {
    assert_eq!(
        codex(Some("gpt-unknown"), 100, 10, Some(0), Some(0)).unpriced_tokens,
        110
    );
    assert_eq!(codex(None, 100, 10, Some(0), Some(0)).priced_tokens, 0);
    assert_eq!(
        codex(Some("gpt-6.1-sol"), 100, 10, Some(101), Some(0)).unpriced_tokens,
        110
    );
    let missing = codex(Some("gpt-6.1-sol"), 100, 10, Some(80), None);
    assert_eq!((missing.priced_tokens, missing.unpriced_tokens), (90, 20));
    assert!((missing.usd - 0.000108).abs() < 1e-12);
}
#[test]
fn claude_cache_write_duration_is_not_guessed() {
    let a = claude(Some("claude-opus-5-5"), [100, 10, 500, 200], None);
    assert_eq!((a.priced_tokens, a.unpriced_tokens), (610, 200));
    let b = claude(
        Some("claude-opus-5-5"),
        [100, 10, 500, 200],
        Some([150, 50]),
    );
    assert_eq!(b.unpriced_tokens, 0);
    assert!((b.usd - 0.00185).abs() < 1e-12);
    assert_eq!(
        claude(Some("claude-new"), [100, 10, 500, 200], None).unpriced_tokens,
        810
    );
}

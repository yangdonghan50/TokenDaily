import test from 'node:test';
import assert from 'node:assert/strict';
import { formatRemaining, normalizeQuotaResponse, quotaFreshness, resetPresentation } from '../src/domain/quota.ts';

const context = { accountRef: 'account-a', workspaceRef: null, collectedAtMs: Date.UTC(2026, 9, 15, 4) };

test('73% used becomes 27% remaining, preserving the source window', () => {
  const snapshot = normalizeQuotaResponse({ rateLimits: {
    limitId: 'codex', primary: { usedPercent: 73, windowDurationMins: 60, resetsAt: 1792051200 },
  } }, context);
  assert.equal(snapshot.status, 'available');
  assert.equal(snapshot.windows.length, 1);
  assert.equal(snapshot.windows[0].remainingPercent, 27);
  assert.equal(snapshot.windows[0].resetsAtMs, 1792051200000);
  assert.equal(snapshot.windows[0].durationMinutes, 60);
});

test('missing, null, and invalid percentages stay unknown', () => {
  for (const value of [undefined, null, -1, 101, '73', Number.NaN]) {
    const snapshot = normalizeQuotaResponse({ rateLimits: {
      limitId: 'codex', primary: { usedPercent: value, resetsAt: 1792051200 },
    } }, context);
    assert.equal(snapshot.windows[0].remainingPercent, null);
    assert.equal(snapshot.status, 'unknown');
  }
});

test('single-bucket and multi-bucket views of the same pool are de-duplicated', () => {
  const bucket = { limitId: 'codex', primary: { usedPercent: 25, resetsAt: 1792051200 } };
  const snapshot = normalizeQuotaResponse({ rateLimits: bucket, rateLimitsByLimitId: { codex: bucket } }, context);
  assert.equal(snapshot.windows.length, 1);
});

test('a weekly-only primary window does not invent another window or a fixed duration', () => {
  const snapshot = normalizeQuotaResponse({ rateLimits: {
    limitId: 'codex', primary: { usedPercent: 20, windowDurationMins: 10080, resetsAt: 1792051200 },
  } }, context);
  assert.equal(snapshot.windows.length, 1);
  assert.equal(snapshot.windows[0].durationMinutes, 10080);
  assert.match(snapshot.windows[0].label, /7 天/);
});

test('seconds normalize to milliseconds and elapsed reset is awaiting confirmation', () => {
  const snapshot = normalizeQuotaResponse({ rateLimits: {
    limitId: 'codex', primary: { usedPercent: 100, resetsAt: 1792051200 },
  } }, context);
  assert.equal(snapshot.windows[0].resetsAtMs, 1792051200000);
  assert.equal(quotaFreshness(snapshot, 1792051200001).resetElapsed, true);
  assert.match(resetPresentation(1792051200000, 1792051200001, 'Asia/Shanghai').relative, /等待新数据确认/);
  assert.equal(formatRemaining(0.4), '<1%');
});

test('account and workspace identity remain distinct', () => {
  const raw = { rateLimits: { limitId: 'codex', primary: { usedPercent: 50 } } };
  const first = normalizeQuotaResponse(raw, context);
  const second = normalizeQuotaResponse(raw, { ...context, accountRef: 'account-b', workspaceRef: 'work-b' });
  assert.notEqual(first.identityKey, second.identityKey);
});

test('a snapshot a few milliseconds newer than UI clock is still fresh', () => {
  const snapshot = normalizeQuotaResponse({ rateLimits: { limitId: 'codex',
    primary: { usedPercent: 30, resetsAt: 1792051200 } } }, context);
  assert.equal(quotaFreshness(snapshot, context.collectedAtMs - 2).fresh, true);
  assert.equal(quotaFreshness(snapshot, context.collectedAtMs - 6000).fresh, false);
});

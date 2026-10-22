import test from 'node:test';
import assert from 'node:assert/strict';
import { floatingUsage, compactTokens } from '../src/domain/floatingUsage.ts';
import type { UsageState } from '../src/domain/usage.ts';

const now = 1_792_051_200_000;
const usage: UsageState = { status: 'available', message: null, lastSuccessAtMs: now,
  snapshot: { identityKey: 'example', collectedAtMs: now, days: [{ date: '2026-10-14', tokens: 0 }] } };

test('floating usage preserves source date and explicit zero; missing records stay unknown', () => {
  assert.deepEqual(floatingUsage(usage, now), { day: { date: '2026-10-14', tokens: 0 }, stale: false });
  assert.equal(floatingUsage({ ...usage, snapshot: null }, now).day, null);
  assert.equal(floatingUsage({ ...usage, snapshot: { ...usage.snapshot!, days: [] } }, now).day, null);
});

test('usage freshness is independent, rejects old/future/missing timestamps and failed states', () => {
  assert.equal(floatingUsage(usage, now + 600_001).stale, true);
  assert.equal(floatingUsage(usage, now - 5_001).stale, true);
  assert.equal(floatingUsage({ ...usage, lastSuccessAtMs: null }, now).stale, true);
  for (const status of ['stale', 'error', 'unknown'] as const) {
    assert.equal(floatingUsage({ ...usage, status }, now).stale, true);
  }
});

test('compact tokens keep units explicit and avoid rounding across a unit boundary', () => {
  assert.deepEqual(compactTokens(0), { value: '0', unit: '' });
  assert.deepEqual(compactTokens(9999), { value: '9,999', unit: '' });
  assert.deepEqual(compactTokens(10000), { value: '1.00', unit: '万' });
  assert.deepEqual(compactTokens(99999999), { value: '9,999.99', unit: '万' });
  assert.deepEqual(compactTokens(166302134), { value: '1.66', unit: '亿' });
});

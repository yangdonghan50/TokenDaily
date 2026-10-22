import test from 'node:test';
import assert from 'node:assert/strict';
import { formatTokenCount, formatExactTokenCount } from '../src/domain/usage.ts';
import { compactTokens } from '../src/domain/floatingUsage.ts';

test('all token displays follow floating units without rounding up', () => {
  for (const [input, expected] of [
    [0, '0'], [9999, '9,999'], [10000, '1.00万'], [10099, '1.00万'],
    [10100, '1.01万'], [627306, '62.73万'], [99999999, '9,999.99万'],
    [100000000, '1.00亿'], [741969661, '7.41亿'], [1000000000000, '10,000.00亿'],
  ] as const) {
    assert.equal(formatTokenCount(input), expected);
    const parts = compactTokens(input);
    assert.equal(parts.value + parts.unit, expected);
  }
  assert.equal(formatExactTokenCount(741969661), '741,969,661');
});

test('invalid and missing counts stay unknown, not zero', () => {
  for (const input of [NaN, Infinity, -1, 0.5, Number.MAX_SAFE_INTEGER + 1, null, undefined]) {
    assert.equal(formatTokenCount(input as number), '—');
    assert.equal(formatExactTokenCount(input as number), '—');
    assert.deepEqual(compactTokens(input as number), { value: '—', unit: '' });
  }
});

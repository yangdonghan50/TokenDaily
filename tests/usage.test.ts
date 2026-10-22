import test from 'node:test';
import assert from 'node:assert/strict';
import { usageCalendar, usageDaysEndingOn, usageLevel } from '../src/domain/usage.ts';

test('missing source date stays unknown while an explicit zero stays zero', () => {
  assert.deepEqual(usageDaysEndingOn([
    { date: '2026-10-13', tokens: 0 }, { date: '2026-10-15', tokens: 26847 },
  ], '2026-10-15', 3), [
    { date: '2026-10-13', tokens: 0 },
    { date: '2026-10-14', tokens: null },
    { date: '2026-10-15', tokens: 26847 },
  ]);
  assert.equal(usageLevel(null, 26847), null);
  assert.equal(usageLevel(0, 26847), 0);
});

test('calendar aligns Monday first and marks dates after latest source day as future', () => {
  const cells = usageCalendar([{ date: '2026-10-15', tokens: 42 }], '2026-10-15', 1);
  assert.equal(cells.length, 7);
  assert.equal(cells[0].date, '2026-10-12');
  assert.equal(cells[3].tokens, 42);
  assert.equal(cells[4].future, true);
  assert.equal(cells[0].tokens, null);
});

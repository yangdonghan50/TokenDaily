import test from 'node:test';
import assert from 'node:assert/strict';
import { usageDatePresentation } from '../src/domain/usage.ts';

test('a successful read of yesterday does not turn it into today or fabricate a zero', () => {
  const days = [{ date: '2026-10-15', tokens: 444124329 }];
  const view = usageDatePresentation(days, Date.parse('2026-10-16T09:30:00+08:00'), 'Asia/Shanghai');
  assert.equal(view.label, '10/15 Token 记录');
  assert.equal(view.notice, '尚未取得 10/16 日记录');
  assert.equal(days[0].tokens, 444124329);
});

test('local midnight advances the requested date without changing the source date', () => {
  const days = [{ date: '2026-10-15', tokens: 42 }];
  assert.equal(usageDatePresentation(days, Date.parse('2026-10-15T23:59:59+08:00'), 'Asia/Shanghai').notice, null);
  assert.equal(usageDatePresentation(days, Date.parse('2026-10-16T00:00:00+08:00'), 'Asia/Shanghai').notice, '尚未取得 10/16 日记录');
  assert.equal(usageDatePresentation(days, Date.parse('2026-10-16T00:00:00+08:00'), 'America/Los_Angeles').notice, null);
});

test('new source date including explicit zero clears missing-date notice; empty stays unknown', () => {
  const now = Date.parse('2026-10-16T09:00:00+08:00');
  const view = usageDatePresentation([{ date: '2026-10-16', tokens: 0 }], now, 'Asia/Shanghai');
  assert.equal(view.label, '10/16 Token 记录');
  assert.equal(view.notice, null);
  assert.equal(usageDatePresentation([], now, 'Asia/Shanghai').label, 'Token 记录未知');
});

test('year boundary retains source year and future source dates are not presented as today', () => {
  const now = Date.parse('2027-01-01T00:01:00+08:00');
  const prior = usageDatePresentation([{ date: '2026-12-31', tokens: 1 }], now, 'Asia/Shanghai');
  assert.equal(prior.label, '2026/12/31 Token 记录');
  assert.equal(prior.notice, '尚未取得 01/01 日记录');
  assert.equal(usageDatePresentation([{ date: '2027-01-02', tokens: 1 }], now, 'Asia/Shanghai').notice, '来源日期晚于本地日期，口径待确认');
});

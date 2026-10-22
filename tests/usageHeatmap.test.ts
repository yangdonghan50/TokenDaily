import test from 'node:test';
import assert from 'node:assert/strict';
import { heatmapLabel, heatmapMonths } from '../src/domain/usageHeatmap.ts';
import { usageCalendar } from '../src/domain/usage.ts';

test('hover copy distinguishes missing, confirmed zero, compact counts and future',()=>{
  assert.equal(heatmapLabel({date:'2026-10-18',tokens:null}),'2026-10-18 · 本机日志未记录');
  assert.equal(heatmapLabel({date:'2026-10-18',tokens:0}),'2026-10-18 · 0 Tokens');
  assert.equal(heatmapLabel({date:'2026-10-18',tokens:98552800}),'2026-10-18 · 9,855.28万 Tokens');
  assert.equal(heatmapLabel({date:'2026-10-19',tokens:null,future:true}),'2026-10-19 · 未来日期');
});
test('month labels align with week columns and remain sparse across year boundary',()=>{
  const cells=usageCalendar([], '2027-01-05');
  const months=heatmapMonths(cells);
  assert.equal(months.length,13);
  assert.deepEqual(months.filter(Boolean),['10月','11月','12月','1月']);
});

test('extended floating calendar spans twenty columns across year boundary without filling missing days',()=>{
  const cells=usageCalendar([{date:'2026-09-01',tokens:42}],'2027-01-05',20);
  assert.equal(cells.length,140);
  assert.equal(heatmapMonths(cells).length,20);
  assert.equal(cells.find(c=>c.date==='2026-09-01')?.tokens,42);
  assert.equal(cells.find(c=>c.date==='2026-09-02')?.tokens,null);
  assert.equal(cells.at(-1)?.future,true);
});

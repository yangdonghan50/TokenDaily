import { test } from 'node:test';
import assert from 'node:assert/strict';
import { combinedUsage } from '../src/domain/combinedUsage.ts';
import type { UsageState } from '../src/domain/usage.ts';
const now = Date.parse('2026-10-17T00:00:00Z');
const usage = (tokens: number, date = '2026-10-17', status: UsageState['status'] = 'available'): UsageState => ({ status, snapshot: { identityKey: 'local-test', collectedAtMs: now, days: [{ date, tokens }] }, lastSuccessAtMs: now, message: null });
test('unconnected sources are excluded without concealing connected failures or fabricating zero',()=>{
  const absent:UsageState={status:'unknown',snapshot:null,lastSuccessAtMs:null,message:'No logs',sourceUnavailable:true};
  const known=usage(120);
  const total=combinedUsage(known,absent,now,usage(50));
  assert.equal(total.complete,true);assert.equal(total.usage.snapshot?.days[0].tokens,170);assert.deepEqual(total.missing,[]);
  assert.deepEqual(total.entries.map(e=>e.label),['Codex','WorkBuddy']);
  const disabled=combinedUsage(known,usage(90),now,usage(50),[true,false,true]);
  assert.equal(disabled.usage.snapshot?.days[0].tokens,170);
  assert.equal(combinedUsage(absent,absent,now).usage.snapshot,null);
  assert.equal(combinedUsage(absent,absent,now).complete,false);
  assert.equal(combinedUsage(known,{...absent,sourceUnavailable:false,status:'error'},now).complete,false);
  assert.equal(combinedUsage(known,{...usage(3),sourceUnavailable:true},now).usage.snapshot?.days[0].tokens,123,'inconsistent missing flag cannot discard known data');
});
test('local usage combines same Beijing day and includes a healthy zero', () => {
  const total = combinedUsage(usage(120), usage(80), now);
  assert.equal(total.usage.snapshot?.days[0].tokens, 200); assert.equal(total.complete, true);
  assert.equal(combinedUsage(usage(0), usage(0), now).usage.snapshot?.days[0].tokens, 0);
});

test('WorkBuddy participates in totals without masking unknown or old-day data', () => {
  const total = combinedUsage(usage(120), usage(80), now, usage(50));
  assert.equal(total.usage.snapshot?.days[0].tokens, 250);
  assert.equal(total.entries[2].label, 'WorkBuddy');
  assert.equal(total.complete, true);
  const partial = combinedUsage(usage(120), usage(80), now, usage(50, '2026-10-16', 'stale'));
  assert.equal(partial.usage.snapshot?.days[0].tokens, 200);
  assert.equal(partial.complete, false);
  assert.deepEqual(partial.missing, ['WorkBuddy']);
});
test('missing or failed sources stay incomplete, with no fabricated full total', () => {
  const missing: UsageState = { status: 'unknown', snapshot: null, lastSuccessAtMs: null, message: null };
  const partial = combinedUsage(usage(120), missing, now);
  assert.equal(partial.complete, false); assert.deepEqual(partial.missing, ['Claude Code']);
  assert.equal(partial.usage.snapshot?.days[0].tokens, 120);
  assert.equal(combinedUsage(missing, missing, now).usage.snapshot, null);
  assert.equal(combinedUsage(usage(100), usage(80, '2026-10-17', 'stale'), now).complete, false);
});
test('previous-day failed counts cannot enter today, and overflow stays unknown', () => {
  assert.equal(combinedUsage(usage(100), usage(80, '2026-10-16', 'stale'), now).usage.snapshot?.days[0].tokens, 100);
  assert.equal(combinedUsage(usage(Number.MAX_SAFE_INTEGER), usage(1), now).usage.snapshot, null);
});

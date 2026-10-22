import test from 'node:test';
import assert from 'node:assert/strict';
import { beijingToday, localTodayUsage } from '../src/domain/localUsage.ts';
import type { UsageState } from '../src/domain/usage.ts';

const prior: UsageState = {status:'available',lastSuccessAtMs:Date.parse('2026-10-15T15:59:59Z'),message:null,
  snapshot:{identityKey:'local-codex-beijing',collectedAtMs:Date.parse('2026-10-15T15:59:59Z'),days:[{date:'2026-10-15',tokens:500}]}};
test('today is always Beijing time and rolls over to zero without using yesterday',()=>{
  const now=Date.parse('2026-10-15T16:00:00Z');
  assert.equal(beijingToday(now),'2026-10-16');
  assert.equal(localTodayUsage(prior,now).snapshot?.days[0].tokens,0);
  const priced={...prior,snapshot:{...prior.snapshot!,apiCost:{usd:2,pricedTokens:500,unpricedTokens:0}}};
  assert.equal(localTodayUsage(priced,now).snapshot?.apiCost,null,'yesterday cost cannot follow the synthetic new-day zero');
  assert.equal(beijingToday(Date.parse('2026-12-31T16:00:00Z')),'2027-01-01');
});
test('unreadable or old source never invents a new zero',()=>{
  const now=Date.parse('2026-10-15T16:00:00Z');
  assert.equal(localTodayUsage({...prior,status:'stale'},now).snapshot,null);
  assert.equal(localTodayUsage(prior,now+120_000).snapshot,null);
  assert.equal(localTodayUsage({...prior,status:'error',snapshot:null},now).snapshot,null);
});
test('a fresh real zero is retained, failed same-day counts remain explicitly stale',()=>{
  const now=Date.parse('2026-10-15T15:59:59Z');
  assert.equal(localTodayUsage({...prior,status:'stale'},now).snapshot?.days[0].tokens,500);
  assert.equal(localTodayUsage({...prior,status:'stale'},now).status,'stale');
  const zero={...prior,snapshot:{...prior.snapshot!,days:[{date:'2026-10-15',tokens:0}]}};
  assert.equal(localTodayUsage(zero,now).snapshot?.days[0].tokens,0);
});
test('idle collector stays healthy between 30 second reconciliations',()=>{
  const now=Date.parse('2026-10-15T15:59:59Z');
  assert.equal(localTodayUsage(prior,now+31_000).status,'available');
  assert.notEqual(localTodayUsage(prior,now+91_000).status,'available');
});

import test from 'node:test';
import assert from 'node:assert/strict';
import { createHistoryCache, calendarPresentation } from '../src/domain/floatingCalendar.ts';
import { orderedSources } from '../src/domain/floatingBalances.ts';
import { normalizeHidden } from '../src/domain/floatingComponents.ts';
import { floatingWindowSize } from '../src/domain/floating.ts';
import type { UsageState } from '../src/domain/usage.ts';

const now = Date.parse('2026-10-18T16:00:00Z');
const state: UsageState = {status:'available',lastSuccessAtMs:now,message:null,snapshot:{identityKey:'local',collectedAtMs:now,days:[{date:'2026-10-18',tokens:100},{date:'2026-10-19',tokens:0}]}};
test('calendar migration appends below old quota order and supports hiding',()=>{
  assert.deepEqual(orderedSources(['codex','claude','workbuddy','calendar'].map(id=>({id})),['workbuddy','codex','claude']).map(s=>s.id),['workbuddy','codex','claude','calendar']);
  assert.deepEqual(normalizeHidden(['calendar','calendar']),['calendar']);
  assert.equal(floatingWindowSize(true,false,true,4,false,true).height,478);
  assert.equal(floatingWindowSize(true,false,true,1,false,true).height,292);
  assert.equal(floatingWindowSize(false,false,true,4,true,true).height,56);
});
test('calendar uses Beijing today, distinguishes missing, zero, future and stale',()=>{
  const p=calendarPresentation(state,now);
  assert.equal(p.today,'2026-10-19');assert.equal(p.cells.length,140);
  assert.equal(p.cells[0].date,'2026-06-08');
  assert.equal(p.cells.find(c=>c.date==='2026-10-19')?.tokens,0);
  assert.equal(p.cells.find(c=>c.date==='2026-10-17')?.tokens,null);
  assert.equal(p.cells.find(c=>c.date==='2026-10-20')?.future,true);
  assert.equal(p.stale,false);assert.equal(calendarPresentation(state,now+300001).stale,true);
});
test('history cache coalesces scans, expires on time/day, and forced refresh bypasses cache',async()=>{
  let time=now,calls=0;
  const cache=createHistoryCache(async()=>{calls++;return state;},()=>time);
  await Promise.all([cache.read(),cache.read()]);assert.equal(calls,1);
  await cache.read();assert.equal(calls,1);
  time+=300001;await cache.read();assert.equal(calls,2);
  await cache.read(true);assert.equal(calls,3);
  time=Date.parse('2026-10-19T15:59:59Z');await cache.read();
  time+=2000;await cache.read();assert.equal(calls,5);
});
test('failed history scans retain explicit stale data, throttle retries, recover on refresh',async()=>{
  let fail=false,time=now,calls=0;
  const cache=createHistoryCache(async()=>{calls++;if(fail)throw Error('private path');return state;},()=>time);
  await cache.read();fail=true;
  const result=await cache.read(true);assert.equal(result.status,'stale');assert.equal(result.snapshot,state.snapshot);
  assert.ok(!result.message?.includes('private path'));await cache.read();assert.equal(calls,2);
  time+=30001;await cache.read();assert.equal(calls,3);
  fail=false;assert.equal((await cache.read(true)).status,'available');
});

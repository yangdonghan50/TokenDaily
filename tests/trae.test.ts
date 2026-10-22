import test from 'node:test';
import assert from 'node:assert/strict';
import { combinedUsage } from '../src/domain/combinedUsage.ts';
import { providerConfig } from '../src/domain/provider.ts';
import { orderedSources, normalizeOrder } from '../src/domain/floatingBalances.ts';
import { floatingWindowSize } from '../src/domain/floating.ts';
import { normalizeHidden } from '../src/domain/floatingComponents.ts';
import type { UsageState } from '../src/domain/usage.ts';
const now=Date.parse('2026-10-19T03:00:00Z');
const usage=(tokens:number):UsageState=>({status:'available',lastSuccessAtMs:now,message:null,snapshot:{identityKey:'local',collectedAtMs:now,days:[{date:'2026-10-19',tokens}]}});
test('TRAE is a separate local source included only when enabled, never an account bucket',()=>{
  assert.equal(providerConfig('trae').usageCommand,'get_trae_local_usage');
  assert.equal(combinedUsage(usage(10),usage(20),now,usage(30),[true,true,true,true],usage(26900)).usage.snapshot!.days[0].tokens,26960);
  assert.equal(combinedUsage(usage(10),usage(20),now,usage(30),[true,true,true,false],usage(26900)).usage.snapshot!.days[0].tokens,60);
  assert.deepEqual(normalizeHidden(['trae']),['trae']);
});
test('four AI sources retain independent calendar, saved position and native geometry',()=>{
  const ids=['calendar','trae','codex','workbuddy','claude'];
  assert.deepEqual(normalizeOrder(ids),ids);
  assert.deepEqual(orderedSources(ids.map(id=>({id})),ids).map(s=>s.id),ids);
  assert.equal(orderedSources([...ids,'extra'].map(id=>({id})),[]).length,5);
  assert.equal(floatingWindowSize(true,false,true,5,false,true).height,540);
});

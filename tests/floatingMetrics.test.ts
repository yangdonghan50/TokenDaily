import test from 'node:test';
import assert from 'node:assert/strict';
import { normalizeMetricOrder, METRIC_IDS } from '../src/domain/floatingMetrics.ts';
import { normalizeHidden, setComponentVisible, mergeVisibleOrder, visibleComponents } from '../src/domain/floatingComponents.ts';
import { moveSource } from '../src/domain/floatingBalances.ts';
import { floatingWindowSize } from '../src/domain/floating.ts';

test('metric preferences migrate safely and stay separate from balance order',()=>{
  assert.deepEqual(normalizeMetricOrder(null),METRIC_IDS);
  assert.deepEqual(normalizeMetricOrder(['rank','rank','codex',null,'usage']),['rank','usage','cost']);
  const hidden=normalizeHidden(['claude','calendar','rank','rank','unknown']);
  assert.deepEqual(hidden,['claude','calendar','rank']);
  assert.deepEqual(setComponentVisible(hidden,'rank',true),['claude','calendar']);
  const all=METRIC_IDS.reduce((state,id)=>setComponentVisible(state,id,false),hidden);
  assert.equal(visibleComponents(METRIC_IDS.map(id=>({id})),all).length,0);
  assert.deepEqual(visibleComponents(METRIC_IDS.map(id=>({id})),setComponentVisible(all,'cost',true)),[{id:'cost'}]);
  assert.deepEqual(mergeVisibleOrder(['usage','cost','rank'],moveSource(['usage','rank'],'rank','usage')),['rank','cost','usage']);
});

test('compact top strip shrinks when hidden, while editing reserves recovery controls',()=>{
  assert.equal(floatingWindowSize(true,false,true,4,false,true,3).height,478);
  assert.equal(floatingWindowSize(true,false,true,4,false,true,0).height,414);
  assert.equal(floatingWindowSize(true,false,true,4,true,true,0).height,542);
  assert.deepEqual(floatingWindowSize(false,true,true,4,true,true,0),{width:28,height:56});
});

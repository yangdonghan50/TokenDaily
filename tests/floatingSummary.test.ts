import test from 'node:test';
import assert from 'node:assert/strict';
import { rankPresentation, costPresentation } from '../src/domain/floatingSummary.ts';
import type { UsageState } from '../src/domain/usage.ts';
const now = Date.parse('2026-10-18T10:00:00Z');
const board = {period:'day',tool:'all',start:'2026-10-18',end:'2026-10-18'};
const status = { supported:true, loggedIn:true, enabled:true, thisDevice:'a', remote:{enrollment:{device:'a',active:1,held:0},board:{...board,me:{rank:12,tokens:100}}}};
const present = (value:Parameters<typeof rankPresentation>[0]) => rankPresentation(value,now);
test('zero or unknown usage has no public rank',()=>{
 const zero={...status,remote:{...status.remote,board:{...board,me:{rank:1,tokens:0}}}};
 assert.equal(present(zero)?.value,'—');
 assert.equal(present(zero)?.label,'日榜 暂无名次');
 assert.match(present(zero)!.title,/没有有效用量则不排名/);
 assert.equal(present({...zero,remote:{...zero.remote,enrollment:{...zero.remote.enrollment,held:1}}})?.value,'—');
 assert.equal(present({...zero,remote:{...zero.remote,board:{...board,me:{rank:null,tokens:0}}}})?.value,'—');
});
test('rank requires enrollment, not merely an identity; paused enrollment keeps rank',()=>{
  assert.equal(present(null),null);
  assert.equal(present({...status,loggedIn:false}),null);
  assert.equal(present({...status,remote:{enrollment:null,board:{...board,me:{rank:1,tokens:1}}}}),null);
  assert.equal(present({...status,remote:{enrollment:null,account:{publicId:'account-public',boardHeld:0},board:{...board,me:{rank:1,tokens:100}}}})?.value,'#1');
  assert.equal(present({...status,enabled:false})?.label,'日榜 #12');
  assert.equal(present({...status,remote:{...status.remote,enrollment:{...status.remote.enrollment,held:1}}})?.label,'日榜 待核查');
  assert.equal(present({...status,remote:{...status.remote,board:{...board,me:{rank:0}}}})?.label,'日榜 暂无名次');
  assert.equal(present({...status,remote:null}),null);
});

test('daily rank rejects weekly, previous-day and missing board metadata at Beijing midnight',()=>{
  for(const change of [{period:'week'},{start:'2026-10-17'},{end:'2026-10-17'},{tool:'codex'},{period:undefined}]) {
    assert.equal(present({...status,remote:{...status.remote,board:{...status.remote.board,...change}}}),null);
  }
  assert.equal(rankPresentation(status,Date.parse('2026-10-18T16:00:00Z')),null);
  assert.match(present(status)!.title,/当日排名/);
  assert.match(present(status)!.note,/今日/);
});
function usage(tokens:number,cost?:{usd:number;pricedTokens:number;unpricedTokens:number}):UsageState {
  return {status:'available',snapshot:{identityKey:'local',collectedAtMs:now,days:[{date:'2026-10-18',tokens}],apiCost:cost},lastSuccessAtMs:now,message:null};
}
test('cost distinguishes unknown, partial, zero, stale and previous-day data',()=>{
  const known=usage(100,{usd:1.23,pricedTokens:100,unpricedTokens:0});
  assert.equal(costPresentation([known],now).value,'≈$1.23');
  assert.equal(costPresentation([usage(100)],now).value,'待估价');
  assert.equal(costPresentation([usage(0)],now).value,'$0.00');
  assert.match(costPresentation([known,usage(100)],now).value,/部分/);
  assert.match(costPresentation([known,{status:'unknown',snapshot:null,lastSuccessAtMs:null,message:null}],now).value,/部分/);
  assert.match(costPresentation([{...known,status:'stale'}],now).value,/上次/);
  assert.equal(costPresentation([known],now+86400000).value,'待估价');
  assert.equal(costPresentation([usage(100,{usd:NaN,pricedTokens:100,unpricedTokens:0})],now).value,'待估价');
  assert.equal(costPresentation([usage(100,{usd:1,pricedTokens:200,unpricedTokens:0})],now).value,'待估价');
  assert.match(costPresentation([usage(1,{usd:0.0001,pricedTokens:1,unpricedTokens:0})],now).value,/<\$0.01/);
});

test('three-column metrics separate numbers from scope and never invent zero for unknowns',()=>{
  const partial=costPresentation([usage(100,{usd:113.39,pricedTokens:90,unpricedTokens:10})],now);
  assert.equal(partial.amount,'≈$113.39');
  assert.equal(partial.note,'USD · 部分');
  assert.equal(costPresentation([usage(100)],now).amount,'—');
  assert.equal(costPresentation([usage(100)],now).note,'待估价');
  assert.equal(costPresentation([usage(0)],now).amount,'$0.00');
  assert.match(costPresentation([{...usage(100,{usd:1,pricedTokens:100,unpricedTokens:0}),status:'stale'}],now).note,/上次/);
  assert.equal(present(status)?.value,'#12');
  assert.equal(present(status)?.note,'今日 · 多工具');
  assert.equal(present({...status,remote:{...status.remote,enrollment:{...status.remote.enrollment,held:1}}})?.value,'—');
});

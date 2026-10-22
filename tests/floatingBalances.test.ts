import test from 'node:test';
import assert from 'node:assert/strict';
import { balanceStrip, orderedSources, moveSource, normalizeOrder, canStartBalanceDrag, FLOATING_ORDER_KEY } from '../src/domain/floatingBalances.ts';
import { floatingWindowSize } from '../src/domain/floating.ts';
const window = (id:string,remaining:number|null) => ({poolId:'codex',windowId:id,label:id,usedPercent:null,remainingPercent:remaining,resetsAtMs:null,durationMinutes:null,reachedType:null});

test('whole-card sorting admits primary presses but leaves controls and other pointers alone',()=>{
  const press={button:0,isPrimary:true,sourceCount:3,interactive:false,dragging:false};
  assert.equal(canStartBalanceDrag(press),true);
  assert.equal(canStartBalanceDrag({...press,sourceCount:1}),true,'single card still supports long-press editing');
  for(const override of [{button:2},{isPrimary:false},{sourceCount:0},{interactive:true},{dragging:true}]){
    assert.equal(canStartBalanceDrag({...press,...override}),false);
  }
});
test('one source chooses the tightest known window, preserves unknown and reports hidden windows',()=>{
  const row=balanceStrip({id:'codex',label:'Codex',windows:[window('5 小时',70),window('7 天',20),window('未知',null)],status:'剩余额度',update:'刚刚更新'},1000,20);
  assert.equal(row.amount,'20%');assert.equal(row.windowLabel,'7 天');assert.equal(row.additional,1);assert.equal(row.partial,true);assert.equal(row.low,true);
  assert.deepEqual(row.featured.map(item=>[item.label,item.amount]),[['5 小时','70%'],['7 天','20%']]);
  assert.equal(balanceStrip({id:'claude',label:'Claude',windows:[],status:'额度未接入',update:'未读取'},1000,20).amount,'—');
  assert.equal(balanceStrip({id:'codex',label:'Codex',windows:[window('7 天',0)],status:'',update:''},1000,20).amount,'0%');
});
test('floating quota keeps two windows from the same source pool, without inventing plan limits',()=>{
  const five={...window('5 小时额度',62),durationMinutes:300};
  const week={...window('7 天额度',11),durationMinutes:10080};
  const other={...window('别的模型 · 7 天',3),poolId:'model-x',durationMinutes:10080};
  const row=balanceStrip({id:'codex',label:'Codex',windows:[five,week,other],status:'',update:'读取于 19:00'},1000,20);
  assert.deepEqual(row.featured.map(item=>item.label),['5 小时','7 天']);
  assert.equal(row.additional,1);
  assert.equal(row.hiddenLow,true);
  const standard=balanceStrip({id:'claude',label:'Claude',windows:[five,week],status:'',update:''},1000,20);
  assert.deepEqual(standard.featured.map(item=>item.amount),['62%','11%']);
  assert.equal(standard.featured[1].low,true);
  const claude=balanceStrip({id:'claude',label:'Claude',windows:[
    {...five,poolId:'claude:five_hour'},
    {...week,poolId:'claude:seven_day'},
    {...other,poolId:'claude:seven_day_opus'},
  ],status:'',update:''},1000,20);
  assert.deepEqual(claude.featured.map(item=>item.amount),['62%','11%']);
  assert.equal(claude.additional,1);
  assert.equal(balanceStrip({id:'claude',label:'Claude',windows:[],status:'',update:''},1000,20).featured.length,0);
});
test('credits stay exact amounts, with stale and expiry independent of percentage quotas',()=>{
  const row=balanceStrip({id:'workbuddy',label:'WorkBuddy',windows:[],status:'',update:'',credits:{status:'available',supported:true,authorized:true,refreshing:false,identityLabel:'test',message:'',balance:{remaining:'1883.34',packages:[],collectedAtMs:1000,nextExpiryAtMs:1500}}},2000,20);
  assert.equal(row.amount,'1,883.34');assert.equal(row.unit,'积分');assert.equal(row.percent,null);assert.equal(row.stale,true);assert.equal(row.low,false);
});
test('floating credits distinguish restoring authorization from needing a new login',()=>{
  const row=balanceStrip({id:'workbuddy',label:'WorkBuddy',windows:[],status:'',update:'',credits:{status:'loading',supported:true,authorized:false,refreshing:true,identityLabel:null,message:'正在恢复授权',balance:null}},2000,20);
  assert.equal(row.amount,'—');
  assert.equal(row.detail,'正在恢复授权');
});
test('ordering removes duplicates, appends new sources, limits four and keeps inputs immutable',()=>{
  const sources=['a','b','c','d','e'].map(id=>({id}));
  assert.deepEqual(orderedSources(sources,['c','c','unknown','a']).map(s=>s.id),['c','a','b','d']);
  const order=['a','b','c'];assert.deepEqual(moveSource(order,'c','a'),['c','a','b']);assert.deepEqual(order,['a','b','c']);
  assert.deepEqual(moveSource(order,'missing','a'),order);assert.deepEqual(normalizeOrder(['a',null,1,'a','b']),['a','b']);
  assert.deepEqual(normalizeOrder({bad:true}),[]);assert.ok(FLOATING_ORDER_KEY.startsWith('tokendaily.'));
});
test('native layout fits up to four rows and preserves collapsed ball geometry',()=>{
  assert.equal(floatingWindowSize(true,false,true,4).height,422);
  assert.equal(floatingWindowSize(true,false,true,9).height,422);
  assert.equal(floatingWindowSize(true,false,false,1).height,236);
  assert.deepEqual(floatingWindowSize(false,true,true,4),{width:28,height:56});
});

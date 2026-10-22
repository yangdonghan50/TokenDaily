import test from 'node:test';
import assert from 'node:assert/strict';
import { canRefreshCredits, creditPresentation, formatCreditAmount, type CreditState } from '../src/domain/credits.ts';
const state: CreditState = { status:'available', supported:true, authorized:true, refreshing:false, identityLabel:'测试', message:'', balance:{remaining:'123.456789', collectedAtMs:1000, nextExpiryAtMs:null, packages:[]} };
test('credits display two decimals without altering raw balances or unknown state',()=>{
  assert.equal(creditPresentation(state,1001).amount,'123.46');
  assert.equal(state.balance!.remaining,'123.456789');
  assert.equal(creditPresentation({...state,balance:null},1001).amount,'—');
  assert.equal(creditPresentation({...state,balance:{...state.balance!,remaining:'0'}},1001).amount,'0.00');
});

test('decimal string rounding handles carry, small values, exact halves and large balances',()=>{
  for(const [raw,shown] of [['1234.56789','1,234.57'],['1.005','1.01'],['999.9999','1,000.00'],['123','123.00'],['0.1','0.10'],['0.0001','0.00'],['9007199254740993.125','9,007,199,254,740,993.13']])assert.equal(formatCreditAmount(raw),shown);
  for(const raw of [null,undefined,'','-1','NaN','Infinity','1e3','12.bad'])assert.equal(formatCreditAmount(raw),'—');
});
test('failed, old and expired snapshots remain visibly historical',()=>{
  assert.equal(creditPresentation({...state,status:'stale'},1001).stale,true);
  assert.equal(creditPresentation(state,601001).stale,true);
  assert.equal(creditPresentation({...state,balance:{...state.balance!,nextExpiryAtMs:2000}},2000).stale,true);
  assert.equal(creditPresentation(state,2000).stale,false);
});
test('startup restoration is distinct from a missing authorization',()=>{
  assert.equal(creditPresentation({...state,status:'loading',authorized:false,balance:null,refreshing:true},1001).label,'正在恢复授权');
  assert.equal(creditPresentation({...state,status:'auth-required',authorized:false,balance:null},1001).label,'积分待授权');
});
test('restored last balance shows its calendar date and is never presented as current',()=>{
  const readAt=Date.UTC(2026,9,19,2,30);
  const view=creditPresentation({...state,status:'stale',balance:{...state.balance!,collectedAtMs:readAt}},readAt+86400000);
  assert.equal(view.label,'上次积分余额');
  assert.match(view.update,/10\/19.*10:30.*待更新/);
});
test('manual refresh is offered for recoverable credit states only',()=>{
  for(const status of ['loading','available','stale','error']) assert.equal(canRefreshCredits({...state,status}),true);
  for(const status of ['auth-required','authorizing','unsupported']) assert.equal(canRefreshCredits({...state,status}),false);
  assert.equal(canRefreshCredits({...state,refreshing:true}),false);
  assert.equal(canRefreshCredits({...state,supported:false}),false);
});

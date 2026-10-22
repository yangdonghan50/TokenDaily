import test from 'node:test';
import assert from 'node:assert/strict';
import { normalizeHidden, setComponentVisible, visibleComponents, createComponentPress, mergeVisibleOrder } from '../src/domain/floatingComponents.ts';

test('visibility is independent of source data and legacy order; empty is intentional',()=>{
  const sources=['codex','workbuddy','claude'].map(id=>({id,tokens:10}));
  assert.deepEqual(normalizeHidden(null),[]);
  assert.deepEqual(normalizeHidden(['claude','claude',null,'future']),['claude']);
  const hidden=setComponentVisible([], 'claude', false);
  assert.deepEqual(visibleComponents(sources,hidden).map(s=>s.id),['codex','workbuddy']);
  assert.equal(sources.reduce((sum,s)=>sum+s.tokens,0),30);
  assert.deepEqual(setComponentVisible(hidden,'claude',false),hidden);
  assert.deepEqual(setComponentVisible(hidden,'claude',true),[]);
  assert.deepEqual(setComponentVisible(hidden,'unsupported',false),hidden);
  assert.deepEqual(visibleComponents(sources,['codex','claude','workbuddy']),[]);
  assert.deepEqual(mergeVisibleOrder(['codex','claude','workbuddy'],['workbuddy','codex']),['workbuddy','claude','codex']);
});

function harness(){
 let time=0,id=0,edits=0;const jobs=new Map<number,{at:number;run:()=>void}>();
 const press=createComponentPress(()=>edits++,{schedule:(run,delay)=>{jobs.set(++id,{at:time+delay,run});return id;},cancel:id=>{jobs.delete(id);}});
 return {press,get edits(){return edits;},tick(ms:number){time+=ms;for(const [id,job] of jobs)if(job.at<=time){jobs.delete(id);job.run();}}};
}
test('stationary long press enters edit once; release never commits a drag',()=>{
 const h=harness();h.press.down(1,0,0);h.tick(599);assert.equal(h.edits,0);
 h.tick(1);assert.equal(h.edits,1);h.tick(1000);assert.equal(h.edits,1);
 assert.equal(h.press.move(1,100,100),false);assert.equal(h.press.up(1),false);
});
test('movement in either direction cancels long press permanently; tiny jitter is tolerated',()=>{
 const h=harness();h.press.down(1,0,0);assert.equal(h.press.move(1,2,1),false);h.tick(100);
 assert.equal(h.press.move(1,4,0),true);h.tick(1000);assert.equal(h.edits,0);
 assert.equal(h.press.move(1,0,0),true);assert.equal(h.press.up(1),true);
 h.press.down(1,0,0);assert.equal(h.press.move(1,0,4),true);h.tick(600);assert.equal(h.edits,0);
});
test('wrong pointer, short click, cancellation and replaced press cannot trigger stale editing',()=>{
 const h=harness();h.press.down(1,0,0);assert.equal(h.press.move(2,50,50),false);assert.equal(h.press.up(2),false);
 h.tick(300);assert.equal(h.press.up(1),false);h.tick(600);assert.equal(h.edits,0);
 h.press.down(1,0,0);h.press.cancel();h.tick(600);assert.equal(h.edits,0);
 h.press.down(1,0,0);h.tick(300);h.press.down(2,0,0);h.tick(300);assert.equal(h.edits,0);h.tick(300);assert.equal(h.edits,1);
});

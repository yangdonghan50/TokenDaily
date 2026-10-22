import test from 'node:test';
import assert from 'node:assert/strict';
import { rankHonor } from '../src/domain/leaderboardHonors.ts';
import { rankPresentation, type RankStatus } from '../src/domain/floatingSummary.ts';
test('floating honors preserve complete tie boundaries, zero and older API behavior',()=>{
 for(const [rank,end,total,id] of [[1,1,1,'gold'],[2,2,100,'silver'],[3,3,100,'bronze'],[4,4,100,'stellar'],[5,6,100,'elite'],[10,11,100,'progress'],[25,26,100,'rising'],[50,50,100,'rising'],[50,51,100,null],[51,51,100,null]] as const){
  assert.equal(rankHonor({rank,rankEnd:end,tokens:1},total)?.id??null,id);
 }
 for(const row of [{rank:1,rankEnd:1,tokens:0},{rank:1,tokens:1},{rank:1,rankEnd:2,tokens:1},{rank:null,rankEnd:1,tokens:1}])assert.equal(rankHonor(row,1),null);
 assert.equal(rankHonor({rank:1,rankEnd:1,tokens:1},undefined),null);
});
test('status gates honors and preserves daily scope and paused uploaded usage',()=>{
 const now=Date.parse('2026-10-22T04:00:00Z');
 const status:RankStatus={supported:true,loggedIn:true,enabled:true,thisDevice:'a',remote:{enrollment:{device:'a',held:0,active:1},board:{period:'day',tool:'all',start:'2026-10-22',end:'2026-10-22',honorTotal:100,me:{rank:5,rankEnd:5,tokens:200}}}};
 assert.equal(rankPresentation(status,now)?.honor?.id,'stellar');
 assert.match(rankPresentation(status,now)!.title,/今日荣誉：星耀 · 前 5%/);
 status.remote!.enrollment!.active=0;assert.equal(rankPresentation(status,now)?.honor?.id,'stellar');
 status.remote!.enrollment!.held=1;assert.equal(rankPresentation(status,now)?.honor,null);
 status.remote!.enrollment!.held=0;status.remote!.board!.me!.tokens=0;
 assert.equal(rankPresentation(status,now)?.honor,null);assert.equal(rankPresentation(status,now)?.value,'—');
 status.loggedIn=false;assert.equal(rankPresentation(status,now),null);
});

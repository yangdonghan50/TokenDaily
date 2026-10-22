import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { rankPresentation, type RankStatus } from '../domain/floatingSummary.ts';
import { beijingToday } from '../domain/localUsage.ts';

export function useFloatingRank(active:boolean,now:number,demo:boolean) {
  const [revision,setRevision]=useState(0);
  const [result,setResult]=useState<{status:RankStatus;at:number;day:string}|null>(null);
  const day=beijingToday(now);
  useEffect(()=>{
    if(demo)return;
    const subscription=listen('leaderboard-updated',()=>{setResult(null);setRevision(n=>n+1);});
    return ()=>{void subscription.then(unlisten=>unlisten()).catch(()=>{});};
  },[demo]);
  useEffect(()=>{
    if(!active||demo)return;
    let disposed=false;
    // Cache only completed reads. Closing during a request must not suppress
    // the next opening for five minutes with an expired/missing result.
    if(result?.day===day && now-result.at<300_000)return;
    void invoke<RankStatus>('leaderboard_action',{action:'daily-status',body:{}})
      .then(status=>{if(!disposed)setResult({status,at:Date.now(),day});})
      .catch(()=>{if(!disposed)setResult(null);});
    return ()=>{disposed=true;};
    // A minute tick while expanded, not each quota update; hidden windows do no polling.
  },[active,demo,day,Math.floor(now/60_000),revision]);
  return {identity:result?.status??null,rank:result && result.day===day && now-result.at<300_000?rankPresentation(result.status,now):null,
    refresh:()=>{setResult(null);setRevision(n=>n+1);}};
}

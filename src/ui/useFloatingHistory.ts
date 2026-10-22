import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { createHistoryCache, emptyHistory } from '../domain/floatingCalendar.ts';
import { beijingToday } from '../domain/localUsage.ts';
import type { UsageState } from '../domain/usage.ts';

export function useFloatingHistory(active: boolean, now: number, demoUsage?: UsageState) {
  const cache = useRef(createHistoryCache(() => '__TAURI_INTERNALS__' in window
    ? invoke<UsageState>('get_local_total_history')
    : Promise.resolve({...emptyHistory,message:'请在桌面应用中读取本机历史日志。'})));
  const [usage,setUsage] = useState(emptyHistory);
  const [loading,setLoading] = useState(false);
  const [revision,setRevision] = useState(0);
  const force = useRef(false);
  const date = beijingToday(now);
  useEffect(()=>{
    if (!active || demoUsage) return;
    let disposed=false;
    setLoading(true);
    const request=cache.current.read(force.current);force.current=false;
    void request.then(result=>{if(!disposed){setUsage(result);setLoading(false);}});
    return ()=>{disposed=true;};
  },[active,date,revision,demoUsage]);
  return {usage:demoUsage??usage,loading:demoUsage?false:loading,
    refresh:()=>{force.current=true;setRevision(value=>value+1);}};
}

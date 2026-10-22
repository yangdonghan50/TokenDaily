import { beijingToday } from './localUsage.ts';
import { usageCalendar, type UsageState } from './usage.ts';

export const HISTORY_CACHE_MS = 300_000;
export const FLOATING_CALENDAR_WEEKS = 20;
export const emptyHistory: UsageState = {status:'unknown',snapshot:null,lastSuccessAtMs:null,message:null};
export function calendarPresentation(usage: UsageState, now: number) {
  const today = beijingToday(now);
  const cells = usageCalendar(usage.snapshot?.days ?? [], today, FLOATING_CALENDAR_WEEKS);
  const stamp = usage.lastSuccessAtMs;
  return {today,cells,max:Math.max(0,...cells.filter(c=>!c.future).map(c=>c.tokens??0)),
    stale:usage.status !== 'available' || stamp === null || now-stamp > HISTORY_CACHE_MS || stamp-now > 5_000};
}

// Demand driven: no timers or background log scans. Reopening reuses a recent
// result, but a Beijing date change invalidates it even within the TTL.
export function createHistoryCache(load: () => Promise<UsageState>, clock = Date.now) {
  let value = emptyHistory, expires = 0, date = '';
  let pending: Promise<UsageState> | null = null;
  return { read(force = false): Promise<UsageState> {
    if (pending) return pending;
    const now = clock();
    if (!force && now < expires && date === beijingToday(now)) return Promise.resolve(value);
    const requestedDate = beijingToday(now);
    pending = Promise.resolve().then(load).catch((): UsageState => ({...value,
      status:value.snapshot ? 'stale' : 'error',message:'历史读取失败，可重试；已有记录保留。'}))
      .then(next => {value=next;date=requestedDate;expires=clock()+(next.status==='error'||next.status==='stale'?30_000:HISTORY_CACHE_MS);return value;})
      .finally(()=>{pending=null;});
    return pending;
  }};
}

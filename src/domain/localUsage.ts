import type { UsageState } from './usage.ts';

export function beijingToday(now: number) {
  return new Date(now + 8 * 3600_000).toISOString().slice(0, 10);
}

export function localTodayUsage(usage: UsageState, now: number): UsageState {
  const today = beijingToday(now);
  const day = usage.snapshot?.days.find(day => day.date === today);
  const fresh = usage.status === 'available' && usage.lastSuccessAtMs !== null
    && now - usage.lastSuccessAtMs <= 90_000 && now - usage.lastSuccessAtMs >= -5_000;
  if (day) return { ...usage, status: fresh ? 'available' : usage.status === 'available' ? 'stale' : usage.status,
    snapshot: { ...usage.snapshot!, days: [day] } };
  // A healthy collector crosses midnight before its next disk poll. Yesterday's
  // tokens never carry over. Errors, sleep and initial scans must not invent zero.
  if (fresh && usage.snapshot?.days.at(-1) && usage.snapshot.days.at(-1)!.date < today) return {
    ...usage, snapshot: { ...usage.snapshot, apiCost: null, days: [{ date: today, tokens: 0 }] },
  };
  return { ...usage, snapshot: null, status: usage.status === 'available' ? 'unknown' : usage.status };
}

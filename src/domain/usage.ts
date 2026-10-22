export type UsageDay = { date: string; tokens: number };
export type UsageSnapshot = { identityKey: string; collectedAtMs: number; days: UsageDay[]; apiCost?: {usd:number;pricedTokens:number;unpricedTokens:number} | null };
export type UsageState = { status: 'unknown' | 'available' | 'stale' | 'error'; snapshot: UsageSnapshot | null; lastSuccessAtMs: number | null; message: string | null; sourceUnavailable?: boolean };
export function participatesInTotal(usage:UsageState,enabled=true) {
  return enabled && !(usage.sourceUnavailable===true && !usage.snapshot && usage.lastSuccessAtMs===null);
}

// Compare date labels only. The provider's unspecified timezone must not be
// inferred from the machine timezone or used to relabel historical counts.
export function usageDatePresentation(days: UsageDay[], now: number, timeZone?: string) {
  const parts = new Intl.DateTimeFormat('en-US', { timeZone, year: 'numeric', month: '2-digit', day: '2-digit' }).formatToParts(now);
  const part = (type: string) => parts.find(value => value.type === type)!.value;
  const today = `${part('year')}-${part('month')}-${part('day')}`;
  const latest = days.at(-1);
  const shortDate = (date: string) => (date.slice(0, 4) === today.slice(0, 4) ? date.slice(5) : date).replaceAll('-', '/');
  const notice = latest && latest.date > today ? '来源日期晚于本地日期，口径待确认'
    : days.some(day => day.date === today) ? null : `尚未取得 ${shortDate(today)} 日记录`;
  return { label: latest ? `${shortDate(latest.date)} Token 记录` : 'Token 记录未知', notice };
}

export function usageDaysEndingOn(days: UsageDay[], endDate: string, count: number): (UsageDay | { date: string; tokens: null })[] {
  const end = new Date(`${endDate}T00:00:00Z`);
  if (!Number.isFinite(end.getTime()) || count < 1) return [];
  const byDate = new Map(days.map((day) => [day.date, day.tokens]));
  return Array.from({ length: count }, (_, index) => {
    const date = new Date(end);
    date.setUTCDate(end.getUTCDate() - count + index + 1);
    const iso = date.toISOString().slice(0, 10);
    return { date: iso, tokens: byDate.get(iso) ?? null };
  });
}

export function usageCalendar(days: UsageDay[], endDate: string, weeks = 13) {
  const end = new Date(`${endDate}T00:00:00Z`);
  if (!Number.isFinite(end.getTime())) return [];
  const mondayOffset = (end.getUTCDay() + 6) % 7;
  end.setUTCDate(end.getUTCDate() + (6 - mondayOffset));
  const cells = usageDaysEndingOn(days, end.toISOString().slice(0, 10), weeks * 7);
  return cells.map((cell) => ({ ...cell, future: cell.date > endDate }));
}

export function usageLevel(tokens: number | null, max: number): number | null {
  if (tokens === null) return null;
  if (tokens === 0) return 0;
  if (max <= 0) return 1;
  return Math.min(4, Math.max(1, Math.ceil((tokens / max) * 4)));
}

export { formatTokenCount, formatExactTokenCount } from './tokenFormat.ts';

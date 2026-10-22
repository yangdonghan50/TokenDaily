import { localTodayUsage } from './localUsage.ts';
import { participatesInTotal, type UsageState } from './usage.ts';

export function combinedUsage(codex: UsageState, claude: UsageState, now: number, workbuddy?: UsageState, enabled: boolean[] = [], trae?: UsageState) {
  const sources: [string, UsageState][] = [['Codex', codex], ['Claude Code', claude]];
  if (workbuddy) sources.push(['WorkBuddy', workbuddy]);
  if (trae) sources.push(['TRAE 本地记录',trae]);
  const entries = sources.flatMap(([label,raw],index)=>participatesInTotal(raw,enabled[index]??true)?[{label,sourceIndex:index,usage:localTodayUsage(raw,now)}]:[]);
  const known = entries.filter(({ usage }) => usage.snapshot?.days.at(-1));
  const missing = entries.filter(({ usage }) => !usage.snapshot?.days.at(-1)).map(e => e.label);
  const stale = known.some(({ usage }) => usage.status !== 'available');
  const total = known.reduce((sum, { usage }) => sum + usage.snapshot!.days.at(-1)!.tokens, 0);
  const complete = known.length > 0 && known.length === entries.length && !stale && Number.isSafeInteger(total);
  const message = !Number.isSafeInteger(total) ? '合计超出可表示范围' : missing.length ? `统计不完整 · ${missing.join('、')} 用量未读取` : stale ? '含上次记录 · 等待更新' : null;
  const timestamp = known.length ? Math.min(...known.map(({ usage }) => usage.lastSuccessAtMs ?? usage.snapshot!.collectedAtMs)) : null;
  const usage: UsageState = {
    status: !known.length || !Number.isSafeInteger(total) ? 'unknown' : stale ? 'stale' : 'available',
    snapshot: known.length && Number.isSafeInteger(total) ? { identityKey: `local:${sources.map(([label]) => label).join('+')}`, collectedAtMs: timestamp!, days: [{ date: known[0].usage.snapshot!.days.at(-1)!.date, tokens: total }] } : null,
    lastSuccessAtMs: timestamp, message,
  };
  return { usage, complete, missing, entries };
}

import { normalizeQuotaResponse, type QuotaSnapshot } from './domain/quota.ts';
import type { UsageState } from './domain/usage.ts';

export type DemoScenario = 'normal' | 'single' | 'low' | 'unknown' | 'stale' | 'disconnected';
export const demoScenarios: { id: DemoScenario; label: string }[] = [
  { id: 'normal', label: '两个窗口' },
  { id: 'single', label: '单窗口' },
  { id: 'low', label: '低额度' },
  { id: 'unknown', label: '字段缺失' },
  { id: 'stale', label: '旧数据' },
  { id: 'disconnected', label: '未连接' },
];

export function demoSnapshot(scenario: DemoScenario, nowMs: number): QuotaSnapshot | null {
  if (scenario === 'disconnected') return null;
  const collectedAtMs = scenario === 'stale' ? nowMs - 20 * 60_000 : nowMs;
  const primary = { usedPercent: scenario === 'low' ? 93 : scenario === 'unknown' ? null : 73,
    windowDurationMins: 10080, resetsAt: Math.floor((nowMs + 52 * 3600_000) / 1000) };
  const secondary = { usedPercent: scenario === 'low' ? 100 : 36,
    windowDurationMins: 300, resetsAt: Math.floor((nowMs + 80 * 60_000) / 1000) };
  return normalizeQuotaResponse({ rateLimits: { limitId: 'codex',
    primary, secondary: scenario === 'single' || scenario === 'unknown' ? null : secondary } },
  { accountRef: 'demo-account', workspaceRef: null, collectedAtMs });
}

export function demoUsage(scenario: DemoScenario, nowMs: number): UsageState {
  if (scenario === 'disconnected') return { status: 'unknown', snapshot: null, lastSuccessAtMs: null, message: null };
  const collectedAtMs = scenario === 'stale' ? nowMs - 20 * 60_000 : nowMs;
  const end = new Date(nowMs);
  end.setUTCHours(0, 0, 0, 0);
  const days = Array.from({ length: 68 }, (_, index) => {
    const date = new Date(end);
    date.setUTCDate(end.getUTCDate() - 67 + index);
    return { date: date.toISOString().slice(0, 10), tokens: index % 11 === 0 ? 0 : 4_200 + (index * 3_211) % 32_000 };
  }).filter((_, index) => index % 17 !== 0);
  return { status: scenario === 'stale' ? 'stale' : 'available',
    snapshot: { identityKey: 'codex:demo-account:', collectedAtMs, days },
    lastSuccessAtMs: collectedAtMs, message: null };
}

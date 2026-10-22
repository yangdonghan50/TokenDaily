import type { UsageState } from './usage.ts';

export function floatingUsage(usage: UsageState, now: number) {
  const timestamp = usage.lastSuccessAtMs;
  return {
    day: usage.snapshot?.days.at(-1) ?? null,
    stale: usage.status !== 'available' || timestamp === null || !Number.isFinite(timestamp)
      || now - timestamp > 600_000 || now - timestamp < -5_000,
  };
}

export { compactTokens } from './tokenFormat.ts';

export type QuotaWindow = {
  poolId: string;
  windowId: 'primary' | 'secondary';
  label: string;
  usedPercent: number | null;
  remainingPercent: number | null;
  resetsAtMs: number | null;
  durationMinutes: number | null;
  reachedType: string | null;
};

export type QuotaSnapshot = {
  identityKey: string;
  status: 'available' | 'unknown';
  collectedAtMs: number;
  windows: QuotaWindow[];
};

type RecordValue = Record<string, unknown>;
const asRecord = (value: unknown): RecordValue | null =>
  value !== null && typeof value === 'object' && !Array.isArray(value) ? value as RecordValue : null;

function validPercent(value: unknown): number | null {
  return typeof value === 'number' && Number.isFinite(value) && value >= 0 && value <= 100 ? value : null;
}

function validDuration(value: unknown): number | null {
  return typeof value === 'number' && Number.isInteger(value) && value > 0 ? value : null;
}

function epochSecondsToMs(value: unknown): number | null {
  // The documented Codex field is Unix seconds. Reject a millisecond value instead of guessing.
  if (typeof value !== 'number' || !Number.isInteger(value) || value < 0 || value > 100_000_000_000) return null;
  return value * 1000;
}

function windowLabel(duration: number | null, poolName: string | null, windowId: string): string {
  const durationText = duration === null ? null
    : duration % 1440 === 0 ? `${duration / 1440} 天额度`
    : duration % 60 === 0 ? `${duration / 60} 小时额度`
    : `${duration} 分钟额度`;
  if (poolName && durationText) return `${poolName} · ${durationText}`;
  return poolName ?? durationText ?? `${windowId === 'primary' ? '主' : '次'}额度窗口`;
}

export function normalizeQuotaResponse(
  raw: unknown,
  context: { accountRef: string; workspaceRef: string | null; collectedAtMs: number },
): QuotaSnapshot {
  const root = asRecord(raw) ?? {};
  const buckets = new Map<string, RecordValue>();
  const multi = asRecord(root.rateLimitsByLimitId);
  if (multi) for (const [key, value] of Object.entries(multi)) {
    const bucket = asRecord(value);
    if (bucket) buckets.set(key, bucket);
  }
  const single = asRecord(root.rateLimits);
  if (single) {
    const id = typeof single.limitId === 'string' && single.limitId ? single.limitId : 'default';
    if (!buckets.has(id)) buckets.set(id, single);
  }
  const windows: QuotaWindow[] = [];
  for (const [poolId, bucket] of buckets) {
    const name = typeof bucket.limitName === 'string' && bucket.limitName.trim() ? bucket.limitName.trim() : null;
    const reachedType = typeof bucket.rateLimitReachedType === 'string' && bucket.rateLimitReachedType
      ? bucket.rateLimitReachedType : null;
    for (const windowId of ['primary', 'secondary'] as const) {
      const source = asRecord(bucket[windowId]);
      if (!source) continue;
      const usedPercent = validPercent(source.usedPercent);
      const durationMinutes = validDuration(source.windowDurationMins);
      windows.push({
        poolId, windowId,
        label: windowLabel(durationMinutes, name, windowId),
        usedPercent,
        remainingPercent: usedPercent === null ? null : 100 - usedPercent,
        resetsAtMs: epochSecondsToMs(source.resetsAt),
        durationMinutes,
        reachedType,
      });
    }
  }
  return {
    identityKey: `codex:${context.accountRef}:${context.workspaceRef ?? ''}`,
    status: windows.some((window) => window.remainingPercent !== null) ? 'available' : 'unknown',
    collectedAtMs: context.collectedAtMs,
    windows,
  };
}

export function quotaFreshness(snapshot: QuotaSnapshot, nowMs: number, staleAfterMs = 600_000): {
  fresh: boolean; resetElapsed: boolean;
} {
  const resetElapsed = snapshot.windows.some((window) => window.resetsAtMs !== null && nowMs >= window.resetsAtMs);
  const age = nowMs - snapshot.collectedAtMs;
  // UI clocks can be sampled just before a newly collected snapshot reaches the view.
  return { fresh: age >= -5000 && age <= staleAfterMs && !resetElapsed, resetElapsed };
}

export function formatRemaining(value: number | null): string {
  if (value === null) return '未知';
  if (value > 0 && value < 1) return '<1%';
  return `${Math.round(value)}%`;
}

export function resetPresentation(resetAtMs: number | null, nowMs: number, timeZone?: string): {
  relative: string; absolute: string | null;
} {
  if (resetAtMs === null || !Number.isFinite(resetAtMs)) return { relative: '重置时间未知', absolute: null };
  const absolute = new Intl.DateTimeFormat('zh-CN', {
    timeZone, month: 'numeric', day: 'numeric', hour: '2-digit', minute: '2-digit', hour12: false,
  }).format(new Date(resetAtMs));
  if (nowMs >= resetAtMs) return { relative: '预计重置时间已到，等待新数据确认', absolute };
  const minutes = Math.ceil((resetAtMs - nowMs) / 60_000);
  const days = Math.floor(minutes / 1440);
  const hours = Math.floor((minutes % 1440) / 60);
  const mins = minutes % 60;
  const parts = [days && `${days} 天`, hours && `${hours} 小时`, mins && `${mins} 分钟`].filter(Boolean);
  return { relative: `${parts.join(' ')}后重置`, absolute };
}

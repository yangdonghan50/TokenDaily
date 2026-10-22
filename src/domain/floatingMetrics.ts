export const METRIC_ORDER_KEY = 'tokendaily.floating-metric-order.v1';
export const METRIC_IDS = ['usage', 'cost', 'rank'] as const;
export const METRIC_OPTIONS = [
  { id: 'usage', label: '今日 Token' }, { id: 'cost', label: '预估费用' }, { id: 'rank', label: '排行' },
];
export function normalizeMetricOrder(value: unknown): string[] {
  const saved = Array.isArray(value) ? value.filter(id => METRIC_IDS.includes(id)) : [];
  return [...new Set([...saved, ...METRIC_IDS])];
}

type MetricUpdate = {
  status: string; stale: boolean; failed: boolean; hasData: boolean; lastSuccessAtMs: number | null;
};

export function metricUpdateLabel(metric: MetricUpdate, now: number): string {
  if (metric.status === 'disconnected') return '未连接';
  if (metric.status === 'auth-required') return '需要授权';
  if (metric.status === 'unsupported') return '暂不支持';
  if (metric.failed) return metric.hasData ? '读取失败 · 保留上次' : '读取失败 · 暂无数据';
  if (metric.stale && metric.hasData) return '上次数据 · 待读取';
  if (metric.status === 'unknown') return '数据未知';
  if (!metric.hasData) return '暂无数据';
  const time = metric.lastSuccessAtMs;
  if (time === null || !Number.isFinite(time) || time - now > 5000) return '读取时间未知';
  const minutes = Math.max(0, Math.floor((now - time) / 60_000));
  if (minutes === 0) return '刚刚读取';
  if (minutes < 60) return `${minutes} 分钟前读取`;
  if (minutes < 1440) return `${Math.floor(minutes / 60)} 小时前读取`;
  return `${Math.floor(minutes / 1440)} 天前读取`;
}

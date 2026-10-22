// Display-only: never use abbreviated values for storage, upload or ranking.
export function compactTokens(tokens: number) {
  if (!Number.isSafeInteger(tokens) || tokens < 0) return { value: '—', unit: '' };
  const divisor = tokens >= 100_000_000 ? 100_000_000 : tokens >= 10_000 ? 10_000 : 1;
  // Divide integer counts into hundredths first, avoiding floating-point underflow at 1.01万.
  const value = divisor === 1 ? tokens : Math.floor(tokens / (divisor / 100)) / 100;
  return { value: value.toLocaleString('zh-CN', { minimumFractionDigits: divisor === 1 ? 0 : 2,
    maximumFractionDigits: divisor === 1 ? 0 : 2 }), unit: divisor === 1 ? '' : divisor === 10_000 ? '万' : '亿' };
}

export function formatTokenCount(tokens: number): string {
  const { value, unit } = compactTokens(tokens);
  return value + unit;
}

export function formatExactTokenCount(tokens: number): string {
  return Number.isSafeInteger(tokens) && tokens >= 0 ? new Intl.NumberFormat('zh-CN').format(tokens) : '—';
}

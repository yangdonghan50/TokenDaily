export type CreditPackage = { name: string; remaining: string; expiresAtMs: number | null };
export type CreditState = { status: string; supported: boolean; authorized: boolean; refreshing: boolean; identityLabel: string | null; message: string;
  balance: { remaining: string; packages: CreditPackage[]; nextExpiryAtMs: number | null; collectedAtMs: number } | null };
export function canRefreshCredits(state: CreditState): boolean {
  return state.supported && !state.refreshing && ['loading','available','stale','error'].includes(state.status);
}
// Round only for presentation, using decimal strings to avoid binary float drift.
export function formatCreditAmount(raw: string | null | undefined): string {
  if (!raw || !/^\d+(?:\.\d+)?$/.test(raw)) return '—';
  const [whole, fraction = ''] = raw.split('.');
  let cents = BigInt(whole) * 100n + BigInt((fraction + '00').slice(0, 2));
  if (fraction.length > 2 && fraction[2] >= '5') cents += 1n;
  return (cents / 100n).toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',') + '.' + (cents % 100n).toString().padStart(2, '0');
}
export function creditPresentation(state: CreditState, now: number) {
  const balance = state.balance;
  const stale = !!balance && (state.status !== 'available' || now - balance.collectedAtMs > 600_000 || balance.nextExpiryAtMs !== null && balance.nextExpiryAtMs <= now);
  const amount = formatCreditAmount(balance?.remaining);
  const label = balance ? stale ? '上次积分余额' : '积分余额' : state.status === 'authorizing' ? '等待官方登录' : state.status === 'loading' ? state.authorized ? '积分待查询' : '正在恢复授权' : state.supported ? '积分待授权' : '当前平台未支持';
  const update = balance ? `${new Intl.DateTimeFormat('zh-CN',{timeZone:'Asia/Shanghai',...(stale?{month:'2-digit' as const,day:'2-digit' as const}:{}),hour:'2-digit',minute:'2-digit'}).format(balance.collectedAtMs)} 读取${stale ? ' · 待更新' : ''}` : label;
  const expiry = balance?.nextExpiryAtMs ? `${new Intl.DateTimeFormat('zh-CN',{timeZone:'Asia/Shanghai',month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}).format(balance.nextExpiryAtMs)} 部分积分到期（北京）` : '积分有效期未提供';
  return { amount, stale, label, update, expiry };
}

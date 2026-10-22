import type { Monitor } from './useMonitor.ts';
import { quotaFreshness, formatRemaining, resetPresentation } from '../domain/quota.ts';
import { TokenCount } from './TokenCount.tsx';
import { combinedUsage } from '../domain/combinedUsage.ts';
import { metricUpdateLabel } from '../domain/refresh.ts';
import type { Provider } from '../domain/provider.ts';
import { creditPresentation, type CreditState } from '../domain/credits.ts';
import { CreditCard } from './WorkBuddyCredits.tsx';

export function monitorStatus(monitor: Monitor, now: number, credits?: CreditState) {
  const state = monitor.state;
  const stale = state.status === 'stale' || Boolean(state.snapshot && !quotaFreshness(state.snapshot, now).fresh);
  const claudeUnlinked = monitor.provider === 'claude' && state.status === 'auth-required';
  const desktopHistory = monitor.provider === 'claude' && state.snapshot?.identityKey.startsWith('claude:desktop:');
  if ((monitor.provider === 'workbuddy'||monitor.provider === 'trae')) return credits && state.connected ? creditPresentation(credits,now) : { stale: false, label: state.connected ? '积分待授权' : '未连接', update: state.connected ? 'Token 独立采集' : '尚未连接' };
  return { stale, label: !state.connected ? '未连接' : claudeUnlinked ? '额度未接入' : state.status === 'auth-required' ? '需要登录' : state.status === 'loading' ? '读取中' : state.status === 'error' ? '读取失败' : !state.snapshot?.windows.length ? '额度未知' : stale ? '上次记录' : desktopHistory ? '本机快照' : '剩余额度',
    update: claudeUnlinked ? '未连接额度来源' : metricUpdateLabel({ status: state.status, stale, hasData: Boolean(state.snapshot?.windows.length), failed: state.status === 'error', lastSuccessAtMs: state.lastSuccessAtMs }, now) };
}
export function OverviewPanel({ monitors, now, threshold, onSelect, onHistory, onRefresh, refreshing, credits, traeCredits }: { monitors: Monitor[]; now: number; threshold: number; onSelect: (provider: Provider) => void; onHistory: () => void; onRefresh: () => void; refreshing: boolean; credits: CreditState; traeCredits:CreditState }) {
  const total = combinedUsage(monitors[0].usage, monitors[1].usage, now, monitors[2]?.usage, monitors.map(m=>m.state.connected),monitors[3]?.usage);
  const day = total.usage.snapshot?.days[0];
  return <>
    <section className="overview-quotas" aria-label="各来源剩余额度与积分">
      {monitors.map(monitor => {
        const status = monitorStatus(monitor, now, monitor.provider==='trae'?traeCredits:credits);
        return <article className="overview-provider" key={monitor.provider}>
          <div className="overview-provider-heading"><strong>{monitor.source.label}</strong><span>{status.label}</span></div>
          {(monitor.provider === 'workbuddy'||monitor.provider === 'trae') && monitor.state.connected ? <CreditCard state={monitor.provider==='trae'?traeCredits:credits} now={now} compact label={monitor.source.label}/> : monitor.state.snapshot?.windows.length ? monitor.state.snapshot.windows.map(window => <div className="overview-window" key={`${window.poolId}:${window.windowId}`}>
            <div><span>{window.label}</span><strong className={window.remainingPercent !== null && window.remainingPercent <= threshold ? 'is-low' : ''}>{formatRemaining(window.remainingPercent)}</strong></div>
            {window.remainingPercent !== null && <div className="quota-track" role="meter" aria-label={`${monitor.source.label} ${window.label}剩余`} aria-valuemin={0} aria-valuemax={100} aria-valuenow={window.remainingPercent}><div style={{ width: `${window.remainingPercent}%` }} /></div>}
            <small>{resetPresentation(window.resetsAtMs, now).relative}</small>
          </div>) : <div className="overview-unknown"><strong>—</strong><p>{monitor.state.message ?? '连接后读取账户剩余额度'}</p></div>}
          <footer><small>{status.update}</small>{!monitor.state.connected ? <button type="button" className="text-button" disabled={monitor.refreshing} onClick={monitor.connect}>连接</button> : <button type="button" className="text-button" onClick={() => onSelect(monitor.provider)}>详情 ↗</button>}</footer>
          {monitor.error && <p className="window-warning">{monitor.error}</p>}
        </article>;
      })}
    </section>
    <section className="overview-total" aria-label="今日 Token 合计">
      <div><span>{total.complete ? '今日 Token · 合计' : '今日 Token · 已统计'}</span><small>{monitors[3]?.state.connected ? '本地记录' : '本机'} · 含缓存</small></div>
      <strong><TokenCount tokens={day?.tokens} /><small> Tokens</small></strong>
      {total.usage.message && <p className="window-warning">{total.usage.message}</p>}
      <div className="overview-breakdown">{total.entries.map(entry => <button type="button" key={entry.label} onClick={() => onSelect(monitors[entry.sourceIndex].provider)}><span>{entry.label}</span><b><TokenCount tokens={entry.usage.snapshot?.days.at(-1)?.tokens} /></b></button>)}</div>
      <p>北京时间 00:00 起 · 不含 Claude 网页 / 桌面聊天 Token；TRAE 本地记录可能含恢复/同步会话。</p>
    </section>
    <footer className="panel-footer"><button type="button" className="text-button" onClick={onHistory}>总 Token 历史 ↗</button><button type="button" className="refresh-button" disabled={monitors.every(m => !m.state.connected) || refreshing} onClick={onRefresh}>{refreshing ? '刷新中…' : '刷新全部'}</button></footer>
  </>;
}

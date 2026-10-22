import { type QuotaWindow } from '../domain/quota.ts';
import { useFloatingHover } from './useFloatingHover.ts';
import { compactTokens, floatingUsage } from '../domain/floatingUsage.ts';
import { formatExactTokenCount, usageDatePresentation, type UsageState } from '../domain/usage.ts';
import { useEffect, useRef, useState, type ReactNode } from 'react';
import { listen } from '@tauri-apps/api/event';
import { nativeEscapeTarget } from '../domain/floating.ts';
import brandMark from '../../assets/icon.svg';
import { FloatingBalances, type FloatingComponent } from './FloatingBalances.tsx';
import { FloatingCalendar } from './FloatingCalendar.tsx';
import { useFloatingHistory } from './useFloatingHistory.ts';
import { orderedSources, type BalanceSource } from '../domain/floatingBalances.ts';
import { creditPresentation, type CreditState } from '../domain/credits.ts';
import { useFloatingComponents } from './useFloatingComponents.ts';
import { visibleComponents } from '../domain/floatingComponents.ts';
import { floatingRefreshState } from '../domain/floatingRefresh.ts';
import { costPresentation } from '../domain/floatingSummary.ts';
import { FloatingMetrics, type Metric } from './FloatingMetrics.tsx';
import { MetricNumber } from './MetricNumber.tsx';
import { METRIC_IDS } from '../domain/floatingMetrics.ts';
import { floatingIdentity } from '../domain/floatingIdentity.ts';
import { useFloatingRank } from './useFloatingRank.ts';
import { RankHonorIcon } from './RankHonorIcon.tsx';
import { UpdateButton } from './UpdateButton.tsx';
import './floatingSummary.css';

type Props = {
  credits?: CreditState;
  balanceSources?: BalanceSource[];
  usageLabel?: string; usageBreakdown?: ReactNode;
  windows: QuotaWindow[]; now: number; threshold: number;
  stale: boolean; error: string | null; demo: boolean; busy: boolean;
  onClose: () => void; onOpen: () => void;
  onDetail: (kind: 'provider' | 'history' | 'rank' | 'cost', provider?: string) => void;
  onConfigure?: () => void;
  onIdentity: () => void;
  usage: UsageState;
  costSources?: UsageState[];
  refreshing: boolean; canRefresh: boolean; onRefresh: () => void;
  usageUpdate: string;
  edgeHide: boolean;
};

export function FloatingPanel({ windows, now, threshold, stale, error, demo, busy, onClose, onOpen, onDetail,
  usage, refreshing, canRefresh, onRefresh, edgeHide,
  usageLabel, credits, balanceSources, costSources, onConfigure = onOpen, onIdentity }: Props) {
  const preferences = useFloatingComponents(!demo);
  const [editing, setEditing] = useState(false);
  const host = useRef<HTMLDivElement>(null);
  // macOS keeps the floating panel non-activating, so Esc reaches the app's key
  // window instead. The native monitor forwards it while the pointer is on the panel.
  useEffect(() => {
    if (!('__TAURI_INTERNALS__' in window)) return;
    const unlisten = listen('floating-escape', () => {
      const root = host.current;
      if (!root) return;
      const active = document.activeElement instanceof HTMLElement ? document.activeElement : null;
      nativeEscapeTarget<HTMLElement>(active, root, (parent, node) => parent.contains(node))
        .dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true, cancelable: true }));
    });
    return () => { void unlisten.then(stop => stop()); };
  }, []);
  const sources: FloatingComponent[] = [...orderedSources(balanceSources ?? [], []), {id:'calendar',label:'Token 日历',status:'本机总 Token · 最近 20 周',content:null}];
  const visible = visibleComponents(sources, preferences.hidden);
  const calendarVisible = !preferences.hidden.includes('calendar');
  const { expanded, hiddenEdge, resizeError, handlers, drag, ballPointer, collapse, keyboard } = useFloatingHover(edgeHide, true, editing ? sources.length : visible.length, editing, editing || calendarVisible, METRIC_IDS.filter(id=>!preferences.hidden.includes(id)).length);
  const history = useFloatingHistory(expanded && calendarVisible, now, demo ? usage : undefined);
  const ranking = useFloatingRank(expanded,now,demo);
  const identity = floatingIdentity(ranking.identity);
  const cost=costPresentation(costSources??[usage],now);
  const {busy:panelRefreshing,enabled:panelCanRefresh} = floatingRefreshState(canRefresh,refreshing,calendarVisible,history.loading);
  function refreshPanel() {
    ranking.refresh();
    if (calendarVisible) history.refresh();
    if (canRefresh) onRefresh();
  }
  function collapsePanel() {
    setEditing(false);
    collapse.current();
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
  }
  sources[sources.length-1] = {id:'calendar',label:'Token 日历',status:'本机总 Token · 最近 20 周',content:<FloatingCalendar {...history} now={now}/>};
  useEffect(() => { if (!expanded) setEditing(false); }, [expanded]);
  const { day, stale: usageStale } = floatingUsage(usage, now);
  const amount = day ? compactTokens(day.tokens) : null;
  const dateView = demo ? usageDatePresentation(usage.snapshot?.days ?? [], now)
    : { label: '今日 Token', notice: usage.message ?? (usage.status === 'available' ? null : day ? '采集暂停 · 保留本日记录' : '等待本机日志 · 连接后自动采集') };
  const hasRisk = windows.some(quota => quota.reachedType || quota.remainingPercent !== null && quota.remainingPercent <= threshold);
  const attention = stale || Boolean(credits && creditPresentation(credits,now).stale) || Boolean(balanceSources?.some(s=>s.credits&&creditPresentation(s.credits,now).stale)) || Boolean(day && usageStale) || hasRisk || Boolean(error || resizeError);
  const metrics: Metric[] = [
    {id:'usage',label:'今日 Token',stale:usageStale,title:[day ? `${usageLabel ?? dateView.label} · ${day.date} · ${formatExactTokenCount(day.tokens)} Tokens${usageStale?' · 上次记录':''}` : 'Token 用量未知',
      dateView.notice, 'Codex、Claude Code、WorkBuddy 本机日志与 TRAE 本地数据库，含缓存，按北京时间归日；TRAE 恢复/同步的会话可能来自其他设备。不含 Claude 网页或桌面聊天。'].filter(Boolean).join('。'),
      content:<MetricNumber value={amount?.value ?? '—'} unit={amount?.unit}/>},
    {id:'cost',label:'预估费用',stale:cost.note.includes('上次'),title:cost.title,
      content:<MetricNumber value={cost.amount.replace(/^≈/,'')}/>},
    {id:'rank',label:'今日排名',title:[ranking.rank?.note,ranking.rank?.title ?? '暂无可用排名，仅显示北京时间当日多工具总榜的有效排名；不会自动报名或上传。'].filter(Boolean).join('。'),
      content:<MetricNumber value={ranking.rank?.value ?? '—'} addon={ranking.rank?.honor?<RankHonorIcon honor={ranking.rank.honor}/>:undefined}/>},
  ];
  return <div ref={host} className={`floating-host ${expanded ? 'is-expanded' : hiddenEdge ? `is-edge edge-${hiddenEdge}` : 'is-collapsed'}`}
    onMouseEnter={() => handlers.current?.enter()} onMouseLeave={() => handlers.current?.leave()}
    onPointerDownCapture={event=>{if(editing && event.button===0 && !(event.target as Element).closest('button,a,input,select,textarea,[data-metric-id],[data-balance-id],.balance-picker,.floating-drag'))setEditing(false);}}
    onFocusCapture={event => { if (keyboard.current && event.target.matches(':focus-visible')) handlers.current?.focus(true); }}
    onBlurCapture={event => { if (!event.currentTarget.contains(event.relatedTarget)) handlers.current?.focus(false); }}
    onKeyDown={event => { if (event.key === 'Escape') collapsePanel(); }}>
    {hiddenEdge ? <button type="button" className={`floating-ball floating-edge-handle ${attention ? 'has-attention' : ''}`}
      aria-label="TokenDaily 贴边隐藏，悬停或按回车展开" title="TokenDaily · 悬停展开，展开后可拖离边缘"
      {...ballPointer} onClick={event => { if (event.detail === 0) handlers.current?.activate(); }}>
      <img className="brand-mark" src={brandMark} alt="" />
      {attention && <span className="ball-status" aria-hidden="true" />}
    </button>
    : !expanded ? <button type="button" className={`floating-ball ${attention ? 'is-stale' : ''}`}
      aria-label="TokenDaily 悬浮球，点击查看用量与额度" aria-expanded={false}
      title={resizeError ?? (attention ? 'TokenDaily · 数据或额度需留意，点击查看' : 'TokenDaily · 点击查看，按住拖动')} {...ballPointer}
      onClick={event => { if (event.detail === 0) handlers.current?.activate(); }}>
      <img className="brand-mark" src={brandMark} alt="" />
      {attention && <span className="ball-status" aria-hidden="true" />}
    </button> : <main className={`floating-shell mode-usage ${stale ? 'is-stale' : ''} ${hasRisk ? 'has-risk' : ''}`} aria-label="TokenDaily 悬浮窗">
    <header className="floating-header">
      <div className="floating-drag" onPointerDown={event => void drag(event)} title="按住拖动悬浮窗">
        <img className="brand-mark" src={brandMark} alt="" />
        <span>TokenDaily</span>
      </div>
      <UpdateButton demo={demo}/>
      <button type="button" className="floating-icon" aria-label="打开设置" title="打开设置" onClick={onOpen}>⚙</button>
      <button type="button" className="floating-icon floating-collapse" aria-label="收起悬浮面板" title="收起为悬浮球（Esc）" onClick={collapsePanel}>
        <svg viewBox="0 0 24 24" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>
      </button>
      <button type="button" className="floating-icon" aria-label="关闭悬浮窗" title="关闭悬浮窗，可在设置中重新开启" disabled={busy} onClick={onClose}>×</button>
    </header>
    {demo && <div className="floating-demo">演示 · 非真实数据</div>}
    <FloatingMetrics metrics={metrics} preferences={preferences} editing={editing} onEditing={setEditing} onInteraction={active=>handlers.current?.drag(active)} onOpen={id=>onDetail(id==='usage'?'history':id==='rank'?'rank':'cost')} persist={!demo}/>
    <FloatingBalances sources={sources} now={now} threshold={threshold} onOpen={id=>onDetail(id==='calendar'?'history':'provider',id)} onConfigure={onConfigure} preferences={preferences}
      editing={editing} onEditing={setEditing} onInteraction={active => handlers.current?.drag(active)}/>
    <footer className="floating-footer">
      <button type="button" className="floating-identity" onClick={onIdentity} title={identity.title} aria-label={`${identity.label}，打开身份与参榜`}>
        {identity.avatar?<span aria-hidden="true">{identity.avatar}</span>:<svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="8" r="3.5"/><path d="M5 21v-3a7 7 0 0 1 14 0v3"/></svg>}
      </button>
      <button type="button" className="floating-refresh" disabled={panelRefreshing || !panelCanRefresh} onClick={refreshPanel}
        title={panelCanRefresh ? '刷新额度、Token 与可见日历' : '请先在设置中连接监测来源或添加日历'} aria-label={panelRefreshing ? '正在刷新额度与 Token' : '刷新额度与 Token'}>
        <span aria-hidden="true">↻</span> {panelRefreshing ? '刷新中…' : '刷新'}
      </button>
    </footer>
    {(error || resizeError) && <p className="floating-error" role="alert" title={[error,resizeError].filter(Boolean).join("；")}>{error || resizeError}</p>}
  </main>}
  </div>;
}

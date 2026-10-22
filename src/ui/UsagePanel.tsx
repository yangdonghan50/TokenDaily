import { useState } from 'react';
import { formatTokenCount, usageCalendar, usageDatePresentation, usageDaysEndingOn, type UsageState } from '../domain/usage.ts';
import { formatExactTokenCount } from '../domain/tokenFormat.ts';
import { TokenCount } from './TokenCount.tsx';
import { UsageHeatmap } from './UsageHeatmap.tsx';

type UsageView = 'trend' | 'calendar';
const dateLabel = (date: string) => `${Number(date.slice(5, 7))} 月 ${Number(date.slice(8, 10))} 日`;

export function UsagePanel({ usage, now, demo }: { usage: UsageState; now: number; demo: boolean }) {
  const [view, setView] = useState<UsageView>('trend');
  const [selectedDate, setSelectedDate] = useState<string | null>(null);
  const days = usage.snapshot?.days ?? [];
  const latest = days.at(-1);
  const selected = view === 'calendar' ? latest : days.find((day) => day.date === selectedDate) ?? latest;
  const stale = usage.status === 'stale' || Boolean(usage.lastSuccessAtMs && now - usage.lastSuccessAtMs > 600_000);
  const cells = latest ? view === 'trend' ? usageDaysEndingOn(days, latest.date, 30) : usageCalendar(days, latest.date) : [];
  const max = Math.max(0, ...cells.map((cell) => cell.tokens ?? 0));
  const recorded = cells.filter((cell) => cell.tokens !== null).length;
  const dateView = usageDatePresentation(days, now);

  return <section className="usage-section" aria-labelledby="usage-title">
    <div className="usage-title-row"><div><p className="section-eyebrow">LOCAL / 本机日志合计</p><h2 id="usage-title">总 Token 历史</h2></div>
      {latest && <span className={`usage-health ${stale ? 'is-stale' : ''}`}>{stale ? '上次记录' : '来源数据'}</span>}</div>
    {latest && selected ? <>
      <div className="usage-total"><strong><TokenCount tokens={selected.tokens} /></strong><span>Tokens</span></div>
      <p className="usage-date">{dateLabel(selected.date)} · 本机日志记录{selected.date === latest.date ? ' · 最近统计的一天' : ''}</p>
      {usage.message && <p className="window-warning">{usage.message}</p>}
      {dateView.notice && <p className="usage-date-notice">{dateView.notice}。读取成功不代表统计日期已经推进。</p>}
      <div className="usage-tabs" role="tablist" aria-label="用量图表">
        <button type="button" role="tab" aria-selected={view === 'trend'} className={view === 'trend' ? 'active' : ''} onClick={() => setView('trend')}>趋势</button>
        <button type="button" role="tab" aria-selected={view === 'calendar'} className={view === 'calendar' ? 'active' : ''} onClick={() => setView('calendar')}>日历</button>
      </div>
      {view === 'trend' ? <>
        <div className="trend-chart" role="group" aria-label="最近 30 天本机总 Token 记录">
          {cells.map((cell) => <button key={cell.date} type="button" className={`trend-day ${cell.tokens === null ? 'is-missing' : ''} ${selected.date === cell.date ? 'is-selected' : ''}`}
            disabled={cell.tokens === null} onClick={() => setSelectedDate(cell.date)}
            title={`${dateLabel(cell.date)}：${cell.tokens === null ? '本机日志未记录' : `${formatTokenCount(cell.tokens)} Tokens（${formatExactTokenCount(cell.tokens)}）`}`}
            aria-label={`${dateLabel(cell.date)}：${cell.tokens === null ? '本机日志未记录' : `${formatExactTokenCount(cell.tokens)} Tokens`}`}>
            <span style={{ height: cell.tokens === null ? '4px' : cell.tokens === 0 ? '3px' : `${Math.max(9, cell.tokens / Math.max(1, max) * 100)}%` }} /></button>)}
        </div>
        <div className="chart-axis"><span>{dateLabel(cells[0].date)}</span><span>{dateLabel(latest.date)}</span></div>
      </> : <>
        <UsageHeatmap cells={cells}/>
        <div className="calendar-legend"><span>少</span>{['missing', '0', '1', '2', '3', '4'].map((level) => <i key={level} className={`level-${level}`} />)}<span>多</span></div>
      </>}
      <p className="usage-caption">最近{view === 'trend' ? '30 天' : '13 周'} · {recorded} 天有记录。空白表示本机日志未记录，只有完成扫描后确认的当天 0 才算零用量。</p>
      {stale && <p className="state-note">读取状态待确认，显示上次成功读取的记录。</p>}
    </> : <div className="usage-empty"><strong>暂无可靠的总 Token 记录</strong><p>{usage.message ?? '当前没有可展示的本机日志记录。'}</p></div>}
    <p className="usage-scope">{demo ? '演示用量，非真实账户数据。' : '按北京时间汇总 Codex、Claude Code、WorkBuddy 本机日志和已连接的 TRAE 本地数据库。TRAE 恢复/同步会话可能来自其他设备；不混入账号日统计或 Claude 网页 / 桌面聊天。部分来源无法读取时明确标注，空白日期不补成 0。'}</p>
  </section>;
}

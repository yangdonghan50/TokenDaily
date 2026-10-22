import { useEffect, useRef, useState, type PointerEvent, type ReactNode } from 'react';
import { balanceStrip, FLOATING_ORDER_KEY, normalizeOrder, orderedSources, moveSource, canStartBalanceDrag, type BalanceSource } from '../domain/floatingBalances.ts';
import { createComponentPress, mergeVisibleOrder, visibleComponents } from '../domain/floatingComponents.ts';
import type { useFloatingComponents } from './useFloatingComponents.ts';
import './floatingBalances.css';
import { METRIC_OPTIONS } from '../domain/floatingMetrics.ts';
import { ProviderLogo } from './ProviderLogo.tsx';
import type { Provider } from '../domain/provider.ts';

function readOrder(): string[] { try { return normalizeOrder(JSON.parse(localStorage.getItem(FLOATING_ORDER_KEY) ?? '[]')); } catch { return []; } }
export type FloatingComponent = BalanceSource | {id:'calendar';label:string;status:string;connected?:boolean;content:ReactNode};
type Props = { sources: FloatingComponent[]; now: number; threshold: number; onOpen: (id: string) => void; onConfigure: () => void;
  onInteraction: (active: boolean) => void; editing: boolean; onEditing: (active: boolean) => void; preferences: ReturnType<typeof useFloatingComponents> };

export function FloatingBalances({ sources, now, threshold, onOpen, onConfigure, onInteraction, editing, onEditing, preferences }: Props) {
  const [saved, setSaved] = useState(readOrder);
  const [preview, setPreview] = useState<string[] | null>(null);
  const [active, setActive] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState('');
  const [adding, setAdding] = useState(false);
  const board = useRef<HTMLDivElement>(null);
  const done = useRef<HTMLButtonElement>(null);
  const addButton = useRef<HTMLButtonElement>(null);
  const drag = useRef<{ id: string; pointer: number; moved: boolean } | null>(null);
  const held = useRef(false);
  const visible = visibleComponents(orderedSources(sources, preview ?? saved), preferences.hidden);
  const ids = visible.map(source => source.id);
  const latest = useRef(ids); latest.current = ids;
  const callbacks = useRef({ onInteraction, onEditing, editing }); callbacks.current = { onInteraction, onEditing, editing };
  const press = useRef(createComponentPress(() => {
    held.current = true;
    setActive(null); callbacks.current.onInteraction(false); callbacks.current.onEditing(true);
  }, { schedule: (run, delay) => window.setTimeout(run, delay), cancel: id => window.clearTimeout(id) }));

  function cancel() {
    press.current.cancel();
    const pointer = drag.current?.pointer; drag.current = null;
    if (pointer !== undefined && board.current?.hasPointerCapture(pointer)) board.current.releasePointerCapture(pointer);
    setPreview(null); setActive(null); callbacks.current.onInteraction(false);
  }
  function exitEditing() { cancel(); setAdding(false); callbacks.current.onEditing(false); }
  useEffect(() => {
    const storage = (event: StorageEvent) => { if (event.key === FLOATING_ORDER_KEY || event.key === null) { cancel(); setSaved(readOrder()); } };
    const blur = () => { cancel(); setAdding(false); callbacks.current.onEditing(false); };
    const keydown = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return;
      if (drag.current) { event.preventDefault(); event.stopPropagation(); cancel(); }
      else if (callbacks.current.editing) { event.preventDefault(); event.stopPropagation(); exitEditing(); }
    };
    window.addEventListener('storage', storage); window.addEventListener('blur', blur); window.addEventListener('keydown', keydown, true);
    return () => { press.current.cancel(); drag.current = null; callbacks.current.onInteraction(false);
      window.removeEventListener('storage', storage); window.removeEventListener('blur', blur); window.removeEventListener('keydown', keydown, true); };
  }, []);
  const sourceKey = sources.map(source => source.id).join('|');
  const hiddenKey = preferences.hidden.join('|');
  useEffect(() => { cancel(); }, [sourceKey, hiddenKey]);
  useEffect(() => { if (!editing) { setAdding(false); cancel(); } }, [editing]);

  function save(next: string[]) {
    const full = mergeVisibleOrder(orderedSources(sources, saved).map(source => source.id), next);
    try { localStorage.setItem(FLOATING_ORDER_KEY, JSON.stringify(full)); setSaved(full); setError(null); }
    catch { setError('排序未保存，请重试'); }
  }
  function start(event: PointerEvent<HTMLElement>, id: string) {
    if (!canStartBalanceDrag({button:event.button,isPrimary:event.isPrimary,sourceCount:ids.length,dragging:drag.current!==null,
      interactive:Boolean((event.target as Element).closest('button,a,input,select,textarea,[contenteditable="true"]'))})) return;
    event.preventDefault(); event.stopPropagation();
    held.current = false;
    drag.current = { id, pointer: event.pointerId, moved: false };
    press.current.down(event.pointerId, event.clientX, event.clientY);
    callbacks.current.onInteraction(true); board.current?.setPointerCapture(event.pointerId);
  }
  function move(event: PointerEvent<HTMLElement>) {
    const current = drag.current;
    if (!current || !board.current || !press.current.move(event.pointerId, event.clientX, event.clientY)) return;
    if (ids.length < 2) return;
    current.moved = true; setActive(current.id);
    const rows = Array.from(board.current.querySelectorAll<HTMLElement>('[data-balance-id]'));
    const target = rows.find(row => event.clientY < row.getBoundingClientRect().bottom + 3) ?? rows.at(-1);
    if (target) { const next = moveSource(latest.current, current.id, target.dataset.balanceId!); latest.current = next; setPreview(mergeVisibleOrder(orderedSources(sources, saved).map(source => source.id), next)); }
  }
  function finish(event: PointerEvent<HTMLElement>, commit: boolean) {
    const current = drag.current; if (!current || current.pointer !== event.pointerId) return;
    const moved = press.current.up(event.pointerId);
    if (commit && moved && current.moved) save(latest.current);
    const open = commit && !moved && !current.moved && !held.current && !editing;
    const id = current.id;
    cancel();
    if (open) onOpen(id);
  }
  function remove(source: FloatingComponent) {
    if (preferences.setVisible(source.id, false)) { setNotice(`已移除 ${source.label} 卡片；采集与总 Token 不变，可重新添加。`); done.current?.focus(); }
  }
  return <section className={`floating-balances ${editing ? 'is-editing' : ''}`} aria-label="AI 监控组件">
    {adding && editing ? <div className="balance-picker" aria-label="添加监控组件">
      <div className="balance-picker-heading"><strong>添加组件</strong><button type="button" onClick={() => { setAdding(false); addButton.current?.focus(); }}>返回</button></div>
      <div className="metric-picker" aria-label="顶部指标组件">
        {METRIC_OPTIONS.map(metric=><button key={metric.id} type="button" disabled={!preferences.hidden.includes(metric.id)}
          aria-label={!preferences.hidden.includes(metric.id)?metric.label+' 已添加':'添加 '+metric.label}
          onClick={()=>{if(preferences.setVisible(metric.id,true))setNotice('已添加 '+metric.label);}}>{preferences.hidden.includes(metric.id)?'＋ ': '✓ '}{metric.label}</button>)}
      </div>
      {sources.map(source => {
        const added = !preferences.hidden.includes(source.id);
        return <div className="balance-picker-row" key={source.id}><div><strong>{source.label}</strong><small>{source.connected === false ? '需要连接 · 添加不自动授权' : source.status || '已接入来源'}</small></div>
          <button type="button" disabled={added} aria-label={added ? `${source.label} 已添加` : `添加 ${source.label}`} onClick={() => {
            if (preferences.setVisible(source.id, true)) setNotice(source.connected === false ? `已添加 ${source.label}；请到设置连接。` : `已添加 ${source.label}`);
          }}>{added ? '已添加' : '＋ 添加'}</button></div>;
      })}
      {sources.some(source => source.connected === false) && <button type="button" className="balance-connect" onClick={onConfigure}>前往设置连接工具 ↗</button>}
    </div> : <div ref={board} className="balance-strips" onPointerMove={move} onPointerUp={event => finish(event, true)} onPointerCancel={event => finish(event, false)} onLostPointerCapture={event => finish(event, false)}>
      {visible.map(source => {
        const row = 'content' in source ? null : balanceStrip(source, now, threshold);
        return <article key={source.id} data-balance-id={source.id} tabIndex={0} aria-label={`${source.label} 监控组件`} aria-describedby="balance-help"
          onPointerDown={event => start(event, source.id)}
          onKeyDown={event => {
            if (event.target !== event.currentTarget) return;
            if (event.key === 'Enter') { event.preventDefault(); if (!editing) onOpen(source.id); }
            if (event.key === ' ') { event.preventDefault(); onEditing(true); }
            if (['ArrowUp', 'ArrowDown'].includes(event.key)) { event.preventDefault(); event.stopPropagation(); const at = ids.indexOf(source.id) + (event.key === 'ArrowUp' ? -1 : 1); if (at >= 0 && at < ids.length) save(moveSource(ids, source.id, ids[at])); }
            if (editing && event.key === 'Delete') { event.preventDefault(); remove(source); }
          }}
          className={`balance-strip is-sortable ${'content' in source ? 'is-calendar' : ''} ${row && row.featured.length > 1 ? 'has-quota-pair' : ''} ${active === source.id ? 'is-dragging' : ''} ${row?.low ? 'is-low' : ''} ${row?.stale ? 'is-stale' : ''}`} title={row?.title ?? undefined}>
          {editing && <button type="button" className="balance-remove" aria-label={`移除 ${source.label} 组件`} title="仅从悬浮窗移除，不停止采集" onClick={() => remove(source)}>×</button>}
          {'content' in source ? source.content : row && <><ProviderLogo provider={source.id as Provider}/><div className="balance-strip-body">
            {row.featured.length > 1 ? <><div className="balance-pair-heading"><strong>{source.label}</strong>{row.stale && <span>上次记录</span>}
              {row.additional > 0 && <button type="button" className={`balance-more ${row.hiddenLow ? 'has-hidden-low' : ''}`} onClick={()=>onOpen(source.id)} title={`${row.title}\n查看全部额度窗口`} aria-label={`另有 ${row.additional} 个额度窗口${row.hiddenLow?'，其中有低额度':''}，打开详情`}>另 {row.additional} 项{row.hiddenLow?' !':''}</button>}
              <small title={source.message ?? row.update}>{row.update}</small></div>
              <div className="balance-quota-pair">{row.featured.map((item,index)=><div className={`balance-quota-mini ${item.low ? 'is-low' : ''}`} key={`${item.label}-${index}`} title={item.title}>
                <div><span>{item.label}</span><strong>{item.amount}</strong></div>
                <div className={`balance-track ${item.percent === null ? 'is-neutral' : ''}`} role={item.percent === null ? undefined : 'meter'} aria-label={`${source.label} ${item.label}剩余`} aria-valuemin={item.percent === null ? undefined : 0} aria-valuemax={item.percent === null ? undefined : 100} aria-valuenow={item.percent ?? undefined}>{item.percent !== null && <i style={{width:`${item.percent}%`}}/>}</div>
                <small>{item.detail}</small>
              </div>)}</div></> : <>
            <div className="balance-strip-heading"><span className="balance-strip-name">{source.label}<small>{row.windowLabel}</small></span>
              {row.additional > 0 && <button type="button" className="balance-more" onClick={()=>onOpen(source.id)} title={`${row.title}\n查看全部额度窗口`}>+{row.additional}{row.partial ? ' ?' : ''}</button>}
              <strong className="balance-strip-amount">{row.amount}<small>{row.unit}</small></strong></div>
            {row.percent !== null ? <div className="balance-track" role="meter" aria-label={`${source.label} ${row.windowLabel}剩余`} aria-valuenow={row.percent} aria-valuemin={0} aria-valuemax={100}><div style={{ width: `${row.percent}%` }} /></div> : <div className="balance-track is-neutral" aria-hidden="true" />}
            <div className="balance-strip-meta"><span title={row.detail}>{row.low ? `${row.stale ? '上次' : '当前'}偏低 · ` : ''}{row.detail}</span><small title={source.message ?? row.update}>{row.update}</small></div>
            </>}
          </div></>}
        </article>;
      })}
      {!visible.length && <div className="balance-empty"><span>还没有显示的组件</span><small>采集与总 Token 不受影响</small>{!editing && <button type="button" onClick={() => { onEditing(true); setAdding(true); }}>＋ 添加组件</button>}</div>}
    </div>}
    {editing && <div className="balance-edit-actions"><button ref={addButton} type="button" onClick={() => setAdding(!adding)} aria-expanded={adding}>＋ 添加组件</button><button ref={done} type="button" className="balance-done" onClick={exitEditing}>完成</button></div>}
    <div className={`balance-order-note ${error || preferences.error ? 'has-error' : ''}`} role="status">{error || preferences.error || (editing ? notice || '仅调整展示，不影响采集与总 Token' : active ? '松开保存顺序' : '')}</div>
    <span id="balance-help" className="balance-help">点击查看详情，拖动排序，长按编辑；键盘方向键排序，回车打开，空格编辑。</span>
    {!editing && visible.length > 0 && <button type="button" className="balance-manage" onClick={() => onEditing(true)}>长按编辑 · 管理组件</button>}
  </section>;
}

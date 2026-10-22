import { useEffect, useRef, useState, type PointerEvent, type ReactNode } from 'react';
import { METRIC_ORDER_KEY, normalizeMetricOrder } from '../domain/floatingMetrics.ts';
import { canStartBalanceDrag, moveSource } from '../domain/floatingBalances.ts';
import { createComponentPress, mergeVisibleOrder } from '../domain/floatingComponents.ts';
import type { useFloatingComponents } from './useFloatingComponents.ts';

export type Metric = { id: string; label: string; title: string; content: ReactNode; stale?: boolean };
type Props = { metrics: Metric[]; preferences: ReturnType<typeof useFloatingComponents>; editing: boolean;
  onEditing: (active: boolean) => void; onInteraction: (active: boolean) => void; onOpen: (id: string) => void; persist: boolean };
function readOrder() {
  try { return normalizeMetricOrder(JSON.parse(localStorage.getItem(METRIC_ORDER_KEY) ?? 'null')); }
  catch { return normalizeMetricOrder(null); }
}
export function FloatingMetrics({metrics,preferences,editing,onEditing,onInteraction,onOpen,persist}:Props) {
  const [saved,setSaved]=useState(()=>persist?readOrder():normalizeMetricOrder(null));
  const [preview,setPreview]=useState<string[]|null>(null);
  const [active,setActive]=useState<string|null>(null);
  const [error,setError]=useState<string|null>(null);
  const board=useRef<HTMLDivElement>(null);
  const drag=useRef<{id:string;pointer:number;moved:boolean}|null>(null);
  const held=useRef(false);
  const callbacks=useRef({onEditing,onInteraction});callbacks.current={onEditing,onInteraction};
  const ids=(preview??saved).filter(id=>!preferences.hidden.includes(id));
  const latest=useRef(ids);latest.current=ids;
  const press=useRef(createComponentPress(()=>{
    held.current=true;
    setActive(null);callbacks.current.onInteraction(false);callbacks.current.onEditing(true);
  },{schedule:(run,delay)=>window.setTimeout(run,delay),cancel:id=>window.clearTimeout(id)}));
  function cancel() {
    press.current.cancel();const pointer=drag.current?.pointer;drag.current=null;
    if(pointer!==undefined&&board.current?.hasPointerCapture(pointer))board.current.releasePointerCapture(pointer);
    setPreview(null);setActive(null);callbacks.current.onInteraction(false);
  }
  useEffect(()=>{
    const storage=(event:StorageEvent)=>{if(persist&&(event.key===METRIC_ORDER_KEY||event.key===null)){cancel();setSaved(readOrder());}};
    const escape=(event:KeyboardEvent)=>{if(event.key==='Escape'&&drag.current){event.preventDefault();event.stopImmediatePropagation();cancel();}};
    window.addEventListener('storage',storage);window.addEventListener('blur',cancel);window.addEventListener('keydown',escape,true);
    return ()=>{press.current.cancel();drag.current=null;callbacks.current.onInteraction(false);window.removeEventListener('storage',storage);window.removeEventListener('blur',cancel);window.removeEventListener('keydown',escape,true);};
  },[persist]);
  useEffect(()=>{cancel();},[preferences.hidden.join('|')]);
  useEffect(()=>{if(!editing)cancel();},[editing]);
  function save(next:string[]) {
    const full=mergeVisibleOrder(saved,next);
    try{if(persist)localStorage.setItem(METRIC_ORDER_KEY,JSON.stringify(full));setSaved(full);setError(null);}
    catch{setError('顶部排序未保存，请重试');}
  }
  function start(event:PointerEvent<HTMLElement>,id:string) {
    if(!canStartBalanceDrag({button:event.button,isPrimary:event.isPrimary,sourceCount:ids.length,dragging:!!drag.current,
      interactive:!!(event.target as Element).closest('button,a,input')}))return;
    event.preventDefault();event.stopPropagation();held.current=false;drag.current={id,pointer:event.pointerId,moved:false};
    press.current.down(event.pointerId,event.clientX,event.clientY);onInteraction(true);board.current?.setPointerCapture(event.pointerId);
  }
  function move(event:PointerEvent<HTMLElement>) {
    const current=drag.current;if(!current||current.pointer!==event.pointerId||!press.current.move(event.pointerId,event.clientX,event.clientY)||ids.length<2)return;
    current.moved=true;setActive(current.id);
    const tiles=Array.from(board.current?.querySelectorAll<HTMLElement>('[data-metric-id]')??[]);
    const target=tiles.find(tile=>event.clientX<tile.getBoundingClientRect().right)??tiles.at(-1);
    if(target){const next=moveSource(latest.current,current.id,target.dataset.metricId!);latest.current=next;setPreview(mergeVisibleOrder(saved,next));}
  }
  function finish(event:PointerEvent<HTMLElement>,commit:boolean) {
    const current=drag.current;if(!current||current.pointer!==event.pointerId)return;
    const moved=press.current.up(event.pointerId);
    if(moved&&commit&&current.moved)save(latest.current);
    const open=commit&&!moved&&!current.moved&&!held.current&&!editing;
    const id=current.id;cancel();if(open)onOpen(id);
  }
  function remove(metric:Metric) {
    if(preferences.setVisible(metric.id,false))document.querySelector<HTMLButtonElement>('.balance-done')?.focus();
  }
  if(!ids.length&&!error)return null;
  return <section className={`floating-usage metric-components ${editing?'is-editing':''}`} aria-label="顶部指标组件">
    <div ref={board} className="floating-summary-grid" style={{gridTemplateColumns:`repeat(${ids.length},minmax(0,1fr))`}}
      onPointerMove={move} onPointerUp={e=>finish(e,true)} onPointerCancel={e=>finish(e,false)} onLostPointerCapture={e=>finish(e,false)}>
      {ids.map(id=>{const metric=metrics.find(item=>item.id===id)!;return <article key={id} data-metric-id={id} tabIndex={0}
        className={`floating-metric metric-${id} ${metric.stale?'is-old':''} ${active===id?'is-dragging':''}`}
        title={metric.title} aria-label={`${metric.label}：${metric.title}`} aria-describedby="metric-help"
        onPointerDown={e=>start(e,id)} onKeyDown={e=>{
          if(e.target!==e.currentTarget)return;
          if(e.key==='Enter'){e.preventDefault();if(!editing)onOpen(id);}
          if(e.key===' '){e.preventDefault();onEditing(true);}
          if(e.key==='ArrowLeft'||e.key==='ArrowRight'){e.preventDefault();e.stopPropagation();const at=ids.indexOf(id)+(e.key==='ArrowLeft'?-1:1);if(at>=0&&at<ids.length)save(moveSource(ids,id,ids[at]));}
          if(editing&&e.key==='Delete'){e.preventDefault();remove(metric);}
        }}>
        <span className="floating-metric-label">{metric.label}</span>{metric.content}
        {editing&&<button type="button" className="balance-remove metric-remove" aria-label={`移除 ${metric.label} 组件`} title="仅隐藏展示，可从添加组件恢复" onClick={()=>remove(metric)}>×</button>}
      </article>;})}
    </div>
    {error&&<div className="balance-order-note has-error" role="alert">{error}</div>}
    <span id="metric-help" className="balance-help">点击查看详情，左右拖动排序，长按编辑；键盘回车打开，空格编辑。</span>
  </section>;
}

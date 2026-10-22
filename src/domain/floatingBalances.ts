import { formatRemaining, resetPresentation, type QuotaWindow } from './quota.ts';
import { creditPresentation, type CreditState } from './credits.ts';
export const FLOATING_ORDER_KEY='tokendaily.floating-order.v1';
export const MAX_BALANCE_SOURCES=4;
export function canStartBalanceDrag(press:{button:number;isPrimary:boolean;sourceCount:number;interactive:boolean;dragging:boolean}):boolean{
  return press.button===0&&press.isPrimary&&press.sourceCount>0&&!press.interactive&&!press.dragging;
}
export type BalanceSource={id:string;label:string;windows:QuotaWindow[];status:string;update:string;stale?:boolean;message?:string|null;credits?:CreditState;connected?:boolean};
type FeaturedQuota={label:string;amount:string;percent:number|null;low:boolean;detail:string;title:string};
function compactWindowLabel(window:QuotaWindow):string{
  const minutes=window.durationMinutes;
  if(minutes!==null){
    if(minutes%1440===0)return `${minutes/1440} 天`;
    if(minutes%60===0)return `${minutes/60} 小时`;
    return `${minutes} 分钟`;
  }
  return window.label;
}
function featuredWindows(sourceId:string,windows:QuotaWindow[]):QuotaWindow[]{
  const pools=new Map<string,QuotaWindow[]>();
  for(const window of windows){
    // Claude's five-hour and general weekly limits arrive as separate keys,
    // while model-specific weekly limits remain separate pools.
    const pool=sourceId==='claude'&&['claude:five_hour','claude:seven_day'].includes(window.poolId)?'claude:general':window.poolId;
    pools.set(pool,[...(pools.get(pool)??[]),window]);
  }
  const groups=[...pools.values()].sort((a,b)=>{
    const distinct=(items:QuotaWindow[])=>new Set(items.map(item=>item.durationMinutes??item.label)).size;
    return distinct(b)-distinct(a);
  });
  const pool=groups[0]??[];
  const unique=new Map<number|string,QuotaWindow>();
  for(const window of pool){
    const key=window.durationMinutes??window.label;
    const previous=unique.get(key);
    if(!previous||previous.remainingPercent===null&&window.remainingPercent!==null||previous.remainingPercent!==null&&window.remainingPercent!==null&&window.remainingPercent<previous.remainingPercent)unique.set(key,window);
  }
  return [...unique.values()].sort((a,b)=>(a.durationMinutes??Number.MAX_SAFE_INTEGER)-(b.durationMinutes??Number.MAX_SAFE_INTEGER)).slice(0,2);
}
export function normalizeOrder(value:unknown):string[]{return Array.isArray(value)?[...new Set(value.filter((id):id is string=>typeof id==='string'&&id.length>0&&id.length<=64))].filter((id,i,ids)=>id==='calendar'||ids.slice(0,i+1).filter(v=>v!=='calendar').length<=MAX_BALANCE_SOURCES):[];}
export function orderedSources<T extends {id:string}>(sources:T[],order:string[]):T[]{
  const unique=[...new Map(sources.map(s=>[s.id,s])).values()];
  return [...order.flatMap(id=>unique.find(s=>s.id===id)??[]),...unique.filter(s=>!order.includes(s.id))].filter((s,i,a)=>a.findIndex(v=>v.id===s.id)===i).filter((s,i,a)=>s.id==='calendar'||a.slice(0,i+1).filter(v=>v.id!=='calendar').length<=MAX_BALANCE_SOURCES);
}
export function moveSource(order:string[],id:string,target:string):string[]{
  if(id===target||!order.includes(id)||!order.includes(target))return [...order];
  const result=order.filter(v=>v!==id);result.splice(order.indexOf(target),0,id);return result;
}
export function balanceStrip(source:BalanceSource,now:number,threshold:number){
  if(source.credits){const c=creditPresentation(source.credits,now);return {amount:c.amount,unit:'积分',percent:null,windowLabel:'',additional:0,partial:false,stale:c.stale,low:false,hiddenLow:false,featured:[] as FeaturedQuota[],
    detail:source.credits.balance?c.stale?'上次积分 · 等待更新':c.expiry:c.label,
    title:source.credits.message,update:c.update};}
  const known=source.windows.filter(w=>w.remainingPercent!==null);
  const selected=[...known].sort((a,b)=>Number(Boolean(b.reachedType))-Number(Boolean(a.reachedType))||(a.remainingPercent!-b.remainingPercent!))[0]??source.windows[0];
  const partial=source.windows.some(w=>w.remainingPercent===null);
  const low=source.windows.some(w=>Boolean(w.reachedType)||w.remainingPercent!==null&&w.remainingPercent<=threshold);
  const shown=featuredWindows(source.id,source.windows);
  const hiddenLow=source.windows.some(window=>!shown.includes(window)&&(Boolean(window.reachedType)||window.remainingPercent!==null&&window.remainingPercent<=threshold));
  const featured:FeaturedQuota[]=shown.map(window=>({
    label:compactWindowLabel(window),amount:formatRemaining(window.remainingPercent),percent:window.remainingPercent,
    low:Boolean(window.reachedType)||window.remainingPercent!==null&&window.remainingPercent<=threshold,
    detail:resetPresentation(window.resetsAtMs,now).relative,
    title:`${window.label}：${formatRemaining(window.remainingPercent)}；${resetPresentation(window.resetsAtMs,now).relative}`,
  }));
  const detail=selected?`${source.stale?'上次额度 · ':''}${resetPresentation(selected.resetsAtMs,now).relative}`:source.status;
  return {amount:selected?.remainingPercent==null?'—':formatRemaining(selected.remainingPercent),unit:'',percent:selected?.remainingPercent??null,windowLabel:selected?.label??'',additional:Math.max(0,source.windows.length-featured.length),partial,stale:!!source.stale,low,hiddenLow,featured,
    detail,update:source.update,title:source.windows.length?source.windows.map(w=>`${w.label}：${formatRemaining(w.remainingPercent)}；${resetPresentation(w.resetsAtMs,now).relative}${w.reachedType?'；来源报告受限':''}`).join('\n'):source.message??source.status};
}

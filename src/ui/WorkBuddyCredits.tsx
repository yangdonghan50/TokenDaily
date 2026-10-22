import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { canRefreshCredits, creditPresentation, formatCreditAmount, type CreditState } from '../domain/credits.ts';
const initial: CreditState = {status:'auth-required',supported:false,authorized:false,refreshing:false,identityLabel:null,balance:null,message:'正在读取积分连接状态'};
export function useWorkBuddyCredits(enabled: boolean, provider: 'workbuddy'|'trae' = 'workbuddy') {
  const [state,setState] = useState(initial);
  const [error,setError] = useState<string|null>(null);
  const [busy,setBusy] = useState(false);
  useEffect(()=>{
    if(!enabled)return;
    let stopped=false,received=false;
    const subscription=listen<CreditState>(`${provider}-credits-updated`,event=>{received=true;if(!stopped)setState(event.payload);});
    void subscription.then(()=>invoke<CreditState>(`get_${provider}_credits`)).then(v=>{if(!stopped&&!received)setState(v);}).catch(()=>{if(!stopped)setError('积分服务暂不可用');});
    return ()=>{stopped=true;void subscription.then(stop=>stop());};
  },[enabled,provider]);
  async function command(name:string){if(!enabled||busy)return;setBusy(true);setError(null);try{await invoke(name);}catch(e){setError(typeof e==='string'?e:'积分操作失败，请重试');}finally{setBusy(false);}}
  return {state,error,busy,authorize:()=>command(`authorize_${provider}_credits`),cancel:()=>command(`cancel_${provider}_authorization`),disconnect:()=>command(`disconnect_${provider}_credits`),refresh:()=>command(`refresh_${provider}_credits`)};
}
export function CreditCard({state,now,compact=false,details=false,label='WorkBuddy'}:{state:CreditState;now:number;compact?:boolean;details?:boolean;label?:string}){
  const view=creditPresentation(state,now);
  return <section className={`credit-card ${compact?'is-compact':''} ${view.stale?'is-old':''}`} aria-label={`${label} 积分余额`}>
    <div className="credit-heading"><span>{label} · {view.label}</span>{!compact&&<small>{state.identityLabel}</small>}</div>
    <div className="credit-amount"><strong>{view.amount}</strong><span> 积分</span></div>
    <p>{state.balance?view.expiry:state.message}</p>
    {view.stale&&<p className="window-warning">上次记录，等待官方更新</p>}
    {!compact&&<small>{view.update} · 官方账户数据</small>}
    {details&&state.balance&&<><p className="muted">积分有效期不代表额度重置。本机 Token 不区分账户，与此授权账户分别统计。</p>{state.balance.packages.map((p,index)=><div className="detail-window" key={index}><span>{p.name}</span><strong>{formatCreditAmount(p.remaining)} 积分</strong><small>{p.expiresAtMs?`${new Date(p.expiresAtMs).toLocaleString('zh-CN',{timeZone:'Asia/Shanghai'})} 到期（北京）`:'有效期未知'}</small></div>)}</>}
    {state.balance&&(state.status!=='available'||state.message.includes('保存失败'))&&<p className="window-warning">{state.message}</p>}
  </section>;
}
export function CreditSettings({credits,connected,now,label='WorkBuddy'}:{credits:ReturnType<typeof useWorkBuddyCredits>;connected:boolean;now:number;label?:string}){
  const {state}=credits;
  return <div className="settings-group"><h2>{label} 积分授权</h2><p className="muted">通过官方网页登录查询积分，建议登录 {label} 正在使用的同一账户。{label === 'WorkBuddy' ? '授权由系统安全存储保存在本机' : '登录仅保存在 TokenDaily 本机'}，不修改客户端登录。每 5 分钟自动刷新，失败后每分钟重试；手动刷新至少间隔 10 秒。</p>
    <CreditCard state={state} now={now} label={label}/>
    {!connected&&<p className="muted">请先连接 {label} 本机监测。</p>}
    {state.status==='authorizing'?<button className="settings-action secondary" disabled={credits.busy} onClick={credits.cancel}>取消本次授权</button>:<button className="settings-action" disabled={!connected||!state.supported||credits.busy||state.refreshing} onClick={credits.authorize}>{state.authorized?'重新登录官方账户':'登录官方账户查询积分'}</button>}
    {connected&&state.supported&&state.status!=='auth-required'&&state.status!=='authorizing'&&<button className="settings-action secondary" disabled={credits.busy||!canRefreshCredits(state)} onClick={credits.refresh}>刷新积分</button>}
    {state.identityLabel&&<button className="settings-action secondary" disabled={credits.busy} onClick={credits.disconnect}>清除积分授权</button>}
    {state.status==='authorizing'&&<p className="muted">{state.message}</p>}
    {credits.error&&<p className="window-warning" role="alert">{credits.error}</p>}
    <p className="muted">授权过期后需重新登录；清除授权不会注销 {label}。积分暂不参与百分比提醒。</p>
  </div>;
}

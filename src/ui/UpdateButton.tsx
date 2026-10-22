import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { updatePresentation, type AppUpdateState } from '../domain/appUpdate.ts';
import './appUpdate.css';

const initial: AppUpdateState = {phase:'idle',currentVersion:'',latestVersion:null,downloaded:0,total:null,message:null};
export function UpdateButton({demo=false}:{demo?:boolean}) {
  const [state,setState]=useState<AppUpdateState>({...initial,phase:demo?'unsupported':'idle'});
  const [pending,setPending]=useState(false);
  useEffect(()=>{
    if(demo)return;
    let disposed=false;
    const read=()=>invoke<AppUpdateState>('get_app_update').then(next=>{if(!disposed)setState(next);});
    const subscription=listen<AppUpdateState>('app-update',event=>{if(!disposed)setState(event.payload);});
    void subscription.then(read).catch(()=>{if(!disposed)setState({...initial,phase:'checkError',message:'暂时无法读取更新状态'});});
    return ()=>{disposed=true;void subscription.then(unlisten=>unlisten()).catch(()=>{});};
  },[demo]);
  const view=updatePresentation(state);
  async function activate() {
    if(pending||view.disabled)return;
    setPending(true);
    try { setState(await invoke<AppUpdateState>(view.action==='install'?'install_app_update':'check_app_update')); }
    catch { setState(previous=>({...previous,phase:view.action==='install'?'installError':'checkError',message:'更新操作未完成，请稍后重试'})); }
    finally { setPending(false); }
  }
  return <button type="button" className={`floating-icon app-update-button ${view.attention?'has-update':''}`}
    title={view.title} aria-label={view.title} disabled={pending||view.disabled} onClick={()=>void activate()}>
    {view.progress!==null?<span className="app-update-progress" aria-hidden="true">{view.progress}%</span>:<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3v12m-4-4 4 4 4-4M5 17v4h14v-4"/></svg>}
    {view.attention&&<span className="app-update-dot" aria-hidden="true"/>}
    <span className="app-update-status" role="status" aria-live="polite">{view.title}</span>
  </button>;
}

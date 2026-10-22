import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { captureHotkey, hotkeyLabel, type FloatingHotkey } from '../domain/floatingHotkey.ts';

type Status = { shortcut: FloatingHotkey | null; supported: boolean; active: boolean; recording: boolean; error: string | null };
const platform = /Mac/.test(navigator.platform) ? 'macos' : 'windows';
const label = (value: FloatingHotkey|null) => hotkeyLabel(value,platform);
export function FloatingHotkeySettings({native,enabled,focusRequest=0}:{native:boolean;enabled:boolean;focusRequest?:number}) {
  const [status,setStatus]=useState<Status|null>(null);
  const [recording,setRecording]=useState(false);
  const [draft,setDraft]=useState<FloatingHotkey|null>(null);
  const [busy,setBusy]=useState(false);
  const [error,setError]=useState<string|null>(null);
  const capturing=useRef(false);
  const leased=useRef(false);
  const mounted=useRef(true);
  const recordButton=useRef<HTMLButtonElement>(null);
  useEffect(()=>{
    if(focusRequest&&status?.supported)recordButton.current?.focus({preventScroll:true});
  },[focusRequest,status?.supported]);
  useEffect(()=>{
    mounted.current=true;
    if(!native) return;
    let received=false;
    const stop=listen<Status>('floating-hotkey-updated',event=>{
      received=true;
      if(!mounted.current)return;
      setStatus(event.payload);
      if(!event.payload.recording){capturing.current=false;leased.current=false;setRecording(false);setDraft(null);}
    });
    void stop.then(()=>invoke<Status>('get_floating_hotkey')).then(value=>{if(mounted.current&&!received)setStatus(value);}).catch(()=>{if(mounted.current)setError('无法读取快捷键设置');});
    const cancelOnBlur=()=>{if(leased.current)void cancel();};
    window.addEventListener('blur',cancelOnBlur);
    const keydown=(event:KeyboardEvent)=>{
      if(!capturing.current)return;
      event.preventDefault();event.stopImmediatePropagation();
      try{const value=captureHotkey(event);if(value){setDraft(value);capturing.current=false;setError(null);}}
      catch(error){setError(String(error instanceof Error?error.message:error));}
    };
    window.addEventListener('keydown',keydown,true);
    return ()=>{
      mounted.current=false;capturing.current=false;
      window.removeEventListener('blur',cancelOnBlur);window.removeEventListener('keydown',keydown,true);
      void stop.then(unlisten=>unlisten());
      if(leased.current){leased.current=false;void invoke('record_floating_hotkey',{recording:false}).catch(()=>{});}
    };
  },[native]);
  async function cancel(){
    capturing.current=false;leased.current=false;setRecording(false);setDraft(null);
    try{const next=await invoke<Status>('record_floating_hotkey',{recording:false});if(mounted.current)setStatus(next);}
    catch{if(mounted.current)setError('原快捷键恢复失败，请重试；录入暂停会在 30 秒后自动结束');}
  }
  async function begin(){
    setBusy(true);setError(null);
    try{
      const next=await invoke<Status>('record_floating_hotkey',{recording:true});
      if(!mounted.current){void invoke('record_floating_hotkey',{recording:false});return;}
      setStatus(next);leased.current=true;capturing.current=true;setRecording(true);setDraft(null);recordButton.current?.focus();
    }catch(error){setError(String(error));}finally{if(mounted.current)setBusy(false);}
  }
  async function save(shortcut:FloatingHotkey|null){
    setBusy(true);setError(null);capturing.current=false;
    try{setStatus(await invoke<Status>('set_floating_hotkey',{shortcut}));leased.current=false;setRecording(false);setDraft(null);}
    catch(error){setError(String(error));void cancel();}finally{setBusy(false);}
  }
  return <div className="floating-hotkey-settings">
    <div className="setting-row"><span>展开 / 收缩快捷键</span><strong aria-live="polite">{label(status?.shortcut??null)}</strong></div>
    {native&&status&&!status.supported?<p className="muted">当前平台暂不支持全局悬浮快捷键。</p>:<>
      <button ref={recordButton} type="button" className="settings-action secondary" disabled={!native||!status?.supported||busy} onClick={()=>void begin()}>{recording?(draft?`已录入：${label(draft)}，点击重录`:'请按单键或组合键…'):'录入快捷键'}</button>
      {recording&&<><button type="button" className="settings-action" disabled={!draft||busy} onClick={()=>void save(draft)}>保存快捷键</button><button type="button" className="text-button" disabled={busy} onClick={()=>void cancel()}>取消</button></>}
      {!recording&&status?.shortcut&&<button type="button" className="text-button" disabled={busy} onClick={()=>void save(null)}>清除快捷键</button>}
      <p className="muted">按一次展开并保持，再按一次收缩；聚焦悬浮窗时 Esc 也可收起。支持 F8 等单键或 {platform==='macos'?'⌘ + Shift + K':'Ctrl + Shift + K'} 等组合键，系统保留键除外。单键绑定会占用该键原有输入。</p>
      {recording&&<p className="muted">原快捷键暂时停用，30 秒内保存；离开此窗口会取消录入。</p>}
      {!enabled&&status?.shortcut&&<p className="muted">开启“显示悬浮窗”后快捷键生效。</p>}
      {enabled&&status?.shortcut&&!status.active&&!recording&&!status.error&&<p className="muted">快捷键尚未生效，请重新保存。</p>}
    </>}
    {(error||status?.error)&&<p className="window-warning" role="alert">{error||status?.error}</p>}
  </div>;
}

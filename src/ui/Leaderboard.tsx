import { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { TokenCount } from './TokenCount.tsx';
import { accountGate } from '../domain/accountGate.ts';
type Enrollment = { device: string; nickname: string; active: number; held: number };
type Status = { supported: boolean; storageReady?: boolean; loggedIn: boolean; enabled: boolean; accountSync?: boolean; thisDevice: string; lastSuccess: number | null; message: string | null; remote: { enrollment: Enrollment | null; account: { accountId: string; publicId: string | null; nickname: string | null } | null; board: { me: { rank: number|null; tokens: number } | null } } | null };
type Preview = { key: string; days: Record<string, number>; total: number; records: number; since: number; until: number; rebind: boolean; unread?: string[] };
const date = (ms: number) => new Intl.DateTimeFormat('zh-CN', { timeZone: 'Asia/Shanghai', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', hour12: false }).format(ms);
const call = async <T,>(action: string, body: Record<string, unknown> = {}) => {
  try { return await invoke<T>('leaderboard_action', { action, body }); }
  finally { if (!['status','open','preview'].includes(action)) void emit('leaderboard-updated').catch(()=>{}); }
};
export function Leaderboard({ native, onBack, backLabel = '概览' }: { native: boolean; onBack: () => void; backLabel?: string }) {
  const [state, setState] = useState<Status | null>(null);
  const [error, setError] = useState(''); const [notice, setNotice] = useState(''); const [busy, setBusy] = useState(false);
  const [nickname, setNickname] = useState('');
  const [preview, setPreview] = useState<Preview | null>(null);
  const [destructive, setDestructive] = useState<'leave' | 'delete' | null>(null);
  const readGeneration=useRef(0),lastChecked=useRef(0);
  const update = useCallback(async () => { if (native) { const generation=++readGeneration.current;lastChecked.current=Date.now();const next=await call<Status>('status');if(generation===readGeneration.current){setState(next);setError('');} } },[native]);
  useEffect(() => { void update().catch(e => setError(String(e))); }, [update]);
  useEffect(()=>{
    if(!native)return;
    const foreground=()=>{if(!busy&&document.visibilityState!=='hidden'&&Date.now()-lastChecked.current>=10000)void update().catch(e=>setError(String(e)));};
    window.addEventListener('focus',foreground);document.addEventListener('visibilitychange',foreground);
    const subscription=listen('leaderboard-updated',()=>{if(!busy)void update().catch(e=>setError(String(e)));});
    return()=>{window.removeEventListener('focus',foreground);document.removeEventListener('visibilitychange',foreground);void subscription.then(unlisten=>unlisten()).catch(()=>{});};
  },[native,busy,update]);
  const run = async (fn: () => Promise<void>) => { setBusy(true); setError(''); setNotice(''); try { await fn(); } catch (e) { setError(String(e)); setPreview(null); await update().catch(() => {}); } finally { setBusy(false); } };
  const enrollment = state?.remote?.enrollment; const account = state?.remote?.account; const me = state?.remote?.board.me;
  const mine = enrollment?.device === state?.thisDevice;
  const action = (name: string, body: Record<string, unknown> = {}) => run(async () => { ++readGeneration.current;setState(await call<Status>(name, body)); setPreview(null); setDestructive(null); });
  const prepare = (rebind: boolean) => run(async () => { setPreview(await call<Preview>('preview', { rebind })); setNickname(enrollment?.nickname ?? ''); });
  const gate=accountGate(state,!!error);
  return <section className="subview leaderboard-view"><button className="back-button" onClick={onBack}>← {backLabel}</button><h1>我的用量榜</h1>
    <p className="muted">游客单设备 · 官网账号多设备 · 已接入工具总榜和分榜<br />本地监测默认不上传，参榜或连接账号后同步用量计数，不上传对话。</p>
    <button className="settings-action secondary" disabled={!native || state?.supported === false || busy} onClick={() => void run(async () => { await call('open'); })}>{me ? '在官网查看我的排名 ↗' : '打开网页版榜单 ↗'}</button>
    {me && <p className="muted">浏览器会记住此参榜身份，并将我的排名置顶；只传递公开身份编号，不传递登录凭证。可在网页清除定位。</p>}
    {!native && <p className="state-note">界面预览：账号和日志操作只在桌面应用中执行，不使用模拟排名。</p>}
    {error && <p className="window-warning" role="alert">{error}</p>}{notice && <p className="state-note" role="status">{notice}</p>}{state?.message && <p className="state-note">{state.message}</p>}
    {state?.storageReady===false&&<button className="settings-action secondary" disabled={busy} onClick={()=>void action('retry-storage')}>重试恢复本机身份</button>}
    {native&&(gate==='loading'||gate==='unavailable')&&<div className="rank-preview" role="status"><h2>{gate==='loading'?'正在检查账号与设备连接…':'账号状态暂时无法读取'}</h2><p className="muted">读取完成后显示已有身份。官网浏览器登录与这台应用的设备连接分别检查。</p><button disabled={busy} onClick={()=>void run(update)}>重新检查账号状态</button></div>}
    {gate==='visitor' ? <div className="rank-preview">
      <h2>登录官网账号，汇总多设备用量</h2><p className="muted">从这台应用发起连接，在浏览器登录同一邮箱并批准。每台电脑各连接一次；只登录网页不会自动连接应用。</p>
      <button className="settings-action" disabled={busy} onClick={()=>void action('pair')}>登录官网账号并连接这台设备 ↗</button>
      <button disabled={busy} onClick={()=>void action('retry-storage')}>重新检查已有连接</button>
      <h2>先看看，再决定参榜</h2><p className="muted">开始后仅建立本机试用身份，不上传用量。随后预览本周数据、填写昵称，由你确认是否公开。</p>
      <button className="settings-action" disabled={!native || !state || state.supported === false || state.storageReady === false || busy} onClick={() => void action('start')}>{busy ? '处理中…' : '开始免注册试用'}</button>
    </div> : gate==='ready'&&state&&<>
      <div className="rank-summary"><h2>{account?.nickname ?? enrollment?.nickname ?? (state.remote ? '尚未参榜' : '参榜状态暂不可用')}</h2><strong>{me?.rank ? `#${me.rank}` : '—'}</strong><p>{me ? <><TokenCount tokens={me.tokens} /> Tokens · 北京时间本周 · 已同步</> : state.remote ? '暂无公开名次' : '未取得最新排名'}</p>
      <p className="muted">{account ? '已连接官网账号。本机历史自动补传，其他设备连接后按账号合并；可在官网选择公开参榜。' : enrollment?.held ? '记录待核查，暂不展示名次。' : !enrollment ? '先预览本周可上传的记录。' : !mine ? '账号已绑定另一台设备。本机不会上传。' : state.enabled && enrollment.active ? '每 30 分钟自动同步' : '本机上传已暂停；已上传记录仍保留。'}<br />{state.lastSuccess ? `本机最近同步成功：${date(state.lastSuccess)}（北京时间）` : '本机尚未同步成功'}</p></div>
      {!account && <div className="rank-preview"><h2>连接官网账号</h2><p className="muted">每台设备分别连接同一官网账号。连接后默认补传可读取的历史用量；Codex、Claude Code、WorkBuddy 计入账号汇总，TRAE 暂不跨设备汇总。</p><button className="settings-action" disabled={busy} onClick={() => void action('pair')}>在浏览器连接这台设备 ↗</button><button disabled={busy} onClick={() => void run(update)}>连接完成后刷新状态</button></div>}
      {account && <div className="rank-actions"><button disabled={busy} onClick={() => void action('sync')}>立即同步所有已接入工具</button><button disabled={busy} onClick={() => void run(async () => { await call('open-account'); })}>在官网查看账号用量 ↗</button></div>}
      <div className="rank-actions">
        {!account && !enrollment && state.remote && <button className="settings-action" disabled={busy} onClick={() => void prepare(false)}>以游客身份预览本周数据</button>}
        {!state.remote && <button disabled={busy} onClick={() => void run(update)}>重试读取状态</button>}
        {!account && enrollment && mine && <><button disabled={busy || !state.enabled || !enrollment.active} onClick={() => void action('sync')}>立即同步</button><button disabled={busy} onClick={() => void action(state.enabled && enrollment.active ? 'pause' : 'resume')}>{state.enabled && enrollment.active ? '暂停上传' : '恢复上传'}</button></>}
        {enrollment && !mine && <p className="window-warning">试用身份只能在原设备管理，不支持重绑。</p>}
      </div>
      {preview && <div className="rank-preview"><h2>{preview.rebind ? '重绑确认' : '上传内容预览'}</h2><p><strong><TokenCount tokens={preview.total} /> Tokens</strong> · {preview.records} 条去重记录</p>
        {!!preview.unread?.length && <p className="window-warning">统计不完整：{preview.unread.join('、')} 暂未读取，不会把缺失来源算作 0。</p>}
        <p className="muted">{date(preview.since)} — {date(preview.until)}（北京时间）</p>
        {Object.entries(preview.days).map(([day, tokens]) => <div className="rank-day" key={day}><span>{day}</span><TokenCount tokens={tokens} /></div>)}
        {preview.rebind && <p className="window-warning">将删除旧设备的全部参榜统计，本机只从重绑成功时刻开始计数；不会导入本机历史。</p>}
        <label>公开昵称<input value={nickname} onChange={e => setNickname(e.target.value)} maxLength={24} placeholder="不填邮箱或真实敏感信息" disabled={busy} /></label>
        <p className="muted">仅上传工具、数量、事件时间、脱敏去重键和随机设备标识。公开昵称、用量、排名和同步时间；不公开邮箱，不上传额度、路径、对话或原始日志。</p>
        <button className="settings-action" disabled={!nickname.trim() || busy} onClick={() => void action('enroll', { key: preview.key, nickname, confirm: true })}>{preview.rebind ? '确认删除旧统计并重绑' : '参榜并同步所有已接入工具'}</button>
      </div>}
      {!account && <details className="rank-account"><summary>试用身份与数据</summary><p className="muted">暂停保留已有排名；退出榜单删除上传统计；删除身份同时移除凭证和统计。都不删除本机日志。</p>
        <div className="rank-actions">{enrollment && <button disabled={busy} onClick={() => setDestructive('leave')}>退出榜单并删除统计</button>}<button disabled={busy} onClick={() => setDestructive('delete')}>删除试用身份</button></div>
        {destructive && <div className="rank-preview"><p className="window-warning">{destructive === 'leave' ? '确认删除全部上传统计并退出排行榜？' : '确认删除试用身份和全部上传统计？'}此操作无法撤销。</p>
          <div className="rank-actions"><button disabled={busy} onClick={() => void action(destructive, { confirm: true })}>确认删除</button><button disabled={busy} onClick={() => setDestructive(null)}>取消</button></div></div>}
      </details>}
    </>}
    <p className={account ? 'muted' : 'window-warning'}>{account ? '这台设备已连接官网账号。其他电脑需在各自应用中连接同一账号；云端历史可在官网账号页查看，本机日志仍只留在设备上。' : '游客凭证只保存在这台电脑；游客无法跨设备恢复。连接官网账号后可汇总多设备用量，并在账号页查看云端历史。'}</p>
    <p className="muted">周一 00:00 开始新一周，均按北京时间。{account ? '账号优先同步最新用量，并自动补传本机可读取的历史；额度不足时稍后继续。' : '游客离线重试最多补报本周和上周。'}数据来自本机自报日志，不是厂商账单；用量不代表能力，不建议为排名制造消耗。分享卡可在网页版保存。</p>
  </section>;
}

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

type Paths = { executable: string | null; home: string | null };
type ConnectionInfo = { supported: boolean; settings: Paths; executable: string | null; home: string | null; message: string | null };

export function ConnectionSettings({ connected, refreshing }: { connected: boolean; refreshing: boolean }) {
  const [info, setInfo] = useState<ConnectionInfo | null>(null);
  const [executable, setExecutable] = useState('');
  const [home, setHome] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);
  const disabled = connected || refreshing || busy;
  useEffect(() => {
    let cancelled = false;
    void invoke<ConnectionInfo>('get_connection').then(value => {
      if (cancelled) return;
      setInfo(value); setExecutable(value.settings.executable ?? ''); setHome(value.settings.home ?? '');
    }).catch(() => { if (!cancelled) setError('无法读取连接设置，请重新打开设置页。'); });
    return () => { cancelled = true; };
  }, []);

  async function save() {
    if (disabled) return;
    setBusy(true); setError(null); setSaved(false);
    try {
      const next = await invoke<ConnectionInfo>('set_connection', { settings: { executable: executable || null, home: home || null } });
      setInfo(next); setSaved(true);
    } catch (error) { setError(typeof error === 'string' ? error : '连接设置未保存，请重试。'); }
    finally { setBusy(false); }
  }

  if (info && !info.supported) return null;
  return <div className="connection-settings">
    {info && <>
      <p className="muted connection-path">当前程序：{info.executable ?? '未找到'}<br />数据目录：{info.home ?? '不可用'}</p>
      {info.message && <p className="window-warning" role="status">{info.message}</p>}
      <details open={Boolean(info.message)}><summary>手动指定 Codex 来源</summary>
        <p className="muted">填写绝对路径，不加引号。留空时自动查找程序，数据目录沿用 CODEX_HOME 或默认的 .codex 目录。设置只保存在 TokenDaily。</p>
        <label className="connection-field">Codex 程序路径<input aria-label="Codex 程序路径" type="text" value={executable} disabled={disabled} spellCheck={false} placeholder="留空自动查找" onChange={event => { setExecutable(event.target.value); setSaved(false); }} /></label>
        <label className="connection-field">Codex 数据目录<input aria-label="Codex 数据目录" type="text" value={home} disabled={disabled} spellCheck={false} placeholder="留空使用默认目录" onChange={event => { setHome(event.target.value); setSaved(false); }} /></label>
        <p className="muted">程序应指向可信的 Codex 可执行文件。数据目录需已存在，并包含你要读取的 Codex 登录与日志；两种读取共用这个目录。</p>
        {connected ? <p className="muted">先断开连接即可修改来源。</p> : refreshing && <p className="muted">正在结束上次读取，请稍后再修改。</p>}
        <button type="button" className="settings-action secondary" disabled={disabled} onClick={() => void save()}>{busy ? '保存中…' : '保存连接设置'}</button>
      </details>
    </>}
    {error && <p className="window-warning" role="alert">{error}</p>}
    {saved && <p className="muted" role="status">已保存。点击“连接本机 Codex”开始读取。</p>}
  </div>;
}

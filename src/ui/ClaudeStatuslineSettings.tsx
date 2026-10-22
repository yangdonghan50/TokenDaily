import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

type BridgeStatus = { supported: boolean; enabled: boolean; capturedAtMs: number | null; message: string | null };

export function ClaudeStatuslineSettings() {
  const [status, setStatus] = useState<BridgeStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  useEffect(() => {
    let cancelled = false;
    void invoke<BridgeStatus>('get_claude_statusline').then(value => { if (!cancelled) setStatus(value); })
      .catch(() => { if (!cancelled) setError('无法读取状态栏桥接设置，请重新打开设置页。'); });
    return () => { cancelled = true; };
  }, []);

  async function change(enabled: boolean) {
    if (busy) return;
    setBusy(true); setError(null);
    try { setStatus(await invoke<BridgeStatus>('set_claude_statusline', { enabled })); }
    catch (error) { setError(typeof error === 'string' ? error : '设置未保存，Claude Code 配置保持原样。'); }
    finally { setBusy(false); }
  }

  if (!status) return error ? <p className="window-warning" role="alert">{error}</p> : null;
  return <div className="connection-settings">
    <label className="setting-row"><span>从 Claude Code 状态栏读取重置时间</span><input type="checkbox" role="switch" checked={status.enabled} disabled={busy || !status.supported} onChange={event => void change(event.target.checked)} /></label>
    {!status.supported ? <p className="muted">当前平台暂不支持状态栏桥接，重置时间显示为未知。</p> : <>
      <p className="muted">开启后会修改 ~/.claude/settings.json 的 statusLine：原文件先完整备份到 TokenDaily 数据目录，原有状态栏（如 opentoken）照常显示。只记录 5 小时 / 7 天已用百分比、重置时间和采样时间，不记录会话、路径或对话。关闭或清除本工具数据时还原。</p>
      <p className="muted">仅终端里交互使用的 Claude Code（Pro / Max）会提供数据；Claude Desktop 的 Code 标签页不调用状态栏。重置时间过后显示未知，等待新的采样。</p>
      {status.enabled && <p className="muted" role="status">{status.capturedAtMs ? `最近采样：${new Date(status.capturedAtMs).toLocaleString('zh-CN')}` : '已开启，等待终端 Claude Code 的第一次回复。'}</p>}
    </>}
    {status.message && <p className="window-warning" role="status">{status.message}</p>}
    {error && <p className="window-warning" role="alert">{error}</p>}
  </div>;
}

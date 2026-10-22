import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { permissionMessage, testResultMessage } from '../domain/notification.ts';
import type { NotificationPermission } from '../domain/notification.ts';

export function NotificationSettings() {
  const [permission, setPermission] = useState<NotificationPermission | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<string | null>(null);
  const busyRef = useRef(false);
  const mounted = useRef(false);

  async function action(kind: 'refresh' | 'request' | 'test' | 'check') {
    if (busyRef.current) return;
    busyRef.current = true;
    setBusy(true);
    setError(null);
    try {
      if (kind === 'refresh' || kind === 'request') {
        const next = await invoke<NotificationPermission>(kind === 'refresh' ? 'get_notification_permission' : 'request_notification_permission');
        if (mounted.current) setPermission(next);
      } else {
        const next = await invoke<string>(kind === 'test' ? 'send_test_notification' : 'check_test_notification');
        if (mounted.current) setResult(testResultMessage(next));
      }
    } catch (cause) {
      if (mounted.current) {
        setError(typeof cause === 'string' ? cause : '通知操作失败，当前结果未知。');
        if (kind === 'refresh' || kind === 'request') setPermission(null);
        if (kind === 'test' || kind === 'check') setResult(null);
      }
    } finally {
      busyRef.current = false;
      if (mounted.current) setBusy(false);
    }
  }

  useEffect(() => {
    mounted.current = true;
    void action('refresh');
    const refresh = () => { void action('refresh'); };
    window.addEventListener('focus', refresh);
    return () => { mounted.current = false; window.removeEventListener('focus', refresh); };
  }, []);

  return <div className="notification-controls">
    <p className="muted" role="status">{permissionMessage(permission)}</p>
    <button type="button" className="settings-action secondary" disabled={busy} onClick={() => void action('refresh')}>刷新通知权限</button>
    {permission?.supported && permission.authorization === 'not-determined' &&
      <button type="button" className="settings-action secondary" disabled={busy || import.meta.env.DEV} onClick={() => void action('request')}>申请系统通知权限</button>}
    <button type="button" className="settings-action secondary" disabled={busy || import.meta.env.DEV} onClick={() => void action('test')}>发送测试通知</button>
    {permission?.supported && <button type="button" className="settings-action secondary" disabled={busy || import.meta.env.DEV} onClick={() => void action('check')}>检查测试通知</button>}
    {busy && <p className="muted" role="status">正在等待系统结果…</p>}
    {error && <p className="window-warning" role="alert">{error}</p>}
    {result && <p className="muted" role="status">{result}</p>}
  </div>;
}

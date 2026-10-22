import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export function useFloatingEdge(native: boolean) {
  const [enabled, setEnabled] = useState(false);
  const [ready, setReady] = useState(!native);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const saving = useRef(false);
  useEffect(() => {
    if (!native) { setReady(true); return; }
    let disposed = false;
    let updated = false;
    const subscription = listen<boolean>('floating-edge-updated', event => {
      updated = true;
      if (!disposed) { setEnabled(event.payload); setError(null); }
    });
    void subscription.then(() => invoke<boolean>('get_floating_edge'))
      .then(value => { if (!disposed && !updated) setEnabled(value); })
      .catch(() => { if (!disposed) setError('贴边隐藏设置读取失败，请重试'); })
      .finally(() => { if (!disposed) setReady(true); });
    return () => { disposed = true; void subscription.then(stop => stop()).catch(() => {}); };
  }, [native]);
  async function change(next: boolean) {
    if (saving.current || !ready) return;
    saving.current = true; setBusy(true); setError(null);
    try {
      if (native) await invoke<boolean>('set_floating_edge', { enabled: next });
      else setEnabled(next);
    } catch { setError('贴边隐藏设置未保存，请重试'); }
    finally { saving.current = false; setBusy(false); }
  }
  return { enabled, busy: busy || !ready, error, change };
}

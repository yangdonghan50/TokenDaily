import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ProviderView } from '../domain/provider.ts';

export function useProvider(native: boolean, initial: ProviderView = 'all') {
  const [provider, setProvider] = useState<ProviderView>(initial);
  const [ready, setReady] = useState(!native);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const pending = useRef(false);
  useEffect(() => {
    if (!native) { setReady(true); return; }
    let stopped = false, received = false;
    // WebViews can mount before Tauri setup has registered managed state.
    async function readyCall<T>(call: () => Promise<T>): Promise<T> {
      for (let attempt = 0; ; attempt++) {
        try { return await call(); }
        catch (error) { if (stopped || attempt >= 4) throw error; await new Promise(resolve => window.setTimeout(resolve, 200 * (attempt + 1))); }
      }
    }
    const subscription = readyCall(() => listen<ProviderView>('provider-updated', event => {
      received = true;
      if (!stopped) { setProvider(event.payload); setReady(true); setError(null); }
    }));
    void subscription.then(() => readyCall(() => invoke<ProviderView>('get_provider'))).then(value => {
      if (!stopped) { if (!received) setProvider(value); setReady(true); setError(null); }
    }).catch(() => { if (!stopped) { setError('无法读取监测来源设置'); setReady(true); } });
    return () => { stopped = true; void subscription.then(stop => stop()).catch(() => {}); };
  }, [native]);
  async function change(next: ProviderView) {
    if (pending.current || next === provider) return;
    pending.current = true; setBusy(true); setError(null);
    try {
      if (native) await invoke<ProviderView>('set_provider', { provider: next });
      else setProvider(next);
    } catch { setError('切换失败，请重试'); }
    finally { pending.current = false; setBusy(false); }
  }
  return { provider, ready, busy, error, change };
}

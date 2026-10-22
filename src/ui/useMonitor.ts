import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { providerConfig, type Provider } from '../domain/provider.ts';
import type { QuotaSnapshot } from '../domain/quota.ts';
import type { UsageState } from '../domain/usage.ts';
export type MonitorState = { status: string; connected: boolean; snapshot: QuotaSnapshot | null; identityLabel: string | null; lastSuccessAtMs: number | null; message: string | null };
export const emptyUsage: UsageState = { status: 'unknown', snapshot: null, lastSuccessAtMs: null, message: null };
const emptyState: MonitorState = { status: 'disconnected', connected: false, snapshot: null, identityLabel: null, lastSuccessAtMs: null, message: null };
export function useMonitor(provider: Provider, enabled: boolean, initialRefresh: boolean) {
  const source = providerConfig(provider);
  const [state, setState] = useState(emptyState);
  const [usage, setUsage] = useState(emptyUsage);
  const [refreshing, setRefreshing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const epoch = useRef(0);
  const pending = useRef(false);
  useEffect(() => {
    const generation = ++epoch.current;
    if (!enabled) return;
    let stopped = false;
    const subscriptions: Promise<() => void>[] = [];
    function subscribe<T>(event: string, command: string, update: (value: T) => void) {
      let received = false;
      const subscription = listen<T>(event, e => { received = true; if (!stopped) update(e.payload); });
      subscriptions.push(subscription);
      void subscription.then(() => invoke<T>(command)).then(value => { if (!stopped && !received) update(value); }).catch(() => { if (!stopped) setError(`${source.label} 本地服务暂不可用`); });
    }
    subscribe<MonitorState>(source.stateEvent, source.stateCommand, setState);
    subscribe<UsageState>(source.usageEvent, source.usageCommand, setUsage);
    subscribe<boolean>(source.refreshEvent, source.refreshingCommand, setRefreshing);
    if (initialRefresh) void Promise.all(subscriptions).then(() => { if (!stopped) return invoke(source.refreshCommand); }).catch(() => { if (!stopped) setError('刷新失败，请重试'); });
    return () => { stopped = true; if (epoch.current === generation) epoch.current++; for (const s of subscriptions) void s.then(stop => stop()).catch(() => {}); };
  }, [enabled, provider, initialRefresh]);
  async function command(name: string) {
    if (!enabled || pending.current) return;
    const generation = epoch.current;
    pending.current = true; setBusy(true); setError(null);
    try { await invoke(name); }
    catch { if (epoch.current === generation) setError(`${source.label} 操作失败，请重试`); }
    finally { pending.current = false; setBusy(false); }
  }
  const displaySource = provider === 'claude' && state.snapshot?.identityKey.startsWith('claude:desktop:')
    ? { ...source, label: 'Claude Desktop' } : source;
  return { provider, source: displaySource, state, usage: state.connected ? usage : emptyUsage, refreshing: refreshing || busy, error,
    refresh: () => command(source.refreshCommand), connect: () => command(source.connectCommand), disconnect: () => command(source.disconnectCommand) };
}
export type Monitor = ReturnType<typeof useMonitor>;

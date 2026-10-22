import { NotificationSettings } from './NotificationSettings.tsx';
import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { emitTo, listen } from '@tauri-apps/api/event';
import { demoScenarios, demoSnapshot, demoUsage, type DemoScenario } from '../demo.ts';
import { formatRemaining, quotaFreshness, resetPresentation, type QuotaSnapshot } from '../domain/quota.ts';
import { ConnectionSettings } from './ConnectionSettings.tsx';
import { ClaudeStatuslineSettings } from './ClaudeStatuslineSettings.tsx';
import { usageDatePresentation, participatesInTotal, type UsageState } from '../domain/usage.ts';
import { TokenCount } from './TokenCount.tsx';
import { UsagePanel } from './UsagePanel.tsx';
import { FloatingPanel } from './FloatingPanel.tsx';
import { FloatingHotkeySettings } from './FloatingHotkeySettings.tsx';
import { useFloatingEdge } from './useFloatingEdge.ts';
import { useProvider } from './useProvider.ts';
import { providerConfig } from '../domain/provider.ts';
import { useMonitor } from './useMonitor.ts';
import { monitorStatus } from './OverviewPanel.tsx';
import { combinedUsage } from '../domain/combinedUsage.ts';
import { metricUpdateLabel } from '../domain/refresh.ts';
import { floatingUsage } from '../domain/floatingUsage.ts';
import { localTodayUsage } from '../domain/localUsage.ts';
import { useWorkBuddyCredits, CreditCard, CreditSettings } from './WorkBuddyCredits.tsx';
import { canRefreshCredits, creditPresentation } from '../domain/credits.ts';
import { FLOATING_ORDER_KEY } from '../domain/floatingBalances.ts';
import { METRIC_OPTIONS, METRIC_ORDER_KEY } from '../domain/floatingMetrics.ts';
import { FLOATING_HIDDEN_KEY } from '../domain/floatingComponents.ts';
import { useFloatingComponents } from './useFloatingComponents.ts';
import brandLogo from '../../assets/TokenDaily_full_logo_gradient.svg';
import { IDENTITY_REQUEST_KEY } from '../domain/floatingIdentity.ts';
import { Leaderboard } from './Leaderboard.tsx';
import { panelDetail, type PanelDetail } from '../domain/panelNavigation.ts';
import { costPresentation } from '../domain/floatingSummary.ts';
import { ProviderLogo } from './ProviderLogo.tsx';
import packageInfo from '../../package.json';
import './settingsWindow.css';

type View = 'details' | 'settings' | 'leaderboard' | 'total-history' | 'cost';
type SettingsSection = 'sources' | 'alerts' | 'floating' | 'general';
type ConnectionStatus = 'disconnected' | 'loading' | 'available' | 'unknown' | 'stale' | 'auth-required' | 'unsupported' | 'error';
type DesktopState = { status: ConnectionStatus; snapshot: QuotaSnapshot | null; identityLabel: string | null; identityVerified: boolean; lastSuccessAtMs: number | null; message: string | null; connected: boolean; usage: UsageState };
type AlertSettings = { lowEnabled: boolean; recoveryEnabled: boolean; lowThreshold: number };
const emptyUsage: UsageState = { status: 'unknown', snapshot: null, lastSuccessAtMs: null, message: null };
const emptyState: DesktopState = { status: 'disconnected', snapshot: null, identityLabel: null, identityVerified: false, lastSuccessAtMs: null, message: null, connected: false, usage: emptyUsage };
const isTauri = '__TAURI_INTERNALS__' in window;
const queryDemo = new URLSearchParams(window.location.search).get('demo');
const initialScenario = import.meta.env.DEV && demoScenarios.some(({ id }) => id === queryDemo) ? queryDemo as DemoScenario : null;

function stateFromDemo(scenario: DemoScenario): DesktopState {
  const snapshot = demoSnapshot(scenario, Date.now());
  return { status: scenario === 'disconnected' ? 'disconnected' : scenario === 'stale' ? 'stale' : snapshot?.status ?? 'unknown',
    snapshot, identityVerified: false, identityLabel: scenario === 'disconnected' ? null : '演示账户',
    lastSuccessAtMs: snapshot?.collectedAtMs ?? null, message: scenario === 'unknown' ? '来源未返回有效百分比' : null,
    connected: scenario !== 'disconnected', usage: demoUsage(scenario, Date.now()) };
}

const notificationMessages: Record<string, string> = {
  idle: '尚无新的提醒提交记录。系统通知权限与免打扰设置请在系统中确认。',
  submitted: '提醒提交完成；系统是否展示仍取决于通知权限与专注模式等设置。',
  retry: '提醒提交失败。至少等待 1 分钟后，仅在新的有效数据仍满足条件时重试。',
  'storage-error': '提醒记录无法安全保存或读取，已暂停发送。请检查应用数据目录权限；若记录损坏，可清除 TokenDaily 本地数据后重新连接。',
  uncertain: '上次提醒提交结果或记录未完全确认，为避免重复不会重发该条提醒。',
  disabled: '开发模式不发送系统通知。',
  'identity-unverified': '来源未返回有效工作区，额度提醒暂停，避免混用身份。',
  unavailable: '暂时无法读取提醒状态。',
};

export function App({ floating = false }: { floating?: boolean }) {
  const selection = useProvider(isTauri && !initialScenario, initialScenario ? 'codex' : 'all');
  // The compact floating window is intentionally an overview. Main-panel
  // selection remains independent so details can still be inspected there.
  const provider = floating ? 'all' : selection.provider;
  const source = providerConfig(provider === 'all' ? 'codex' : provider);
  const overviewEnabled = isTauri && !initialScenario && selection.ready && provider === 'all';
  const codexMonitor = useMonitor('codex', overviewEnabled, !floating);
  const claudeMonitor = useMonitor('claude', overviewEnabled, !floating);
  const workbuddyMonitor = useMonitor('workbuddy', overviewEnabled, !floating);
  const traeMonitor = useMonitor('trae', overviewEnabled, !floating);
  const monitors = [codexMonitor, claudeMonitor, workbuddyMonitor, traeMonitor];
  const credits = useWorkBuddyCredits(isTauri && !initialScenario);
  const traeCredits = useWorkBuddyCredits(isTauri && !initialScenario, 'trae');
  const activeCredits = provider === 'trae' ? traeCredits : credits;
  const creditProvider = provider === 'workbuddy' || provider === 'trae';
  const activeProvider = useRef(provider);
  activeProvider.current = provider;
  const [view, setView] = useState<View>('settings');
  const [settingsSection, setSettingsSection] = useState<SettingsSection>('sources');
  const selectionChange = useRef(selection.change); selectionChange.current = selection.change;
  const openHistory = useRef<() => void>(() => {});
  const [hotkeyRequest, setHotkeyRequest] = useState(0);
  useEffect(()=>{
    if(!isTauri||floating)return;
    const read=()=>{if(localStorage.getItem(IDENTITY_REQUEST_KEY)==='1'){localStorage.removeItem(IDENTITY_REQUEST_KEY);setView('leaderboard');}};
    const storage=(event:StorageEvent)=>{if(event.key===IDENTITY_REQUEST_KEY)read();};
    const subscription=listen('open-identity',read);void subscription.then(read).catch(()=>{});
    window.addEventListener('storage',storage);
    return ()=>{window.removeEventListener('storage',storage);void subscription.then(stop=>stop()).catch(()=>{});};
  },[floating]);
  useEffect(() => {
    if (!isTauri || floating) return;
    const subscription = listen<unknown>('open-detail', event => {
      const request = panelDetail(event.payload);
      if (!request) return;
      if (request.kind === 'provider') { void selectionChange.current(request.provider); setView('details'); }
      else if (request.kind === 'history') openHistory.current();
      else setView(request.kind === 'rank' ? 'leaderboard' : 'cost');
    });
    return () => { void subscription.then(stop=>stop()).catch(()=>{}); };
  }, [floating]);
  useEffect(() => {
    if (!isTauri || floating) return;
    let cancelled=false;
    const check=()=>invoke<boolean>('take_hotkey_settings_request').then(requested=>{
      if(requested&&!cancelled){setView('settings');setSettingsSection('floating');setHotkeyRequest(value=>value+1);}
    });
    const subscription=listen('open-hotkey-settings',()=>{void check();});
    void subscription.then(check);
    return ()=>{cancelled=true;void subscription.then(stop=>stop());};
  }, []);
  useEffect(() => {
    if (!hotkeyRequest || view !== 'settings') return;
    const frame=requestAnimationFrame(()=>{
      const section=document.querySelector('.floating-hotkey-settings');
      section?.scrollIntoView({block:'center'});
    });
    return ()=>cancelAnimationFrame(frame);
  }, [hotkeyRequest,view]);
  const [scenario, setScenario] = useState<DemoScenario | null>(initialScenario);
  const [state, setState] = useState<DesktopState>(() => initialScenario ? stateFromDemo(initialScenario) : emptyState);
  const [localUsage, setLocalUsage] = useState<UsageState>(emptyUsage);
  const [totalHistory, setTotalHistory] = useState<UsageState>(emptyUsage);
  const [totalHistoryLoading, setTotalHistoryLoading] = useState(false);
  const [refreshing, setRefreshing] = useState(false);
  const [backgroundRefreshing, setBackgroundRefreshing] = useState(false);
  const [receivedResult, setReceivedResult] = useState(false);
  const refreshRequested = useRef(false);
  const [lowEnabled, setLowEnabled] = useState(true);
  const [recoveryEnabled, setRecoveryEnabled] = useState(true);
  const [threshold, setThreshold] = useState(20);
  const [connectionRevision, setConnectionRevision] = useState(0);
  const [settingsError, setSettingsError] = useState<string | null>(null);
  const [startupEnabled, setStartupEnabled] = useState(false);
  const [startupError, setStartupError] = useState<string | null>(null);
  const [notificationStatus, setNotificationStatus] = useState('idle');
  const [now, setNow] = useState(Date.now());
  const [floatingEnabled, setFloatingEnabled] = useState(false);
  const floatingComponents = useFloatingComponents(!scenario);
  const [manageComponents, setManageComponents] = useState(false);
  const [floatingSupported, setFloatingSupported] = useState(false);
  const [floatingBusy, setFloatingBusy] = useState(false);
  const [floatingError, setFloatingError] = useState<string | null>(null);
  const floatingEdge = useFloatingEdge(isTauri && !scenario);

  useLayoutEffect(() => {
    if (!scenario) { setState(emptyState); setLocalUsage(emptyUsage); setRefreshing(false); setBackgroundRefreshing(false); setReceivedResult(false); }
  }, [provider]);

  useEffect(() => {
    const timer = window.setInterval(() => setNow(Date.now()), 1_000);
    return () => window.clearInterval(timer);
  }, []);
  useEffect(() => {
    if (!scenario || !import.meta.env.DEV) return;
    setState(stateFromDemo(scenario));
  }, [scenario]);
  useEffect(() => {
    if (scenario || !isTauri || !selection.ready) return;
    setState({ ...emptyState, status: 'loading' });
    setLocalUsage(emptyUsage); setReceivedResult(false); setRefreshing(false); setBackgroundRefreshing(false); refreshRequested.current = false;
    let cancelled = false;
    let receivedUpdate = false;
    let receivedLocal = false;
    const unlistenLocal = listen<UsageState>(source.usageEvent, event => {
      receivedLocal = true;
      if (!cancelled) { setLocalUsage(event.payload); setNow(Date.now()); }
    });
    void unlistenLocal.then(() => invoke<UsageState>(source.usageCommand)).then(value => {
      if (!cancelled && !receivedLocal) setLocalUsage(value);
    }).catch(() => { if (!cancelled) setLocalUsage({ ...emptyUsage, status: 'error', message: '本机日志采集服务暂不可用' }); });
    const unlistenState = listen<DesktopState>(source.stateEvent, (event) => {
      receivedUpdate = true;
      if (!cancelled) { setState(event.payload); setNow(Date.now()); setReceivedResult(true); }
    });
    let receivedRefreshStatus = false;
    const unlistenRefresh = listen<boolean>(source.refreshEvent, event => {
      receivedRefreshStatus = true;
      if (!cancelled) setBackgroundRefreshing(event.payload);
    });
    void unlistenRefresh.then(() => invoke<boolean>(source.refreshingCommand)).then(value => {
      if (!cancelled && !receivedRefreshStatus) setBackgroundRefreshing(value);
    }).catch(() => {});
    void unlistenState.then(() => invoke<DesktopState>(source.stateCommand))
      .then((cached) => {
        if (!cancelled && !receivedUpdate) setState(cached);
        // Only the main view requests the initial refresh; floating uses the shared stream.
        if (!cancelled && !floating && provider !== 'all') return invoke<DesktopState>(source.refreshCommand);
      })
      .catch(() => { if (!cancelled) setState((old) => ({ ...old, status: old.snapshot ? 'stale' : 'error', message: '本地服务暂不可用' })); });
    const applyPreferences = (saved: AlertSettings) => {
      setLowEnabled(saved.lowEnabled);
      setRecoveryEnabled(saved.recoveryEnabled);
      setThreshold(saved.lowThreshold);
    };
    void invoke<AlertSettings>('get_preferences').then(applyPreferences).catch(() => setSettingsError('提醒设置暂不可用'));
    if (!floating) void invoke<boolean>('get_autostart').then(setStartupEnabled).catch(() => setStartupError('无法读取开机启动状态'));
    const unlistenPreferences = listen<AlertSettings>('preferences-updated', (event) => applyPreferences(event.payload));
    const unlistenFloating = listen<boolean>('floating-updated', (event) => { setFloatingEnabled(event.payload); setFloatingError(null); });
    void unlistenFloating.then(() => invoke<boolean>('get_floating_enabled')).then(setFloatingEnabled).catch(() => setFloatingError('无法读取悬浮窗设置'));
    if (!floating) void invoke<boolean>('get_floating_supported').then(setFloatingSupported).catch(() => setFloatingSupported(false));
    const unlistenFloatingError = listen<string>('floating-error', event => setFloatingError(event.payload || '悬浮窗操作失败，请重试'));
    let receivedNotificationStatus = false;
    const unlistenNotificationStatus = listen<string>(provider === 'claude' ? 'claude-notification-status' : 'notification-status', event => {
      receivedNotificationStatus = true;
      if (!cancelled) setNotificationStatus(event.payload);
    });
    void unlistenNotificationStatus.then(() => invoke<string>(provider === 'claude' ? 'get_claude_notification_status' : 'get_notification_status')).then(value => {
      if (!cancelled && !receivedNotificationStatus) setNotificationStatus(value);
    }).catch(() => { if (!cancelled) setNotificationStatus('unavailable'); });
    const unlistenSettings = listen<{section?:string}>('open-settings', event => {
      setView('settings');
      if(event.payload?.section==='floating')setSettingsSection('floating');
    });
    return () => {
      cancelled = true;
      for (const subscription of [unlistenNotificationStatus, unlistenState, unlistenLocal, unlistenRefresh, unlistenSettings, unlistenPreferences, unlistenFloating, unlistenFloatingError]) {
        void subscription.then((stop) => stop()).catch(() => {});
      }
    };
  }, [scenario, floating, provider, selection.ready]);

  async function changeFloating(enabled: boolean) {
    if (floatingBusy) return;
    setFloatingBusy(true);
    setFloatingError(null);
    try {
      setFloatingEnabled(isTauri && !scenario ? await invoke<boolean>('set_floating_enabled', { enabled }) : enabled);
    } catch { setFloatingError('悬浮窗设置未保存，请重试'); }
    finally { setFloatingBusy(false); }
  }

  async function openMain() {
    if (!isTauri) return;
    try { await invoke('open_main_panel'); await emitTo('main','open-settings'); }
    catch { setFloatingError('打开设置失败，请从托盘打开'); }
  }

  async function openDetail(request: PanelDetail) {
    if (!isTauri) return;
    try { await invoke('open_main_panel'); await emitTo('main', 'open-detail', request); }
    catch { setFloatingError('打开详情失败，请从托盘打开设置'); }
  }

  async function openIdentity() {
    if(!isTauri)return;
    try{localStorage.setItem(IDENTITY_REQUEST_KEY,'1');await invoke('open_main_panel');await emitTo('main','open-identity');}
    catch{setFloatingError('打开身份页面失败，请从托盘打开设置');}
  }

  async function configureComponents() {
    if (!isTauri) return;
    try { await invoke('open_main_panel'); await emitTo('main', 'open-settings', {section:'floating'}); }
    catch { setFloatingError('打开设置失败，请从托盘重试'); }
  }

  async function refresh() {
    if (refreshRequested.current || backgroundRefreshing) return;
    refreshRequested.current = true;
    setRefreshing(true);
    try {
      if (scenario && import.meta.env.DEV) { setState(stateFromDemo(scenario)); setNow(Date.now()); setReceivedResult(true); }
      // Native results arrive through the shared stream. A concurrent call can return
      // cached data immediately; never overwrite a newer event with that response.
      else if (isTauri) await Promise.all([
        invoke<DesktopState>(source.refreshCommand),
        creditProvider && state.connected && canRefreshCredits(activeCredits.state) ? activeCredits.refresh() : Promise.resolve(),
      ]);
    } catch {
      if (activeProvider.current !== provider) return;
      setReceivedResult(true);
      setState((old) => ({ ...old, status: old.snapshot ? 'stale' : 'error', message: '更新失败，请稍后重试',
        usage: { ...old.usage, status: old.usage.snapshot ? 'stale' : 'error', message: '更新失败，请稍后重试' } }));
    } finally { if (activeProvider.current === provider) { refreshRequested.current = false; setRefreshing(false); } }
  }

  async function openTotalHistory() {
    setView('total-history');
    setTotalHistoryLoading(true);
    try {
      if (scenario && import.meta.env.DEV) {
        setTotalHistory(state.usage);
      } else if (isTauri) {
        setTotalHistory(await invoke<UsageState>('get_local_total_history'));
      } else {
        setTotalHistory({ ...emptyUsage, status: 'unknown', message: '总 Token 历史只能在 TokenDaily 桌面应用中读取。' });
      }
    } catch {
      setTotalHistory({ ...emptyUsage, status: 'error', message: '无法读取本机总 Token 历史，请检查日志目录权限后重试。' });
    } finally {
      setTotalHistoryLoading(false);
    }
  }
  openHistory.current = () => { void openTotalHistory(); };

  async function connect() {
    if (!isTauri) return;
    setState((old) => ({ ...old, status: 'loading' }));
    try { const next = await invoke<DesktopState>(source.connectCommand); if (activeProvider.current === provider) setState(next); }
    catch (error) { if (activeProvider.current === provider) setState((old) => ({ ...old, status: 'error', message: typeof error === 'string' ? error : `无法连接本机 ${source.label}` })); }
  }

  async function disconnect() {
    if (isTauri) try { const next = await invoke<DesktopState>(source.disconnectCommand); if (activeProvider.current === provider) setState(next); }
    catch { if (activeProvider.current === provider) setSettingsError('断开连接失败，请重试'); }
  }

  async function clearData() {
    if (!isTauri || !window.confirm('清除 TokenDaily 本地状态与设置，并删除本机试用凭证、停止上传？凭证删除后无法找回身份或管理旧统计；请先在用量榜删除试用身份及统计。此操作本身不删除云端统计，也不会修改 Codex、Claude、WorkBuddy 或 TRAE 登录与会话。')) return;
    try { setState(await invoke<DesktopState>('clear_local_data')); }
    catch { setSettingsError('本地数据清除失败，请重试；积分授权可能仍保留。'); return; }
    try { localStorage.removeItem(FLOATING_ORDER_KEY); localStorage.removeItem(FLOATING_HIDDEN_KEY); localStorage.removeItem(METRIC_ORDER_KEY); window.dispatchEvent(new StorageEvent('storage', { key: null })); } catch { setSettingsError('悬浮窗偏好清除失败'); }
    setLowEnabled(true); setRecoveryEnabled(true); setThreshold(20);
    setConnectionRevision(value => value + 1); setNotificationStatus('idle');
    invoke<boolean>('get_autostart').then(setStartupEnabled).catch(() => setStartupError('无法确认开机启动状态'));
  }

  async function savePreferences(next: AlertSettings) {
    setLowEnabled(next.lowEnabled);
    setRecoveryEnabled(next.recoveryEnabled);
    setThreshold(next.lowThreshold);
    setSettingsError(null);
    if (!isTauri || scenario) return;
    try { await invoke<AlertSettings>('update_preferences', { settings: next }); }
    catch { setSettingsError('设置保存失败，请重试'); }
  }

  async function changeStartup(enabled: boolean) {
    if (!isTauri || import.meta.env.DEV) return;
    setStartupError(null);
    try { setStartupEnabled(await invoke<boolean>('set_autostart', { enabled })); }
    catch { setStartupError('开机启动设置失败，请重试'); }
  }

  const snapshot = state.snapshot;
  const desktopHistory = provider === 'claude' && snapshot?.identityKey.startsWith('claude:desktop:');
  const settingsPreview = Boolean(scenario && import.meta.env.DEV);
  const freshness = snapshot ? quotaFreshness(snapshot, now) : null;
  const stale = state.status === 'stale' || Boolean(snapshot && freshness && !freshness.fresh);
  const statusLabel = creditProvider && state.connected ? creditPresentation(activeCredits.state,now).label : state.status === 'loading' ? '正在获取额度' : state.status === 'disconnected' ? `尚未连接 ${source.label}`
    : state.status === 'auth-required' ? provider === 'claude' ? 'Claude 订阅额度未接入' : '需要重新授权' : state.status === 'unsupported' ? creditProvider ? '积分余额未接入' : '当前连接无法读取额度'
    : state.status === 'error' ? '更新失败' : stale ? '上次成功的数据' : snapshot?.status === 'unknown' ? '额度未知' : desktopHistory ? 'Claude Desktop 本机历史快照' : '额度已读取';
  const windows = snapshot?.windows ?? [];
  const accountUsage = state.usage ?? emptyUsage;
  const usage = settingsPreview ? accountUsage : localTodayUsage(state.connected ? localUsage : emptyUsage, now);
  const latestUsage = usage.snapshot?.days.at(-1);
  const usageDate = settingsPreview ? usageDatePresentation(usage.snapshot?.days ?? [], now)
    : { label: '今日 Token', notice: usage.message ?? (usage.status === 'available' ? null : latestUsage ? '采集暂停 · 保留本日记录' : state.connected ? '正在读取本机日志…' : `连接 ${source.label} 后采集本机日志`) };
  const isRefreshing = refreshing || backgroundRefreshing || creditProvider && activeCredits.state.refreshing && activeCredits.state.status!=='authorizing';
  const allRefreshing = monitors.some(m => m.refreshing) || credits.state.refreshing && credits.state.status!=='authorizing' || traeCredits.state.refreshing && traeCredits.state.status!=='authorizing';
  function refreshConnected() {
    const tasks = monitors.filter(m => m.state.connected).map(m => m.refresh());
    if (workbuddyMonitor.state.connected && canRefreshCredits(credits.state)) tasks.push(credits.refresh());
    if (traeMonitor.state.connected && canRefreshCredits(traeCredits.state)) tasks.push(traeCredits.refresh());
    void Promise.all(tasks);
  }
  const usageUpdate = metricUpdateLabel({ status: state.connected ? usage.status : 'disconnected',
    stale: floatingUsage(usage, now).stale, hasData: Boolean(latestUsage),
    failed: usage.status === 'error' || receivedResult && usage.status === 'stale', lastSuccessAtMs: usage.lastSuccessAtMs }, now);
  if (floating && provider === 'all') {
    const total = combinedUsage(codexMonitor.usage, claudeMonitor.usage, now, workbuddyMonitor.usage, monitors.map(m=>m.state.connected),traeMonitor.usage);
    const groups = monitors.map(monitor => ({ monitor, ...monitorStatus(monitor, now, monitor.provider === 'trae' ? traeCredits.state : credits.state) }));
    const allWindows = groups.flatMap(({ monitor }) => (monitor.state.snapshot?.windows ?? []).map(window => ({ ...window, label: `${monitor.source.label} · ${window.label}` })));
    const stalePools = groups.filter(group => group.stale).flatMap(({ monitor }) => (monitor.state.snapshot?.windows ?? []).map(window => window.poolId));
    return <FloatingPanel
      costSources={monitors.filter(m=>participatesInTotal(m.usage,m.state.connected)).map(m=>m.usage)}
      balanceSources={groups.map(group => ({id:group.monitor.provider,label:group.monitor.source.label,windows:group.monitor.state.snapshot?.windows ?? [],status:group.label,update:group.update,stale:group.stale,message:group.monitor.state.message,connected:group.monitor.state.connected,credits:group.monitor.state.connected ? group.monitor.provider==='workbuddy'?credits.state:group.monitor.provider==='trae'?traeCredits.state:undefined : undefined}))}
      credits={workbuddyMonitor.state.connected ? credits.state : undefined}
      edgeHide={floatingEdge.enabled} windows={allWindows} now={now} threshold={threshold} stale={stalePools.length > 0}
      usage={total.usage} usageLabel={total.complete ? '今日 Token · 合计' : '今日 Token · 已统计'} usageBreakdown={total.entries.map((entry, index) => <span key={entry.label}>{index > 0 && ' · '}{entry.label} <TokenCount tokens={entry.usage.snapshot?.days.at(-1)?.tokens} /></span>)}
      refreshing={allRefreshing} canRefresh={monitors.some(m => m.state.connected)} onRefresh={refreshConnected}
      usageUpdate={total.complete ? '本机自动采集' : '统计不完整'}
      error={floatingError || codexMonitor.error || claudeMonitor.error || workbuddyMonitor.error || traeMonitor.error}
      demo={false} busy={floatingBusy} onClose={() => void changeFloating(false)} onOpen={() => void openMain()} onDetail={(kind,id)=>{const request=panelDetail({kind,provider:id});if(request)void openDetail(request);}} onConfigure={() => void configureComponents()} onIdentity={() => void openIdentity()} />;
  }
  if (floating) return null;
  return <main className="app-shell settings-app">
    {scenario && import.meta.env.DEV && <div className="demo-banner">演示数据 · 非真实账户额度</div>}
    <header className="topbar">
      <div className="brand"><span className="brand-lockup"><img src={brandLogo} alt="TokenDaily" /></span><span className="settings-window-title">设置</span></div>
      {view !== 'settings' && <button className="icon-button" type="button" aria-label="关闭详情，返回设置" onClick={() => setView('settings')}>×</button>}
    </header>
    {view === 'leaderboard' && <Leaderboard native={isTauri && !scenario} onBack={() => setView('settings')} backLabel="设置" />}
    {view === 'total-history' && <section className="subview"><button type="button" className="back-button" onClick={() => setView('settings')}>← 设置</button><h1>总 Token 历史</h1>
      {totalHistoryLoading ? <p className="muted">正在读取本机日志…</p> : <UsagePanel usage={totalHistory} now={now} demo={settingsPreview} />}
    </section>}
    {view === 'cost' && <section className="subview"><button type="button" className="back-button" onClick={() => setView('settings')}>← 设置</button><h1>预估费用</h1><div className="usage-total"><strong>{costPresentation(monitors.filter(m=>participatesInTotal(m.usage,m.state.connected)).map(m=>m.usage),now).amount}</strong></div><p className="muted">{costPresentation(monitors.filter(m=>participatesInTotal(m.usage,m.state.connected)).map(m=>m.usage),now).title}</p></section>}
    {view === 'details' && provider === 'all' && <section className="subview"><p className="muted">正在打开工具详情…</p></section>}
    {provider !== 'all' && view === 'details' && <section className="subview"><button type="button" className="back-button" onClick={() => setView('settings')}>← 设置</button><h1>{source.label} 详情</h1><button type="button" className="refresh-button" onClick={refresh} disabled={isRefreshing || !state.connected || !isTauri && !scenario}>{isRefreshing ? '刷新中…' : '刷新'}</button>
      <dl className="detail-list"><div><dt>账户</dt><dd>{creditProvider ? activeCredits.state.identityLabel ?? '积分未授权' : state.identityLabel ?? '未连接'}</dd></div><div><dt>来源</dt><dd>{scenario ? '演示适配器' : creditProvider ? `${source.label} 官方积分接口 · 只读` : snapshot ? provider === 'claude' ? state.identityVerified ? 'Claude Code 订阅接口 · 只读' : snapshot.windows.some(w => w.resetsAtMs) ? 'Claude Desktop 本机额度记录 + Claude Code 状态栏重置时间 · 只读' : 'Claude Desktop 本机额度记录 · 只读' : 'Codex App Server · 只读' : '尚未接入'}</dd></div>
        <div><dt>覆盖范围</dt><dd>{scenario ? '虚构示例' : provider === 'trae' ? 'TRAE 本地记录不区分账户；恢复/同步会话可能来自其他设备；账户积分通过独立网页登录授权，可能与本地当前账号不同。' : provider === 'workbuddy' ? '本机 WorkBuddy 日志不区分账户；账户积分通过独立网页登录授权，可能与本机当前账号不同。' : provider === 'claude' ? state.identityVerified ? 'Claude Code 已核验订阅账户；同一 Pro / Max 账户的聊天端共享额度' : `Claude Desktop 本机额度比例记录；未独立核验当前账户，额度提醒暂停，${snapshot?.windows.some(w => w.resetsAtMs) ? '重置时间来自同一组织的 Claude Code 状态栏采样' : '重置时间未知'}` : state.identityVerified ? '该快照的账户与工作区已核对；账号日统计覆盖范围仍待核验' : '工作区未核验；仅显示本次读取，额度提醒暂停'}</dd></div><div><dt>额度 / 积分最近读取成功</dt><dd>{creditProvider ? activeCredits.state.balance ? new Date(activeCredits.state.balance.collectedAtMs).toLocaleString('zh-CN') : '暂无' : state.lastSuccessAtMs ? new Date(state.lastSuccessAtMs).toLocaleString('zh-CN') : '暂无'}</dd></div>
        <div><dt>本机用量采集时间</dt><dd>{usage.lastSuccessAtMs ? new Date(usage.lastSuccessAtMs).toLocaleString('zh-CN') : '暂无'}</dd></div><div><dt>本机统计日期（北京）</dt><dd>{latestUsage?.date ?? '未知'}</dd></div></dl>
      {creditProvider && <CreditCard state={activeCredits.state} now={now} details label={source.label}/>}
      {provider !== 'workbuddy' && <><h2>有效额度窗口</h2>{windows.length ? windows.map((window) => <div className="detail-window" key={`${window.poolId}:${window.windowId}`}><strong>{window.label}</strong><span>{formatRemaining(window.remainingPercent)}</span><small>{window.resetsAtMs ? `预计 ${resetPresentation(window.resetsAtMs, now).absolute} 重置` : '重置时间未知'}</small></div>) : <p className="muted">当前没有有效额度数据。</p>}</>}
      <section className="usage-section" aria-label="本机今日用量"><h2>{provider === 'trae' ? '今日 Token · 本地记录' : '本机今日 Token'}</h2>
        <div className="usage-total"><strong><TokenCount tokens={latestUsage?.tokens} /></strong><span>Tokens</span></div>
        <p className="usage-date">北京时间 00:00 起 · 含缓存 · {usageUpdate}</p>
        {usageDate.notice && <p className="state-note">{usageDate.notice}</p>}
        <p className="usage-scope">{provider === 'trae' ? '只读取 TRAE CN 本地数据库的 Token 计数，不写明文数据库文件；按轮次去重，缓存/推理不重复相加。每 2 秒只检查文件信息，变化后最多每 30 秒重读，手动刷新至少间隔 10 秒。北京时间归日；恢复/同步的历史可能来自其他设备，因此称为本地记录。数据库或 WAL 单文件超过 64 MiB 时显示未知，避免过多占用内存；不根据 Token 估算积分。' : creditProvider ? '只统计本机 WorkBuddy projects 日志中的助手响应和工具调用，按会话与记录标识去重。输入含缓存、输出含推理，不重复叠加；每 2 秒检查近期文件变化，每 30 秒发现新文件，手动刷新立即补查。按北京时间归日；不代表全部设备或当前账户的完整用量，Token 不换算为积分余额。' : provider === 'claude' ? '只统计本机 Claude Code 日志，含子代理；输入、输出、缓存读取与缓存创建相加，同一请求的重复记录去重。每 2 秒检查已有文件变化，每 30 秒发现新日志。按北京时间归属日期，不代表当前订阅账号或所有设备的完整用量。' : '只统计当前 Codex 日志目录内的本机记录，日志变化时采集，每 30 秒补查。输入（含缓存）加输出，不重复加缓存或推理字段；不是所有设备或当前账号的完整用量。总览中的“总 Token 历史”只合并本机日志，不使用账号日统计。'}</p>
      </section>
      {provider === 'claude' ? <section className="usage-section"><h2>网页 / 桌面聊天 Token</h2><p className="muted">Claude Desktop 可提供额度使用比例，但不提供精确聊天 Token。上方今日 Token 仅来自本机 Claude Code 日志。</p></section> : null}
    </section>}
    {view === 'settings' && <section className="settings-layout">
      <aside className="settings-sidebar"><span className="settings-sidebar-caption">偏好设置</span><nav aria-label="设置分类">
        {([['sources','数据来源'],['alerts','额度提醒'],['floating','悬浮窗'],['general','通用']] as const).map(([id,label])=><button key={id} type="button" className={settingsSection===id?'active':''} aria-current={settingsSection===id?'page':undefined} onClick={()=>setSettingsSection(id)}>{label}</button>)}
      </nav><p>设置更改即时生效</p></aside>
      <div className="settings-main" key={settingsSection}>
      {settingsSection === 'sources' && <><h1>数据来源</h1><p className="settings-lead">管理已接入工具与额度授权。用量和余额请在悬浮窗查看。</p>
      {selection.error && <p className="window-warning" role="alert">{selection.error}</p>}
      {provider === 'all' ? <div className="settings-source-list">{monitors.map(monitor => <button type="button" className="settings-source-item" key={monitor.provider} disabled={selection.busy || !selection.ready} onClick={() => void selection.change(monitor.provider)}><ProviderLogo provider={monitor.provider}/><span><strong>{monitor.source.label}</strong><small>{monitor.provider==='claude'?'Claude Desktop 额度 / Code 日志':monitor.provider==='trae'?'TRAE 本地记录':monitor.provider==='workbuddy'?'WorkBuddy 本机日志':'Codex 本机日志'}</small></span><span className="settings-source-state">{monitor.state.message ?? monitorStatus(monitor, now, monitor.provider==='trae'?traeCredits.state:credits.state).label}</span><b aria-hidden="true">›</b></button>)}</div> : <div className="settings-group"><button type="button" className="back-button" onClick={()=>void selection.change('all')}>← 所有数据来源</button><div className="settings-provider-heading"><ProviderLogo provider={provider}/><h2>{source.label}</h2></div><p>{statusLabel}</p><p className="muted">{creditProvider ? `只读 ${source.label} 本地记录统计 Token；账户积分通过官方网页登录独立授权，不根据 Token 估算。` : provider === 'claude' ? '额度优先使用本机 Claude Code 订阅登录（可核验账户，含重置时间，可发送提醒）；没有可用登录时读取 Claude Desktop 本机额度记录（仅百分比）。macOS 可另行开启下方状态栏桥接补充重置时间。Token 仅统计本机 Claude Code 日志。' : '读取本机 Codex 当前活动账户。'}断开连接只停止本来源的 TokenDaily 采集，不会注销 {source.label}。</p>
      {state.message && <p className={provider === 'claude' && snapshot && state.status === 'available' ? 'muted' : 'window-warning'}>{state.message}</p>}
      {provider === 'claude' && desktopHistory && snapshot && <section className="claude-snapshot-preview" aria-label="Claude Desktop 本机额度快照"><h3>{state.identityLabel ?? 'Claude Desktop'} · {stale ? '上次采样' : '本机快照'}</h3>{windows.map(window => <div className="detail-window" key={`${window.poolId}:${window.windowId}`}><strong>{window.label}</strong><span>{formatRemaining(window.remainingPercent)}</span><small>剩余额度 · {window.resetsAtMs ? `预计 ${resetPresentation(window.resetsAtMs, now).absolute} 重置` : '重置时间未知'}</small></div>)}<p className="muted">采样于 {new Date(snapshot.collectedAtMs).toLocaleString('zh-CN')}；本机快照无法核验当前账户，因此暂停额度提醒。</p></section>}
      {provider === 'claude' && <p className="muted">额度约每 5 分钟检查一次本机记录；失败后退避重试，手动刷新成功后间隔至少 10 秒。Claude Desktop 自行更新记录；可在其设置中打开“用量”页核对。网页 / 桌面聊天的精确 Token 暂未接入。</p>}
      {!scenario && isTauri && <button type="button" className="settings-action" disabled={state.status === 'loading' || provider === 'trae' && !state.connected && state.status==='unsupported'} onClick={state.connected ? disconnect : connect}>{state.connected ? '断开连接' : `连接本机 ${source.label}`}</button>}{provider === 'codex' && !scenario && isTauri && <ConnectionSettings key={connectionRevision} connected={state.connected} refreshing={isRefreshing} />}{provider === 'claude' && !scenario && isTauri && <ClaudeStatuslineSettings />}</div>}
      {provider === 'claude' && state.connected && <button type="button" className="settings-action secondary" onClick={() => setView('details')}>查看 Claude 额度与本机用量</button>}
      {provider === 'workbuddy' && <CreditSettings credits={credits} connected={state.connected} now={now}/>}
      {provider === 'trae' && <CreditSettings credits={traeCredits} label="TRAE" connected={state.connected} now={now}/>}
      </>}
      {settingsSection === 'alerts' && <><h1>额度提醒</h1><p className="settings-lead">只在新额度数据确认风险或恢复时提醒。</p><div className="settings-group"><h2>通知</h2>{!settingsPreview && <p className="muted" role="status">{provider === 'claude' ? 'Claude' : 'Codex'}：{notificationMessages[notificationStatus] ?? notificationMessages.unavailable}</p>}<p className="muted">{settingsPreview ? '演示场景不会发送系统通知。' : '仅在读取到新额度后提醒；恢复时间倒计时结束不会触发提醒。'}</p>{settingsError && <p className="window-warning">{settingsError}</p>}<label className="setting-row"><span>低额度提醒</span><input type="checkbox" checked={lowEnabled} disabled={!isTauri && !settingsPreview} onChange={(event) => void savePreferences({ lowEnabled: event.target.checked, recoveryEnabled, lowThreshold: threshold })} /></label>
        <label className="setting-row"><span>提醒阈值</span><select value={threshold} disabled={!isTauri && !settingsPreview} onChange={(event) => void savePreferences({ lowEnabled, recoveryEnabled, lowThreshold: Number(event.target.value) })}><option value={10}>剩余 10%</option><option value={20}>剩余 20%</option><option value={30}>剩余 30%</option></select></label>
        <label className="setting-row"><span>确认恢复提醒</span><input type="checkbox" checked={recoveryEnabled} disabled={!isTauri && !settingsPreview} onChange={(event) => void savePreferences({ lowEnabled, recoveryEnabled: event.target.checked, lowThreshold: threshold })} /></label>{isTauri && !settingsPreview && <NotificationSettings />}</div></>}
      {settingsSection === 'floating' && <><h1>悬浮窗</h1><p className="settings-lead">设置显示、快捷键和组件；日常数据仍在悬浮窗查看。</p><div className="settings-group"><h2>显示与交互</h2>{isTauri && !floatingSupported && <p className="muted">当前平台的悬浮窗仍在适配中。</p>}<label className="setting-row"><span>显示悬浮窗</span><input type="checkbox" role="switch" checked={floatingEnabled} disabled={floatingBusy || isTauri && !floatingSupported || !isTauri && !settingsPreview} onChange={(event) => void changeFloating(event.target.checked)} /></label>
        <label className="setting-row"><span>贴边隐藏</span><input type="checkbox" role="switch" aria-label="贴边隐藏" checked={floatingEdge.enabled} disabled={floatingEdge.busy || isTauri && !floatingSupported || !isTauri && !settingsPreview} onChange={event => void floatingEdge.change(event.target.checked)} /></label>
        <p className="muted">开启后，拖动时悬浮球碰到屏幕左侧或右侧边缘，未松手就会吸附并露出半个 Logo（鼠标不必到最外侧），继续往回拖可解除；放在边缘附近会保留原位置。松手保持半隐藏，离开后再次悬停展开；拖离恢复悬浮球。开关会自动保存。</p>
        {floatingEdge.error && <p className="window-warning" role="alert">{floatingEdge.error}</p>}
        <FloatingHotkeySettings native={isTauri && !settingsPreview} enabled={floatingEnabled} focusRequest={hotkeyRequest} />
        <p className="muted">顶部指标左右拖动排序，余额与日历上下拖动排序，长按进入编辑；移除只隐藏卡片，不停止采集或改变总 Token。编辑时保持展开，完成后移开自动收起。按住球体或顶部可移动窗口。</p>
        <button type="button" className="settings-action secondary" aria-expanded={manageComponents} disabled={isTauri && !floatingSupported || !isTauri && !settingsPreview} onClick={() => setManageComponents(!manageComponents)}>管理悬浮组件</button>
        {manageComponents && <fieldset className="floating-component-settings"><legend>在悬浮窗显示</legend>
          {METRIC_OPTIONS.map(metric=><label className="setting-row" key={metric.id}><span>{metric.label}<small> · 顶部指标</small></span><input type="checkbox" checked={!floatingComponents.hidden.includes(metric.id)} onChange={event=>floatingComponents.setVisible(metric.id,event.target.checked)}/></label>)}
          {monitors.map(monitor => <label className="setting-row" key={monitor.provider}><span>{monitor.source.label}{!monitor.state.connected && <small> · 需要连接</small>}</span><input type="checkbox" checked={!floatingComponents.hidden.includes(monitor.provider)} onChange={event => floatingComponents.setVisible(monitor.provider, event.target.checked)} /></label>)}
          <label className="setting-row"><span>Token 日历<small> · 本机总 Token</small></span><input type="checkbox" checked={!floatingComponents.hidden.includes('calendar')} onChange={event => floatingComponents.setVisible('calendar', event.target.checked)} /></label>
          <p className="muted">取消勾选仅隐藏卡片；再次勾选即可恢复。连接与授权请在监测来源中设置。</p></fieldset>}
        {floatingComponents.error && <p className="window-warning" role="alert">{floatingComponents.error}</p>}
        {settingsPreview && <p className="muted">演示仅预览设置，不会创建桌面窗口或保存。</p>}{floatingError && <p className="window-warning" role="alert">{floatingError}</p>}</div></>}
      {settingsSection === 'general' && <><h1>通用</h1><p className="settings-lead">管理启动和本地数据。</p><div className="settings-group"><h2>本地应用</h2><p className="muted">只保存必要的设置、脱敏身份、最近额度快照与来源日用量，以及系统加密保护的 WorkBuddy 积分授权和 TRAE 独立官方网页的本机登录数据。</p><label className="setting-row"><span>开机启动</span><input type="checkbox" checked={startupEnabled} disabled={!isTauri || import.meta.env.DEV} onChange={(event) => void changeStartup(event.target.checked)} /></label>{import.meta.env.DEV && <p className="muted">开机启动仅在安装版中可设置。</p>}{startupError && <p className="window-warning">{startupError}</p>}
        {!scenario && isTauri && <button type="button" className="settings-action secondary" onClick={clearData}>清除 TokenDaily 本地数据</button>}</div>
      <p className="privacy-note">TRAE 只读本地数据库，官方积分登录保存在独立网页配置中，不导出 Cookie 或令牌，不读取客户端凭据；本地记录不上传。WorkBuddy 用量仅在本机读取；积分授权由系统安全存储保存在本机，仅用于官方积分接口。不读取或解密 WorkBuddy 客户端凭据，不修改其登录和配置。<br />TokenDaily · {packageInfo.version} 公开测试版<br />默认无遥测，不上传对话或原始日志。Claude 凭据仅用于向 Anthropic 读取账户额度，不保存在 TokenDaily 中。</p></>}
      </div>
    </section>}
    {import.meta.env.DEV && scenario && <div className="demo-switcher"><label htmlFor="demo-scenario">演示场景</label><select id="demo-scenario" value={scenario} onChange={(event) => setScenario(event.target.value as DemoScenario)}>{demoScenarios.map(({ id, label }) => <option key={id} value={id}>{label}</option>)}</select></div>}
  </main>;
}

export type AppUpdateState = {
  phase: 'unsupported' | 'idle' | 'checking' | 'current' | 'available' | 'downloading' | 'installing' | 'checkError' | 'installError';
  currentVersion: string; latestVersion: string | null;
  downloaded: number; total: number | null; message: string | null;
};

export function updatePresentation(state: AppUpdateState) {
  const canInstall = Boolean(state.latestVersion) && ['available', 'installError'].includes(state.phase);
  const disabled = ['unsupported', 'checking', 'downloading', 'installing'].includes(state.phase);
  const progress = state.phase === 'downloading' && state.total && state.total > 0
    ? Math.min(100, Math.max(0, Math.floor(state.downloaded / state.total * 100))) : null;
  const current = state.currentVersion ? ` · 当前 v${state.currentVersion}` : '';
  const version = state.latestVersion ? ` v${state.latestVersion}` : '';
  const titles: Record<AppUpdateState['phase'], string> = {
    unsupported: '此平台暂不支持应用内更新',
    idle: `检查软件更新${current}`,
    checking: `正在检查更新${current}`,
    current: `已是最新版本${current}，点击重新检查`,
    available: `发现新版${version}，点击下载安装并重启`,
    downloading: `正在下载${version}${progress === null ? '' : ` · ${progress}%`}，完成后自动安装并重启`,
    installing: `正在安装${version}，即将重启`,
    checkError: '检查更新失败，点击重试',
    installError: `更新失败，点击${canInstall ? '重新下载安装' : '重新检查'}`,
  };
  return { disabled, progress, action: canInstall ? 'install' as const : 'check' as const,
    attention: ['available', 'checkError', 'installError'].includes(state.phase),
    title: [titles[state.phase], state.message].filter(Boolean).join('。') };
}

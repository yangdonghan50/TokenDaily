export type Provider = 'codex' | 'claude' | 'workbuddy' | 'trae';
export type ProviderView = Provider | 'all';
export function providerConfig(provider: Provider) {
  if (provider === 'trae') return {
    label:'TRAE',localLabel:'TRAE 本地记录',stateEvent:'trae-state-updated',usageEvent:'trae-local-usage-updated',refreshEvent:'trae-refresh-status',
    stateCommand:'get_trae_state',usageCommand:'get_trae_local_usage',refreshingCommand:'get_trae_refreshing',refreshCommand:'refresh_trae',connectCommand:'connect_trae',disconnectCommand:'disconnect_trae',
  };
  if (provider === 'workbuddy') return {
    label: 'WorkBuddy', localLabel: 'WorkBuddy', stateEvent: 'workbuddy-state-updated', usageEvent: 'workbuddy-local-usage-updated', refreshEvent: 'workbuddy-refresh-status',
    stateCommand: 'get_workbuddy_state', usageCommand: 'get_workbuddy_local_usage', refreshingCommand: 'get_workbuddy_refreshing', refreshCommand: 'refresh_workbuddy', connectCommand: 'connect_workbuddy', disconnectCommand: 'disconnect_workbuddy',
  };
  return provider === 'claude' ? {
    label: 'Claude', localLabel: 'Claude Code', stateEvent: 'claude-state-updated', usageEvent: 'claude-local-usage-updated', refreshEvent: 'claude-refresh-status',
    stateCommand: 'get_claude_state', usageCommand: 'get_claude_local_usage', refreshingCommand: 'get_claude_refreshing', refreshCommand: 'refresh_claude', connectCommand: 'connect_claude', disconnectCommand: 'disconnect_claude',
  } : {
    label: 'Codex', localLabel: 'Codex', stateEvent: 'state-updated', usageEvent: 'local-usage-updated', refreshEvent: 'refresh-status',
    stateCommand: 'get_state', usageCommand: 'get_local_usage', refreshingCommand: 'get_refreshing', refreshCommand: 'refresh_quota', connectCommand: 'connect_codex', disconnectCommand: 'disconnect_codex',
  };
}

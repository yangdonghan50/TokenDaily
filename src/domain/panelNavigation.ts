import type { Provider } from './provider.ts';

export type PanelDetail =
  | { kind: 'provider'; provider: Provider }
  | { kind: 'history' | 'rank' | 'cost' };

export function panelDetail(value: unknown): PanelDetail | null {
  if (!value || typeof value !== 'object') return null;
  const request = value as { kind?: unknown; provider?: unknown };
  if (request.kind === 'provider' && typeof request.provider === 'string' && ['codex', 'claude', 'workbuddy', 'trae'].includes(request.provider))
    return { kind: 'provider', provider: request.provider as Provider };
  if (request.kind === 'history' || request.kind === 'rank' || request.kind === 'cost')
    return { kind: request.kind };
  return null;
}

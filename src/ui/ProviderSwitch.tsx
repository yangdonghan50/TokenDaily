import type { ProviderView } from '../domain/provider.ts';
export function ProviderSwitch({ provider, busy, onChange }: { provider: ProviderView; busy: boolean; onChange: (provider: ProviderView) => void }) {
  return <div className="provider-switch" role="group" aria-label="监测来源">
    <button type="button" aria-pressed={provider === 'all'} disabled={busy} onClick={() => onChange('all')}>总览</button>
    <button type="button" aria-pressed={provider === 'codex'} disabled={busy} onClick={() => onChange('codex')}>Codex</button>
    <button type="button" aria-pressed={provider === 'claude'} disabled={busy} onClick={() => onChange('claude')}>Claude</button>
    <button type="button" aria-pressed={provider === 'workbuddy'} disabled={busy} onClick={() => onChange('workbuddy')}>WorkBuddy</button>
    <button type="button" aria-pressed={provider === 'trae'} disabled={busy} onClick={() => onChange('trae')}>TRAE</button>
  </div>;
}

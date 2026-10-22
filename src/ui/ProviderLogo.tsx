import type { Provider } from '../domain/provider.ts';
import codex from '../../assets/providers/codex.svg';
import claude from '../../assets/providers/claude.svg';
import workbuddy from '../../assets/providers/workbuddy.svg';
import trae from '../../assets/providers/trae.svg';

const logos: Record<Provider, string> = { codex, claude, workbuddy, trae };

export function ProviderLogo({ provider }: { provider: Provider }) {
  return <img className={`provider-logo provider-logo-${provider}`} src={logos[provider]} alt="" aria-hidden="true" draggable={false}/>;
}

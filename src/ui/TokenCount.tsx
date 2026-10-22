import { formatTokenCount, formatExactTokenCount } from '../domain/tokenFormat.ts';

export function TokenCount({ tokens }: { tokens: number | null | undefined }) {
  const exact = tokens == null ? '—' : formatExactTokenCount(tokens);
  const label = exact === '—' ? 'Token 用量未知' : `${exact} Tokens`;
  return <span className="token-count" title={label} aria-label={label}>{tokens == null ? '—' : formatTokenCount(tokens)}</span>;
}

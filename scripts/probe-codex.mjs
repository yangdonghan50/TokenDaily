// M0 only: issue three documented, read-only app-server requests.
// Never print raw responses, account identifiers, credentials, or stderr.
import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { normalizeQuotaResponse } from '../src/domain/quota.ts';

const executable = process.env.TOKENDAILY_CODEX_EXE || 'codex';
const child = spawn(executable, ['app-server', '--listen', 'stdio://'], {
  stdio: ['pipe', 'pipe', 'pipe'],
  windowsHide: true,
});
const result = { transport: 'stdio', initialized: false, account: 'unverified', rateLimits: 'unverified', usage: 'unverified' };
let stderrText = '';
child.stderr.on('data', (chunk) => { stderrText = (stderrText + chunk.toString()).slice(-4000); });
const send = (message) => child.stdin.write(`${JSON.stringify(message)}\n`);
let finished = false;
function finish(reason) {
  if (finished) return;
  finished = true;
  clearTimeout(timer);
  result.endReason = reason;
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
  child.kill(); // Only the child started by this probe.
}
const timer = setTimeout(() => finish('timeout'), 30000);
child.on('error', () => finish('spawn-error'));
child.on('exit', (code) => {
  result.serverExitCode = code;
  result.errorCategory = /拒绝访问|access is denied|permission denied/i.test(stderrText) ? 'access-denied'
    : /configuration|config.toml|failed to load config/i.test(stderrText) ? 'configuration'
    : /authentication|unauthorized/i.test(stderrText) ? 'authentication'
    : stderrText ? 'other-server-error' : 'no-stderr';
  finish('server-exit');
});
const lines = createInterface({ input: child.stdout });
lines.on('line', (line) => {
  let message;
  try { message = JSON.parse(line); } catch { return; }
  if (message.id === 1) {
    if (message.error) { result.initialized = false; finish('initialize-error'); return; }
    result.initialized = true;
    send({ method: 'initialized' });
    send({ method: 'account/read', id: 2, params: { refreshToken: false } });
    send({ method: 'account/rateLimits/read', id: 3 });
    send({ method: 'account/usage/read', id: 4 });
  } else if (message.id === 2) {
    const account = message.result?.account;
    result.account = message.error ? 'error' : account ? {
      type: account.type ?? 'unknown',
      identityPresent: Boolean(account.email || account.id || account.chatgptAccountId),
      emailPresent: typeof account.email === 'string' && account.email.length > 0,
      accountIdPresent: Boolean(account.id || account.chatgptAccountId),
      workspaceRoutingPresent: Boolean(message.result?.workspaceRouting),
      workspaceIdPresent: typeof message.result?.workspaceRouting?.chatgptAccountId === "string" && message.result.workspaceRouting.chatgptAccountId.trim().length > 0,
      workspaceOriginPresent: typeof message.result?.workspaceRouting?.backendOrigin === "string" && message.result.workspaceRouting.backendOrigin.trim().length > 0,
    } : 'none';
  } else if (message.id === 3) {
    const data = message.result;
    if (message.error) result.rateLimits = 'error';
    else {
      const normalized = normalizeQuotaResponse(data, {
        accountRef: 'probe-only', workspaceRef: null, collectedAtMs: Date.now(),
      });
      const pools = data?.rateLimitsByLimitId && Object.keys(data.rateLimitsByLimitId).length
        ? Object.values(data.rateLimitsByLimitId)
        : data?.rateLimits ? [data.rateLimits] : [];
      result.rateLimits = {
        poolCount: pools.length,
        windows: pools.map((pool) => ['primary', 'secondary'].filter((slot) => pool?.[slot] != null)),
        resetFieldsPresent: pools.map((pool) => ['primary', 'secondary'].map((slot) => Number.isFinite(pool?.[slot]?.resetsAt))),
        percentFieldsValid: pools.map((pool) => ['primary', 'secondary'].map((slot) => Number.isFinite(pool?.[slot]?.usedPercent))),
        normalizedWindowCount: normalized.windows.length,
        normalizedStatus: normalized.status,
        normalizedFieldsValid: normalized.windows.every((window) => window.remainingPercent !== null && window.resetsAtMs !== null),
      };
    }
  } else if (message.id === 4) {
    const buckets = message.result?.dailyUsageBuckets;
    result.usage = message.error ? 'error' : {
      dailyBucketsPresent: Array.isArray(buckets),
      bucketCount: Array.isArray(buckets) ? buckets.length : null,
      allDatesIso: Array.isArray(buckets) && buckets.every((bucket) => typeof bucket?.startDate === 'string' && /^\d{4}-\d{2}-\d{2}$/.test(bucket.startDate)),
      allTokenCountsValid: Array.isArray(buckets) && buckets.every((bucket) => Number.isSafeInteger(bucket?.tokens) && bucket.tokens >= 0),
      datesUnique: Array.isArray(buckets) && new Set(buckets.map((bucket) => bucket?.startDate)).size === buckets.length,
      summaryPresent: Boolean(message.result?.summary),
      timezoneFieldPresent: Boolean(message.result?.timezone || message.result?.timeZone),
      coverageFieldPresent: Boolean(message.result?.coverage || message.result?.scope),
    };
  }
  if (result.account !== 'unverified' && result.rateLimits !== 'unverified' && result.usage !== 'unverified') finish('completed');
});
send({ method: 'initialize', id: 1, params: { clientInfo: { name: 'tokendaily_probe', title: 'TokenDaily M0 Probe', version: '0.1.0' } } });

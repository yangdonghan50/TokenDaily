import { test } from 'node:test';
import assert from 'node:assert/strict';
import { providerConfig } from '../src/domain/provider.ts';
test('Claude and Codex commands and streams remain isolated', () => {
  const codex = providerConfig('codex'), claude = providerConfig('claude');
  for (const key of ['stateEvent', 'usageEvent', 'refreshEvent', 'stateCommand', 'usageCommand', 'refreshingCommand', 'refreshCommand', 'connectCommand', 'disconnectCommand'] as const) {
    assert.notEqual(codex[key], claude[key]);
  }
  assert.equal(claude.localLabel, 'Claude Code');
});

test('WorkBuddy commands cannot route to Claude or Codex', () => {
  const wb = providerConfig('workbuddy');
  assert.equal(wb.label, 'WorkBuddy');
  for (const key of ['stateEvent', 'usageEvent', 'refreshEvent', 'stateCommand', 'usageCommand', 'refreshingCommand', 'refreshCommand', 'connectCommand', 'disconnectCommand'] as const) {
    assert.equal(new Set([providerConfig('codex')[key], providerConfig('claude')[key], wb[key]]).size, 3);
  }
});

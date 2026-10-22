import test from 'node:test';
import assert from 'node:assert/strict';
import { panelDetail } from '../src/domain/panelNavigation.ts';

test('component detail requests select only supported views and providers', () => {
  assert.deepEqual(panelDetail({ kind: 'provider', provider: 'trae' }), { kind: 'provider', provider: 'trae' });
  assert.deepEqual(panelDetail({ kind: 'history' }), { kind: 'history' });
  assert.equal(panelDetail({ kind: 'provider', provider: 'unknown' }), null);
  assert.equal(panelDetail({ kind: 'settings' }), null);
  assert.equal(panelDetail(null), null);
});

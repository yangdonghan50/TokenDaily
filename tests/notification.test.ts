import test from 'node:test';
import assert from 'node:assert/strict';
import { permissionMessage, testResultMessage } from '../src/domain/notification.ts';

test('unknown/unsupported/denied permissions never claim notification authorization', () => {
  assert.match(permissionMessage(null), /尚未读取/);
  assert.match(permissionMessage({ supported: false, authorization: 'authorized', alerts: true, center: true }), /不能读取/);
  for (const authorization of ['unknown', 'future-state']) {
    assert.match(permissionMessage({ supported: true, authorization, alerts: true, center: true }), /未知/);
  }
  assert.match(permissionMessage({ supported: true, authorization: 'denied', alerts: true, center: true }), /已关闭/);
});
test('quiet permission and disabled channels remain explicit', () => {
  assert.match(permissionMessage({ supported: true, authorization: 'authorized', alerts: false, center: false }), /均已关闭/);
  assert.match(permissionMessage({ supported: true, authorization: 'provisional', alerts: false, center: true }), /静默/);
});
test('acceptance, center record, absence and activation are distinct evidence', () => {
  assert.match(testResultMessage('accepted'), /尚不能确认/);
  assert.match(testResultMessage('in-center'), /不证明横幅/);
  assert.match(testResultMessage('not-observed'), /不能据此判定失败/);
  assert.match(testResultMessage('clicked'), /点击回调/);
  assert.match(testResultMessage('queued'), /不能确认系统接受/);
  assert.match(testResultMessage('future'), /未知/);
});

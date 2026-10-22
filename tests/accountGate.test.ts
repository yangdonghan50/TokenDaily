import test from 'node:test';
import assert from 'node:assert/strict';
import { accountGate } from '../src/domain/accountGate.ts';
test('checking or failed account reads never offer a replacement guest identity',()=>{
  assert.equal(accountGate(null),'loading');
  assert.equal(accountGate(null,true),'unavailable');
  assert.equal(accountGate({loggedIn:false,storageReady:false}),'blocked');
  assert.equal(accountGate({loggedIn:false,storageReady:true}),'visitor');
  assert.equal(accountGate({loggedIn:true},true),'ready');
});

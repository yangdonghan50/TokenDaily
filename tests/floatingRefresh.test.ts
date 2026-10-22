import test from 'node:test';
import assert from 'node:assert/strict';
import { floatingRefreshState } from '../src/domain/floatingRefresh.ts';
test('shared refresh permits calendar-only, ignores hidden history and locks during visible reads',()=>{
  assert.deepEqual(floatingRefreshState(false,false,true,false),{enabled:true,busy:false});
  assert.deepEqual(floatingRefreshState(false,false,false,true),{enabled:false,busy:false});
  assert.deepEqual(floatingRefreshState(true,false,false,true),{enabled:true,busy:false});
  assert.deepEqual(floatingRefreshState(true,false,true,true),{enabled:true,busy:true});
  assert.deepEqual(floatingRefreshState(true,true,true,false),{enabled:true,busy:true});
});

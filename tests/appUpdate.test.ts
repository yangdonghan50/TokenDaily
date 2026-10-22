import test from 'node:test';
import assert from 'node:assert/strict';
import { updatePresentation, type AppUpdateState } from '../src/domain/appUpdate.ts';

const state = (phase: AppUpdateState['phase'], extra: Partial<AppUpdateState> = {}): AppUpdateState => ({
  phase, currentVersion:'0.1.23', latestVersion:null, downloaded:0, total:null, message:null, ...extra
});
test('update entry distinguishes unknown, current, newer and unsupported without inventing an update', () => {
  assert.equal(updatePresentation(state('idle')).action,'check');
  assert.match(updatePresentation(state('current')).title,/已是最新/);
  assert.equal(updatePresentation(state('available',{latestVersion:'0.1.24'})).action,'install');
  assert.equal(updatePresentation(state('available')).action,'check');
  assert.match(updatePresentation(state('available',{latestVersion:'0.1.24'})).title,/0\.1\.24/);
  assert.equal(updatePresentation(state('unsupported')).disabled,true);
  assert.equal(updatePresentation(state('checkError',{message:'网络不可用'})).attention,true);
  assert.match(updatePresentation(state('checkError',{message:'网络不可用'})).title,/网络不可用/);
});
test('download/install disable repeated actions and errors retry the original available version', () => {
  const downloading=updatePresentation(state('downloading',{downloaded:125,total:100}));
  assert.equal(downloading.disabled,true);
  assert.equal(downloading.progress,100);
  assert.equal(updatePresentation(state('downloading')).progress,null);
  assert.equal(updatePresentation(state('installing')).disabled,true);
  assert.equal(updatePresentation(state('installError',{latestVersion:'0.1.24'})).action,'install');
  assert.equal(updatePresentation(state('installError')).action,'check');
  assert.equal(updatePresentation(state('checking')).disabled,true);
});

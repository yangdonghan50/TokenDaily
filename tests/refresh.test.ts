import test from 'node:test';
import assert from 'node:assert/strict';
import { metricUpdateLabel } from '../src/domain/refresh.ts';

const now = 1_792_051_200_000;
const fresh = { status: 'available', stale: false, failed: false, hasData: true, lastSuccessAtMs: now };
test('each metric reports its own success time and failure without treating old data as refreshed', () => {
  assert.equal(metricUpdateLabel(fresh, now), '刚刚读取');
  assert.equal(metricUpdateLabel({ ...fresh, lastSuccessAtMs: now - 120_000 }, now), '2 分钟前读取');
  assert.equal(metricUpdateLabel({ ...fresh, failed: true, stale: true }, now), '读取失败 · 保留上次');
  assert.equal(metricUpdateLabel({ ...fresh, failed: true, hasData: false }, now), '读取失败 · 暂无数据');
  assert.equal(metricUpdateLabel({ ...fresh, stale: true }, now), '上次数据 · 待读取');
});
test('unknown, disconnected, missing and future timestamps never imply a successful update', () => {
  assert.equal(metricUpdateLabel({ ...fresh, status: 'unknown' }, now), '数据未知');
  assert.equal(metricUpdateLabel({ ...fresh, hasData: false }, now), '暂无数据');
  assert.equal(metricUpdateLabel({ ...fresh, status: 'disconnected' }, now), '未连接');
  assert.equal(metricUpdateLabel({ ...fresh, lastSuccessAtMs: null }, now), '读取时间未知');
  assert.equal(metricUpdateLabel({ ...fresh, lastSuccessAtMs: now + 6000 }, now), '读取时间未知');
  assert.equal(metricUpdateLabel({ ...fresh, lastSuccessAtMs: now + 500 }, now), '刚刚读取');
});

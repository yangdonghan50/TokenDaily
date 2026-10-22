import test from 'node:test';
import assert from 'node:assert/strict';
import { metricFontSize } from '../src/domain/metricFit.ts';
test('fit whole metric including long Mac text without changing digits',()=>{
  assert.equal(metricFontSize(27,130,70),14.33);
  assert.equal(metricFontSize(27,50,70),27);
  assert.equal(metricFontSize(27,130,140),27);
  for(const width of [40,65,90,250]) assert.ok(metricFontSize(27,160,width)*160/27<width);
  assert.equal(metricFontSize(27,0,0),27);
});

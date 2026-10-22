import test from 'node:test';
import assert from 'node:assert/strict';
import { createLayoutRevision } from '../src/domain/floatingLayout.ts';
test('layout requests skipped during a drag remain pending and newer requests survive async apply',()=>{
 const layout=createLayoutRevision();
 const edit=layout.request();assert.equal(layout.pending(),true);
 layout.applied(edit);assert.equal(layout.pending(),false);
 const normal=layout.request();assert.equal(layout.pending(),true,'skipping while dragging must not acknowledge resize');
 const newest=layout.request();layout.applied(normal);assert.equal(layout.pending(),true);
 layout.applied(newest);assert.equal(layout.pending(),false);
});

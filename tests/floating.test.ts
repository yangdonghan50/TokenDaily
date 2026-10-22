import test from 'node:test';
import assert from 'node:assert/strict';
import { clampFloatingPosition, createFloatingHover, createFloatingPress, createFloatingDockDrag, floatingWindowSize, floatingDockPosition, nativeEscapeTarget } from '../src/domain/floating.ts';

test('native ball contact side is preserved without rechecking pointer boundary', () => {
  const drag = createFloatingDockDrag();
  for (const snapSide of ['left', 'right'] as const) {
    drag.begin();
    assert.equal(drag.finish({ moved: true, cancelled: false, snapSide }, true), snapSide);
  }
});

test('only an active drag previews and commits half-logo hiding, never settings or stale events', () => {
  const drag = createFloatingDockDrag();
  const edge = { moved: true, cancelled: false, snapSide: 'left' as const };
  assert.equal(drag.preview('left', true), null);
  drag.begin();
  assert.equal(drag.preview('right', true), 'right', 'show half-logo before release');
  assert.equal(drag.preview(null, true), null, 'retreat restores the full ball while held');
  assert.equal(drag.preview('left', false), null);
  drag.cancel();
  assert.equal(drag.finish(edge, true), null);
  drag.begin(); assert.equal(drag.finish(edge, true), 'left');
  assert.equal(drag.finish(edge, true), null, 'duplicate finish does not rearm docking');
  drag.begin(); assert.equal(drag.finish({ ...edge, moved: false }, true), null);
  drag.begin(); assert.equal(drag.finish({ ...edge, cancelled: true }, true), null);
  drag.begin(); assert.equal(drag.finish({ ...edge, snapSide: null }, true), null, 'moving pointer to edge after release cannot create a snap');
  drag.begin(); assert.equal(drag.finish(edge, false), null);
  drag.begin(); drag.cancel(); assert.equal(drag.finish(edge, true), null);
});

test('docked handle, expanded panel and restored ball stay fully inside the work area', () => {
  const area = { x: -1920, y: -200, width: 1920, height: 1020 };
  for (const size of [{ width: 35, height: 70 }, { width: 375, height: 313 }, { width: 70, height: 70 }]) {
    const right = floatingDockPosition('right', { x: -50, y: 780 }, size, area);
    assert.equal(right.x + size.width, 0);
    assert.equal(right.y + size.height, 820);
    const left = floatingDockPosition('left', { x: -50, y: -300 }, size, area);
    assert.deepEqual(left, { x: -1920, y: -200 });
  }
});

function harness(docked = true) {
  let time = 0;
  let id = 0;
  const jobs = new Map<number, { due: number; run: () => void }>();
  const changes: boolean[] = [];
  const hover = createFloatingHover(value => changes.push(value), {
    schedule(run, delay) { jobs.set(++id, { due: time + delay, run }); return id; },
    cancel(key) { jobs.delete(key); },
  });
  hover.setDocked(docked);
  return { hover, changes, tick(ms: number) {
    time += ms;
    for (const [key, job] of jobs) if (job.due <= time) { jobs.delete(key); job.run(); }
  } };
}

test('live half-logo remains hidden through release until pointer leaves and re-enters', () => {
  const h = harness(false);
  h.hover.enter(); h.hover.drag(true);
  h.hover.setDocked(true); h.hover.accept(false); h.tick(1000);
  assert.deepEqual(h.changes, [], 'held contact cannot expand the half-logo');
  h.hover.settleDock();
  h.hover.observePointer(true, false); h.hover.enter(); h.tick(1000);
  assert.deepEqual(h.changes, [], 'release inside must preserve visible docking feedback');
  h.hover.leave(); h.hover.observePointer(true, false); h.hover.enter(); h.tick(1000);
  assert.deepEqual(h.changes, [], 'synthetic leave during resize cannot unlock hover');
  h.hover.observePointer(false, false); h.hover.enter(); h.tick(180);
  assert.deepEqual(h.changes, [true]);
  h.hover.leave(); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
});

test('free ball only opens on activation; leaving closes and hovering cannot reopen it', () => {
  const h = harness(false);
  h.hover.enter(); h.tick(2000);
  assert.deepEqual(h.changes, []);
  h.hover.activate();
  assert.deepEqual(h.changes, [true]);
  h.hover.leave(); h.tick(400); h.hover.enter(); h.tick(500);
  assert.deepEqual(h.changes, [true], 'moving into panel cancels closing');
  h.hover.observePointer(false, false); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
  h.hover.enter(); h.tick(2000);
  assert.deepEqual(h.changes, [true, false]);
});

test('docking enables hover and undocking cancels a pending expansion', () => {
  const h = harness(false);
  h.hover.setDocked(true); h.hover.enter(); h.tick(100);
  h.hover.setDocked(false); h.tick(200);
  assert.deepEqual(h.changes, []);
  h.hover.setDocked(true); h.tick(180);
  assert.deepEqual(h.changes, [true]);
  h.hover.leave(); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
});

test('dragging and releasing a free ball never activates it', () => {
  const h = harness(false);
  h.hover.enter(); h.hover.drag(true); h.hover.activate(); h.tick(1000);
  h.hover.observePointer(true, false); h.tick(1000);
  assert.deepEqual(h.changes, []);
  h.hover.activate(); h.hover.observePointer(false, false); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
});

test('ball press distinguishes click, jitter, drag, other pointers and cancellation', () => {
  const p = createFloatingPress();
  p.down(1, { x: 10, y: 10 });
  assert.equal(p.move(2, { x: 30, y: 30 }), false);
  assert.equal(p.up(2, { x: 10, y: 10 }), false);
  assert.equal(p.move(1, { x: 12, y: 12 }), false);
  assert.equal(p.up(1, { x: 12, y: 12 }), true);
  p.down(1, { x: 0, y: 0 });
  assert.equal(p.move(1, { x: 5, y: 0 }), true);
  assert.equal(p.move(1, { x: 10, y: 0 }), false, 'only start one native drag');
  assert.equal(p.up(1, { x: 0, y: 0 }), false, 'returning to start cannot turn drag into click');
  p.down(1, { x: 0, y: 0 });
  assert.equal(p.up(1, { x: 12, y: 0 }), false, 'missed movement cannot trigger click');
  p.down(1, { x: 0, y: 0 }); p.cancel();
  assert.equal(p.up(1, { x: 0, y: 0 }), false);
});

test('half-logo viewport keeps half the 48px ball plus outer spacing inside the screen', () => {
  assert.deepEqual(floatingWindowSize(false, true), { width: 28, height: 56 });
  assert.deepEqual(floatingWindowSize(false, false), { width: 56, height: 56 });
  assert.deepEqual(floatingWindowSize(true, true), { width: 300, height: 236 });
  assert.deepEqual(floatingWindowSize(true, false, true), { width: 300, height: 360 });
  assert.deepEqual(floatingWindowSize(false, true, true), { width: 28, height: 56 });
});

test('hover expands, leaving waits, and re-entry cancels collapse', () => {
  const h = harness();
  h.hover.enter(); h.tick(180);
  assert.deepEqual(h.changes, [true]);
  h.hover.leave(); h.tick(400);
  assert.deepEqual(h.changes, [true]);
  h.hover.enter(); h.tick(200);
  assert.deepEqual(h.changes, [true]);
  h.hover.leave(); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
});

test('passing over the ball or dragging does not unexpectedly expand it', () => {
  const h = harness();
  h.hover.enter(); h.tick(50); h.hover.leave(); h.tick(500);
  assert.deepEqual(h.changes, []);
  h.hover.enter(); h.hover.drag(true); h.tick(500);
  assert.deepEqual(h.changes, []);
  h.hover.drag(false); h.tick(180);
  assert.deepEqual(h.changes, [true]);
});

test('keyboard focus keeps the panel available, reset and dispose cancel timers', () => {
  const h = harness();
  h.hover.focus(true);
  h.hover.leave(); h.tick(600);
  assert.deepEqual(h.changes, [true]);
  h.hover.focus(false); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
  h.hover.enter(); h.hover.reset(); h.tick(600);
  assert.deepEqual(h.changes, [true, false]);
  h.hover.enter(); h.hover.dispose(); h.tick(600);
  assert.deepEqual(h.changes, [true, false]);
});

test('editing holds expansion without blocking resizing or depending on mouse buttons',()=>{
  const h=harness(false);h.hover.activate();h.hover.hold(true);h.hover.leave();h.hover.focus(false);
  h.hover.observePointer(false,false);h.tick(1500);assert.deepEqual(h.changes,[true]);assert.equal(h.hover.isDragging(),false);
  h.hover.hold(false);h.tick(500);assert.deepEqual(h.changes,[true,false]);
  h.hover.activate();h.hover.hold(true);h.hover.reset();h.tick(1000);assert.deepEqual(h.changes,[true,false,true,false]);
  assert.equal(floatingWindowSize(true,false,true,3,true).height,424);
  assert.deepEqual(floatingWindowSize(false,true,true,3,true),{width:28,height:56});
});

test('panel expansion stays in work area, including monitors with negative coordinates', () => {
  assert.deepEqual(clampFloatingPosition({ x: 1850, y: 940 }, { width: 375, height: 313 },
    { x: 0, y: 0, width: 1920, height: 1020 }), { x: 1545, y: 707 });
  assert.deepEqual(clampFloatingPosition({ x: -30, y: -20 }, { width: 300, height: 250 },
    { x: -1920, y: -1080, width: 1920, height: 1080 }), { x: -300, y: -250 });
  assert.deepEqual(clampFloatingPosition({ x: 50, y: 50 }, { width: 300, height: 250 },
    { x: 0, y: 0, width: 200, height: 200 }), { x: 0, y: 0 });
});

test('native pointer observation recovers a missing mouseleave after a drag', () => {
  const h = harness();
  h.hover.enter(); h.tick(180);
  h.hover.drag(true);
  h.hover.observePointer(false, true); h.tick(700);
  assert.deepEqual(h.changes, [true], 'do not shrink during native dragging');
  h.hover.observePointer(false, false); h.tick(200);
  h.hover.observePointer(false, false); h.tick(200);
  h.hover.observePointer(false, false); h.tick(100);
  assert.deepEqual(h.changes, [true, false], 'repeated samples must not restart the collapse timer');
});

test('dragging cancels keyboard hold and ignores native focus restoration during drag', () => {
  const h = harness();
  h.hover.focus(true);
  h.hover.drag(true);
  h.hover.focus(true);
  h.hover.observePointer(false, false); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
});

test('released drag stays expanded while pointer is inside then collapses without DOM leave', () => {
  const h = harness();
  h.hover.enter(); h.tick(180); h.hover.drag(true);
  h.hover.observePointer(true, false); h.tick(800);
  assert.deepEqual(h.changes, [true]);
  h.hover.observePointer(false, false); h.tick(300);
  h.hover.observePointer(true, false); h.tick(500);
  assert.deepEqual(h.changes, [true]);
  h.hover.observePointer(false, false); h.tick(500);
  assert.deepEqual(h.changes, [true, false]);
});

test('a vetoed native collapse does not leave the controller thinking the panel is already closed', () => {
  const h = harness();
  h.hover.enter(); h.tick(180);
  h.hover.leave(); h.tick(500);
  // Native cursor check kept the real window open despite a spurious DOM leave.
  h.hover.enter(); h.hover.accept(true);
  h.hover.observePointer(false, false); h.tick(500);
  assert.deepEqual(h.changes, [true, false, false]);
});

test('manual dismiss clears editing, focus and shortcut holds and waits for an actual edge leave', () => {
  const h = harness();
  h.hover.expandShortcut(); h.hover.hold(true); h.hover.focus(true);
  assert.equal(h.hover.dismiss(), true);
  h.hover.accept(false); h.hover.observePointer(true, false); h.hover.enter(); h.tick(1000);
  assert.deepEqual(h.changes, [true, false], 'the collapse button cannot immediately reopen the edge handle');
  h.hover.observePointer(false, false); h.tick(500); h.hover.enter(); h.tick(180);
  assert.deepEqual(h.changes, [true, false, true], 'hover resumes after leaving and re-entering');
  h.hover.drag(true);
  assert.equal(h.hover.dismiss(), true);
  assert.equal(h.hover.isDragging(), false, 'stale drag state cannot veto an explicit collapse');
  assert.equal(h.hover.dismiss(), false, 'native code may reconcile a previously failed collapse');
});

test('native Esc is replayed where the existing panel handlers listen', () => {
  const host = { id: 'host', inside: true };
  const button = { id: 'button', inside: true };
  const mainBody = { id: 'body', inside: false };
  const contains = (parent: typeof host, node: typeof host) => parent === node || node.inside;
  // A focused control keeps edit/drag cancellation ahead of collapsing.
  assert.equal(nativeEscapeTarget(button, host, contains), button);
  // Nothing focused in the panel: the panel root still collapses it.
  assert.equal(nativeEscapeTarget(null, host, contains), host);
  assert.equal(nativeEscapeTarget(mainBody, host, contains), host);
});

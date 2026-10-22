import test from 'node:test';
import assert from 'node:assert/strict';
import { captureHotkey, hotkeyLabel } from '../src/domain/floatingHotkey.ts';
import { createFloatingHover } from '../src/domain/floating.ts';

const key = (code: string, options = {}) => ({ code, ctrlKey: false, altKey: false, shiftKey: false, metaKey: false, repeat: false, isComposing: false, ...options });
test('records single and combination keys by physical code, ignores repeats and modifiers', () => {
  assert.deepEqual(captureHotkey(key('F8')), {key:119,modifiers:0});
  assert.deepEqual(captureHotkey(key('KeyK',{ctrlKey:true,shiftKey:true})), {key:75,modifiers:6});
  assert.equal(hotkeyLabel({key:75,modifiers:6}), 'Ctrl + Shift + K');
  for (const event of [key('ControlLeft'),key('KeyA',{repeat:true}),key('KeyA',{isComposing:true})]) assert.equal(captureHotkey(event), null);
  assert.throws(() => captureHotkey(key('F12')), /系统保留/);
  assert.throws(() => captureHotkey(key('Unidentified')), /不支持/);
});
test('hotkey pins outside pointer, closes while hovered or editing and does not reopen at edge', () => {
  let state = false;
  const timers = new Map<number,()=>void>(); let id=0;
  const hover=createFloatingHover(value=>{state=value;hover.accept(value);}, {schedule:run=>{timers.set(++id,run);return id;},cancel:id=>{timers.delete(id);}});
  const flush=()=>{const queued=[...timers.values()];timers.clear();queued.forEach(run=>run());};
  hover.setDocked(true);
  hover.toggleShortcut(); hover.observePointer(false,false); hover.focus(false); flush();
  assert.equal(state,true);
  hover.enter(); hover.hold(true); hover.toggleShortcut(); flush();
  assert.equal(state,false);
  hover.hold(false); hover.observePointer(true,false); flush(); assert.equal(state,false);
  hover.observePointer(false,false); hover.enter(); flush(); assert.equal(state,true);
  hover.toggleShortcut(); assert.equal(state,false);
  hover.toggleShortcut(); hover.reset(); hover.observePointer(false,false); flush(); assert.equal(state,false);
  hover.drag(true); hover.toggleShortcut(); assert.equal(state,false);
  hover.drag(false); hover.toggleShortcut(); assert.equal(state,true);
});

test('tray expansion opens and holds; repeated open never collapses', () => {
  let state=false;
  const timers: Array<()=>void>=[];
  const hover=createFloatingHover(value=>{state=value;}, {schedule:run=>{timers.push(run);return timers.length;},cancel:()=>{}});
  hover.expandShortcut(); assert.equal(state,true);
  hover.expandShortcut(); hover.observePointer(false,false); timers.forEach(run=>run());
  assert.equal(state,true);
  hover.toggleShortcut(); assert.equal(state,false);
  hover.reset(); hover.drag(true); hover.expandShortcut(); assert.equal(state,false);
});

test('Mac labels Command and Option while Windows labels stay compatible',()=>{
  assert.equal(hotkeyLabel({key:75,modifiers:9},'macos'),'Option + ⌘ + K');
  assert.equal(hotkeyLabel({key:75,modifiers:9}),'Alt + Win + K');
});

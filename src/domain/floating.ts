type Point = { x: number; y: number };
type Size = { width: number; height: number };
export type DockSide = 'left' | 'right';

/** Where a natively captured Esc is replayed: the focused panel control keeps
 * its edit/drag cancellation priority; otherwise the panel root collapses. */
export function nativeEscapeTarget<T>(active: T | null, host: T, contains: (parent: T, node: T) => boolean): T {
  return active !== null && contains(host, active) ? active : host;
}

export function floatingWindowSize(expanded: boolean, hidden: boolean, overview = false, sourceCount = overview ? 3 : 1, editing = false, calendar = false, metricCount = 3): Size {
  return expanded ? { width: 300, height: 174 - (!editing && metricCount === 0 ? 64 : 0) + 62 * Math.max(1, Math.min(calendar ? 5 : 4, sourceCount)) + (editing ? 64 : 0) + (calendar ? 56 : 0) } : { width: hidden ? 28 : 56, height: 56 };
}

// Decide before entering the native drag loop, which may swallow pointerup/click.
export function createFloatingPress() {
  let press: { id: number; origin: Point } | null = null;
  const moved = (point: Point) => press !== null && Math.hypot(point.x - press.origin.x, point.y - press.origin.y) >= 5;
  return {
    down(id: number, origin: Point) { press = { id, origin }; },
    move(id: number, point: Point) {
      if (press?.id !== id || !moved(point)) return false;
      press = null;
      return true;
    },
    up(id: number, point: Point) {
      if (press?.id !== id) return false;
      const click = !moved(point);
      press = null;
      return click;
    },
    cancel() { press = null; },
  };
}

export type FloatingDragEnd = { moved: boolean; cancelled: boolean; snapSide: DockSide | null };

export function createFloatingDockDrag() {
  let active = false;
  return {
    begin() { active = true; },
    cancel() { active = false; },
    preview(side: DockSide | null, enabled: boolean): DockSide | null { return active && enabled ? side : null; },
    finish(end: FloatingDragEnd, enabled: boolean): DockSide | null {
      const eligible = active && enabled && end.moved && !end.cancelled;
      active = false;
      return eligible ? end.snapSide : null;
    },
  };
}

export function floatingDockPosition(side: DockSide, anchor: Point, size: Size, area: Point & Size): Point {
  return clampFloatingPosition({ x: side === 'left' ? area.x : area.x + area.width - size.width, y: anchor.y }, size, area);
}

export function clampFloatingPosition(position: Point, size: Size, area: Point & Size): Point {
  return {
    x: Math.max(area.x, Math.min(position.x, area.x + Math.max(0, area.width - size.width))),
    y: Math.max(area.y, Math.min(position.y, area.y + Math.max(0, area.height - size.height))),
  };
}

// One cancellable timer prevents hover jitter; native dragging temporarily suspends it.
export function createFloatingHover(change: (expanded: boolean) => void, clock: {
  schedule: (run: () => void, delay: number) => number; cancel: (id: number) => void;
}) {
  let timer: number | undefined;
  let pending: boolean | undefined;
  let inside = false;
  let focused = false;
  let held = false;
  let shortcutHeld = false;
  let dragging = false;
  let expanded = false;
  let docked = false;
  let waitForDockLeave = false;
  let disposed = false;
  const cancel = () => { if (timer !== undefined) clock.cancel(timer); timer = undefined; pending = undefined; };
  const apply = (next: boolean) => {
    if (!disposed && expanded !== next) { expanded = next; change(next); }
  };
  const schedule = () => {
    if (disposed || dragging) { cancel(); return; }
    const next = (held || shortcutHeld) && expanded || focused || !waitForDockLeave && inside && (expanded || docked);
    if (next === expanded) { cancel(); return; }
    if (timer !== undefined && pending === next) return;
    cancel();
    pending = next;
    timer = clock.schedule(() => { timer = undefined; pending = undefined; apply(next); }, next ? 180 : 500);
  };
  return {
    expandShortcut() {
      if (disposed || dragging) return;
      cancel(); shortcutHeld = true; waitForDockLeave = false;
      apply(true);
    },
    toggleShortcut() {
      if (disposed || dragging) return;
      cancel();
      shortcutHeld = !expanded;
      focused = false;
      waitForDockLeave = expanded;
      apply(!expanded);
    },
    hold(value: boolean) { held = value; schedule(); },
    setDocked(value: boolean) { docked = value; if (!value) waitForDockLeave = false; schedule(); },
    settleDock() { cancel(); dragging = false; focused = false; waitForDockLeave = true; schedule(); },
    awaitingDockLeave() { return waitForDockLeave; },
    activate() { if (!disposed && !dragging) { inside = true; cancel(); apply(true); } },
    enter() { inside = true; schedule(); },
    leave() { inside = false; schedule(); },
    focus(value: boolean) { focused = value && !dragging; if (focused) { cancel(); apply(true); } else schedule(); },
    drag(value: boolean) { dragging = value; if (value) focused = false; schedule(); },
    isDragging() { return dragging; },
    observePointer(pointerInside: boolean, primaryDown: boolean) {
      inside = pointerInside;
      if (!pointerInside && !primaryDown) waitForDockLeave = false;
      if (dragging && !primaryDown) dragging = false;
      if (primaryDown && inside) focused = false;
      schedule();
    },
    accept(value: boolean) { expanded = value; schedule(); },
    dismiss() {
      cancel(); inside = false; focused = false; held = false; shortcutHeld = false; dragging = false;
      waitForDockLeave = true;
      const changed = expanded;
      apply(false);
      return changed;
    },
    reset() { cancel(); inside = false; focused = false; held = false; shortcutHeld = false; dragging = false; waitForDockLeave = false; apply(false); },
    dispose() { disposed = true; cancel(); },
  };
}

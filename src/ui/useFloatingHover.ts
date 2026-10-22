import { useEffect, useRef, useState, type PointerEvent } from 'react';
import { currentMonitor, getCurrentWindow, LogicalSize, PhysicalPosition } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { createLayoutRevision } from '../domain/floatingLayout.ts';
import { clampFloatingPosition, createFloatingHover, createFloatingPress, createFloatingDockDrag, floatingWindowSize, floatingDockPosition, type FloatingDragEnd, type DockSide } from '../domain/floating.ts';

const native = '__TAURI_INTERNALS__' in window;

export function useFloatingHover(edgeHide: boolean, overview = false, sourceCount = overview ? 3 : 1, editing = false, calendar = false, metricCount = 3) {
  const [expanded, setExpanded] = useState(false);
  const [hiddenEdge, setHiddenEdge] = useState<DockSide | null>(null);
  const [resizeError, setResizeError] = useState<string | null>(null);
  const handlers = useRef<ReturnType<typeof createFloatingHover> | null>(null);
  const reset = useRef(() => {});
  const collapse = useRef(() => {});
  const observe = useRef(async () => {});
  const keyboard = useRef(false);
  const edgeEnabled = useRef(edgeHide);
  const overviewEnabled = useRef(overview);
  const rows = useRef(sourceCount);
  const editMode = useRef(editing);
  const calendarVisible = useRef(calendar);
  const metrics = useRef(metricCount);
  const reconcile = useRef(() => {});
  const beginDrag = useRef(async () => {});
  const press = useRef(createFloatingPress());
  const nativeDragging = useRef(false);
  const dockDrag = useRef(createFloatingDockDrag());
  useEffect(() => {
    let disposed = false;
    let actual = false;
    let desired = false;
    let offset = { x: 0, y: 0 };
    let dock: DockSide | null = null;
    let hidden: DockSide | null = null;
    let restCheck = false;
    let restTimer: number | undefined;
    const cancelRest = () => { window.clearTimeout(restTimer); restTimer = undefined; };
    // Windows can apply its minimum creation width even to a frameless window.
    // Reconcile the real native size before processing the first hover transition.
    let queue = native ? getCurrentWindow().setSize(new LogicalSize(56, 56)).catch(() => {
      if (!disposed) setResizeError('窗口大小初始化失败，请重启 TokenDaily');
    }) : Promise.resolve();
    let forceCollapse = false;
    const layoutRevision = createLayoutRevision();
    const layout = (next: boolean, forced = false, reconcileSize = false) => {
      queue = queue.then(async () => {
        if (disposed || hover.isDragging() || desired !== next || actual === next && !reconcileSize) return;
        const revision = layoutRevision.current();
        try {
          if (native) {
            const win = getCurrentWindow();
            const [position, outer, inner, scale, monitor] = await Promise.all([
              win.outerPosition(), win.outerSize(), win.innerSize(), win.scaleFactor(), currentMonitor(),
            ]);
            if (disposed || hover.isDragging() || desired !== next) return;
            const area = monitor ? { ...monitor.workArea.position, ...monitor.workArea.size } : null;
            const nextDock = edgeEnabled.current ? dock : null;
            const nextHidden = !next ? nextDock : null;
            if (!next && !forced && (actual || nextHidden && !hidden) && await win.isVisible()) {
              // Resizing a native WebView can emit a synthetic mouseleave. Only
              // collapse when the cursor really left the expanded desktop bounds.
              const pointer = await invoke<{ inside: boolean }>('get_floating_pointer_state');
              if (pointer.inside) {
                if (actual) { desired = true; hover.enter(); hover.accept(true); }
                return;
              }
            }
            const size = floatingWindowSize(next, Boolean(nextHidden), overviewEnabled.current, rows.current, editMode.current, calendarVisible.current, metrics.current);
            const outerSize = {
              width: Math.round(size.width * scale) + outer.width - inner.width,
              height: Math.round(size.height * scale) + outer.height - inner.height,
            };
            const anchor = actual && !next ? { x: position.x + offset.x, y: position.y + offset.y } : position;
            // Keep the entire handle inside its work area: it must not spill onto
            // a neighbouring monitor or cover the taskbar. Disabling restores a ball.
            const positionSide = nextDock ?? (hidden ? dock : null);
            const target = area ? positionSide ? floatingDockPosition(positionSide, anchor, outerSize, area)
              : clampFloatingPosition(anchor, outerSize, area) : anchor;
            // Restore the ball's anchor even when expansion shifted inward at a screen edge.
            const nextOffset = next && !actual ? { x: nextDock ? 0 : position.x - target.x, y: position.y - target.y }
              : next ? offset : { x: 0, y: 0 };
            if (disposed || hover.isDragging() || desired !== next) return;
            await win.setSize(new LogicalSize(size.width, size.height));
            try { await win.setPosition(new PhysicalPosition(target.x, target.y)); }
            catch (error) { await win.setSize(inner); throw error; }
            offset = nextOffset;
            dock = nextDock;
            hidden = nextHidden;
            hover.setDocked(Boolean(dock));
            restCheck = false;
          }
          actual = next;
          layoutRevision.applied(revision);
          hover.accept(next);
          if (!disposed) { setExpanded(next); setHiddenEdge(hidden); setResizeError(null); }
        } catch {
          hover.accept(actual);
          if (!disposed) setResizeError('窗口切换失败，请移开鼠标后重试');
        }
      });
    };
    const hover = createFloatingHover(next => {
      desired = next;
      const forced = forceCollapse;
      forceCollapse = false;
      cancelRest();
      layout(next, forced);
    }, { schedule: (run, delay) => window.setTimeout(run, delay), cancel: id => window.clearTimeout(id) });
    reconcile.current = () => {
      layoutRevision.request();
      cancelRest();
      dockDrag.current.cancel();
      restCheck = false;
      layout(desired, !edgeEnabled.current, true);
    };
    beginDrag.current = async () => {
      cancelRest();
      hover.setDocked(false);
      // Restore the full ball before dragging a half-logo away from the edge.
      // Keep this behind any pending resize and keep its hit area on-screen.
      if (native && hidden) {
        await queue;
        if (disposed) return;
        const win = getCurrentWindow();
        const [position, scale, monitor] = await Promise.all([win.outerPosition(), win.scaleFactor(), currentMonitor()]);
        const size = { width: Math.round(56 * scale), height: Math.round(56 * scale) };
        const target = monitor ? clampFloatingPosition(position, size, { ...monitor.workArea.position, ...monitor.workArea.size }) : position;
        await win.setSize(new LogicalSize(56, 56));
        await win.setPosition(new PhysicalPosition(target.x, target.y));
        hidden = null;
        if (!disposed) setHiddenEdge(null);
      }
      dock = null;
      offset = { x: 0, y: 0 };
      dockDrag.current.begin();
      restCheck = false;
    };
    handlers.current = hover;
    reset.current = () => { cancelRest(); dockDrag.current.cancel(); forceCollapse = true; hover.reset(); };
    collapse.current = () => {
      cancelRest(); restCheck = false;
      press.current.cancel(); dockDrag.current.cancel(); nativeDragging.current = false;
      forceCollapse = true;
      if (!hover.dismiss()) {
        // A failed native resize may have left the controller already collapsed.
        // Explicit user intent still reconciles the real window and bypasses hover vetoes.
        forceCollapse = false; desired = false; layout(false, true, true);
      }
    };
    const unlisten = native ? listen<boolean>('floating-updated', () => reset.current()) : null;
    const shortcut = native ? listen('floating-shortcut-toggle', () => {
      if (disposed || hover.isDragging()) return;
      forceCollapse = true;
      cancelRest();
      hover.toggleShortcut();
    }) : null;
    const checkOpen = () => invoke<boolean>('take_floating_open').then(requested => {
      if (!requested || disposed || hover.isDragging()) return;
      cancelRest(); hover.expandShortcut();
    });
    const openRequest = native ? listen('floating-open', () => { void checkOpen(); }) : null;
    void openRequest?.then(checkOpen);
    // Clip the logo during the native move loop. Keep the native drag surface
    // stable until release; shrinking it mid-drag changes Windows' grab offset.
    const dragSnap = native ? listen<DockSide | null>('floating-drag-snap', event => {
      if (disposed || !nativeDragging.current || actual) return;
      hidden = dockDrag.current.preview(event.payload, edgeEnabled.current);
      setHiddenEdge(hidden);
      hover.setDocked(Boolean(hidden));
    }) : null;
    // Capture intent from the native end-of-drag snapshot, not a later cursor
    // poll: users may move the pointer away immediately after releasing it.
    const dragEnded = native ? listen<FloatingDragEnd>('floating-drag-ended', event => {
      if (disposed) return;
      dock = dockDrag.current.finish(event.payload, edgeEnabled.current);
      nativeDragging.current = false;
      if (hidden) {
        hidden = dock;
        setHiddenEdge(hidden);
        hover.setDocked(Boolean(dock));
        if (dock) hover.settleDock(); else hover.drag(false);
        restCheck = false;
        layout(false, true, true);
      } else {
        restCheck = Boolean(dock);
        hover.drag(false);
      }
      void observe.current();
    }) : null;
    let observing = false;
    const syncPointer = async () => {
      if (!native || disposed || observing || !(actual || hover.isDragging() || restCheck || hover.awaitingDockLeave())) return;
      observing = true;
      try {
        const pointer = await invoke<{ visible: boolean; inside: boolean; primaryDown: boolean }>('get_floating_pointer_state');
        if (!disposed) {
          if (pointer.visible) {
            hover.observePointer(pointer.inside, pointer.primaryDown);
            if (layoutRevision.pending() && !hover.isDragging() && !pointer.primaryDown) layout(desired, false, true);
            if (!pointer.primaryDown) nativeDragging.current = false;
            if (restCheck && !actual && !hidden && !hover.isDragging() && !pointer.inside && !pointer.primaryDown) {
              if (restTimer === undefined) restTimer = window.setTimeout(() => {
                restTimer = undefined; layout(false, false, true);
              }, 500);
            } else cancelRest();
          } else { cancelRest(); restCheck = false; reset.current(); }
        }
      } catch { if (!disposed) hover.drag(false); }
      finally { observing = false; }
    };
    observe.current = syncPointer;
    // Native drag loops can swallow mouseleave/mouseup. Observe only while open
    // or dragging / waiting to dock. Hidden handles and settled balls do not poll.
    const pointerTimer = native ? window.setInterval(() => void syncPointer(), 150) : undefined;
    const keydown = (event: KeyboardEvent) => { if (event.key === 'Tab') keyboard.current = true; };
    const pointerdown = () => { keyboard.current = false; hover.focus(false); };
    const blur = () => { hover.focus(false); hover.leave(); };
    window.addEventListener('blur', blur);
    window.addEventListener('keydown', keydown, true);
    window.addEventListener('pointerdown', pointerdown, true);
    return () => {
      disposed = true;
      hover.dispose();
      cancelRest();
      reconcile.current = () => {};
      beginDrag.current = async () => {};
      press.current.cancel();
      dockDrag.current.cancel();
      handlers.current = null;
      reset.current = () => {};
      collapse.current = () => {};
      observe.current = async () => {};
      window.clearInterval(pointerTimer);
      window.removeEventListener('blur', blur);
      window.removeEventListener('keydown', keydown, true);
      window.removeEventListener('pointerdown', pointerdown, true);
      void unlisten?.then(stop => stop());
      void shortcut?.then(stop => stop());
      void openRequest?.then(stop => stop());
      void dragEnded?.then(stop => stop());
      void dragSnap?.then(stop => stop());
    };
  }, []);

  useEffect(() => {
    edgeEnabled.current = edgeHide;
    reconcile.current();
  }, [edgeHide]);

  useEffect(() => {
    overviewEnabled.current = overview;
    rows.current = sourceCount;
    editMode.current = editing;
    calendarVisible.current = calendar;
    metrics.current = metricCount;
    handlers.current?.hold(editing);
    reconcile.current();
  }, [overview, sourceCount, editing, calendar, metricCount]);

  async function drag(event: PointerEvent) {
    if (event.type === 'pointerdown' && event.button !== 0 || !native) return;
    event.preventDefault();
    keyboard.current = false;
    handlers.current?.drag(true);
    nativeDragging.current = true;
    try { await beginDrag.current(); await invoke('start_floating_drag'); }
    catch { dockDrag.current.cancel(); nativeDragging.current = false; handlers.current?.drag(false); setResizeError('暂时无法拖动窗口，请重试'); }
    finally { void observe.current(); }
  }

  const ballPointer = {
    onPointerDown(event: PointerEvent<HTMLButtonElement>) {
      if (event.button !== 0 || !event.isPrimary) return;
      event.preventDefault();
      keyboard.current = false;
      press.current.down(event.pointerId, { x: event.screenX, y: event.screenY });
      handlers.current?.drag(true); // Holding the half-logo must also suspend hover.
      event.currentTarget.setPointerCapture(event.pointerId);
    },
    onPointerMove(event: PointerEvent<HTMLButtonElement>) {
      if (!press.current.move(event.pointerId, { x: event.screenX, y: event.screenY })) return;
      // The native loop owns pointer input from here, so ignore its later click.
      if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
      if (native) void drag(event);
      else handlers.current?.drag(false);
    },
    onPointerUp(event: PointerEvent<HTMLButtonElement>) {
      if (event.button !== 0) return;
      const activate = press.current.up(event.pointerId, { x: event.screenX, y: event.screenY });
      if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
      if (!nativeDragging.current) handlers.current?.drag(false);
      if (activate) handlers.current?.activate();
    },
    onPointerCancel() { press.current.cancel(); if (!nativeDragging.current) handlers.current?.drag(false); },
    onLostPointerCapture() { press.current.cancel(); if (!nativeDragging.current) handlers.current?.drag(false); },
  };

  return { expanded, hiddenEdge, resizeError, handlers, drag, ballPointer, reset, collapse, keyboard };
}

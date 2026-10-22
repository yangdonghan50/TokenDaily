// Presentation only: never filter the sources used by collection or totals.
export const FLOATING_HIDDEN_KEY = 'tokendaily.floating-hidden.v1';
export const COMPONENT_IDS = ['codex', 'claude', 'workbuddy', 'trae', 'calendar', 'usage', 'cost', 'rank'] as const;
export function normalizeHidden(value: unknown): string[] {
  return Array.isArray(value) ? COMPONENT_IDS.filter(id => value.includes(id)) : [];
}
export function setComponentVisible(hidden: string[], id: string, visible: boolean): string[] {
  return normalizeHidden(visible ? hidden.filter(item => item !== id) : [...hidden, id]);
}
export function visibleComponents<T extends { id: string }>(sources: T[], hidden: string[]): T[] {
  return sources.filter(source => !hidden.includes(source.id));
}
export function mergeVisibleOrder(fullOrder: string[], visibleOrder: string[]): string[] {
  let at = 0;
  return fullOrder.map(id => visibleOrder.includes(id) ? visibleOrder[at++] : id);
}

export function createComponentPress(onLongPress: () => void, clock: {
  schedule: (run: () => void, delay: number) => number; cancel: (id: number) => void;
}) {
  let press: { id: number; x: number; y: number; moved: boolean; held: boolean } | null = null;
  let timer: number | undefined;
  const clear = () => { if (timer !== undefined) clock.cancel(timer); timer = undefined; };
  return {
    down(id: number, x: number, y: number) {
      clear(); press = { id, x, y, moved: false, held: false };
      timer = clock.schedule(() => {
        timer = undefined;
        if (press && !press.moved) { press.held = true; onLongPress(); }
      }, 600);
    },
    move(id: number, x: number, y: number) {
      if (!press || press.id !== id || press.held) return false;
      if (Math.hypot(x - press.x, y - press.y) >= 4) { clear(); press.moved = true; }
      return press.moved;
    },
    up(id: number) {
      if (press?.id !== id) return false;
      const moved = press.moved; clear(); press = null; return moved;
    },
    cancel() { clear(); press = null; },
  };
}

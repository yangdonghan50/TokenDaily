import { useCallback, useEffect, useState } from 'react';
import { FLOATING_HIDDEN_KEY, normalizeHidden, setComponentVisible } from '../domain/floatingComponents.ts';

const changed = 'tokendaily-components-changed';
function readHidden() {
  const value = localStorage.getItem(FLOATING_HIDDEN_KEY);
  try { return normalizeHidden(JSON.parse(value ?? '[]')); } catch { return []; }
}
export function useFloatingComponents(persist = true) {
  const [hidden, setHidden] = useState<string[]>(() => { try { return persist ? readHidden() : []; } catch { return []; } });
  const [error, setError] = useState<string | null>(null);
  useEffect(() => {
    if (!persist) return;
    const read = () => { try { setHidden(readHidden()); setError(null); } catch { setError('组件偏好读取失败，请重试'); } };
    const storage = (event: StorageEvent) => { if (event.key === FLOATING_HIDDEN_KEY || event.key === null) read(); };
    window.addEventListener('storage', storage); window.addEventListener(changed, read);
    return () => { window.removeEventListener('storage', storage); window.removeEventListener(changed, read); };
  }, [persist]);
  const setVisible = useCallback((id: string, visible: boolean) => {
    try {
      const next = setComponentVisible(persist ? readHidden() : hidden, id, visible);
      if (persist) localStorage.setItem(FLOATING_HIDDEN_KEY, JSON.stringify(next));
      setHidden(next); setError(null);
      if (persist) window.dispatchEvent(new Event(changed));
      return true;
    } catch { setError('组件偏好未保存，请重试'); return false; }
  }, [persist, hidden]);
  return { hidden, error, setVisible };
}

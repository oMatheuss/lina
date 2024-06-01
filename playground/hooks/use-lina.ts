import init, { Terminal as LinaTerm } from 'lina-wasm';
import { useEffect, useRef } from 'react';

export function useLina() {
  const linaRef = useRef<LinaTerm | null>(null);

  useEffect(() => {
    const abort = new AbortController();
    if (!linaRef.current) {
      (async () => {
        await init();
        if (!abort.signal.aborted) {
          linaRef.current = new LinaTerm();
        }
      })();
    }

    return () => {
      if (linaRef.current) {
        linaRef.current.free();
        linaRef.current = null;
      } else {
        abort.abort();
      }
    };
  }, []);

  return linaRef;
}

'use client';

import { TerminalSquareIcon } from 'lucide-react';
import { useEffect, useRef } from 'react';

export function Terminal() {
  const textarea = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (!textarea.current) return;
    const terminal = textarea.current;
    window.terminal_write = (str) => {
      terminal.value += str;
    };
    window.terminal_clear = () => {
      terminal.value = '';
    };

    return () => {
      window.terminal_write = () => {};
      window.terminal_clear = () => {};
    };
  }, []);

  return (
    <div className="flex flex-col items-center justify-center bg-slate-600 px-4 pb-4 rounded-sm">
      <header className="w-full py-4">
        <TerminalSquareIcon className="inline mr-2 align-middle" />
        <h3 className="inline align-middle font-medium">TERMINAL</h3>
      </header>
      <textarea
        className="w-full h-full bg-slate-950 text-slate-200 resize-none focus:outline-none px-3 py-2 font-mono"
        autoComplete="off"
        spellCheck="false"
        autoCapitalize="none"
        ref={textarea}
      ></textarea>
    </div>
  );
}

'use client';

import { TerminalSquareIcon } from 'lucide-react';
import { useEffect, useRef } from 'react';

interface TerminalProps {
  value: string;
  onChange: (value: string) => void;
}

export function Terminal({ value, onChange }: TerminalProps) {
  useEffect(() => {
    window.terminal_write = (str) => {
      onChange(str);
    };
    window.terminal_clear = () => {
      onChange('');
    };

    return () => {
      window.terminal_write = () => {};
      window.terminal_clear = () => {};
    };
  }, []);

  return (
    <div className="flex flex-col bg-slate-600 rounded-md border-slate-600 border-4">
      <header className="flex w-full py-2">
        <TerminalSquareIcon className="inline mr-2 align-middle" />
        <h3 className="inline align-middle font-medium">TERMINAL</h3>
      </header>
      <textarea
        className="w-full h-full bg-[#1e1e1e] text-slate-200 resize-none focus:outline-none px-3 py-2 font-mono"
        autoComplete="off"
        spellCheck="false"
        autoCapitalize="none"
        onChange={(e) => onChange(e.target.value)}
        value={value}
      ></textarea>
    </div>
  );
}

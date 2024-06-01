'use client';

import { TerminalSquareIcon } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';

interface TerminalProps {
  value: string;
  onChange: (value: string) => void;
  onPrompt: (value: string) => void;
  enablePrompt: boolean;
}

export function Terminal({
  value,
  onChange,
  onPrompt,
  enablePrompt,
}: TerminalProps) {
  const [prompt, setPrompt] = useState('');
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    window.terminal_write = (str) => {
      if (!textareaRef.current) return;
      const textarea = textareaRef.current;
      textarea.value += str;
      onChange(textarea.value);
    };
    window.terminal_clear = () => onChange('');

    return () => {
      window.terminal_write = () => {};
      window.terminal_clear = () => {};
    };
  }, []);

  const handlePrompt = () => {
    onPrompt(prompt + '\n');
    setPrompt('');
  };

  return (
    <div className="min-h-[500px] flex flex-col bg-slate-600 rounded-md border-slate-600 border-4">
      <header className="flex w-full p-2 rounded-t-md bg-[#1e1e1e] mb-1">
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
        readOnly
        ref={textareaRef}
      ></textarea>
      <div className="flex mt-1">
        <input
          className="w-full mr-1 rounded-bl-md bg-[#1e1e1e] text-slate-200 focus:outline-none px-3 py-2 font-mono"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
        />
        <button
          type="button"
          className="px-2 rounded-br-md bg-[#1e1e1e]"
          onClick={handlePrompt}
          disabled={!enablePrompt}
        >
          Enviar
        </button>
      </div>
    </div>
  );
}

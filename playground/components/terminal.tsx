'use client';

import { CornerDownLeftIcon, TerminalSquareIcon } from 'lucide-react';
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
  const inputRef = useRef<HTMLInputElement>(null);

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

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handlePrompt();
    }
  };

  useEffect(() => {
    if (enablePrompt) inputRef.current?.focus();
  }, [enablePrompt]);

  return (
    <div className="flex min-h-[500px] flex-col rounded-md border-4 border-slate-600 bg-slate-600">
      <header className="mb-1 flex w-full rounded-t-md bg-[#1e1e1e] p-2">
        <TerminalSquareIcon className="mr-2 inline align-middle" />
        <h3 className="inline align-middle font-medium">TERMINAL</h3>
      </header>
      <textarea
        className="h-full w-full resize-none bg-[#1e1e1e] px-3 py-2 font-mono text-slate-200 focus:outline-none"
        autoComplete="off"
        spellCheck="false"
        autoCapitalize="none"
        onChange={(e) => onChange(e.target.value)}
        value={value}
        readOnly
        ref={textareaRef}
      ></textarea>
      <div className="relative mt-1 flex justify-end">
        <input
          ref={inputRef}
          className="w-full rounded-b-md bg-[#1e1e1e] py-2 pl-3 pr-10 font-mono text-slate-200 focus:outline-none disabled:cursor-not-allowed"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={!enablePrompt}
        />
        <button
          type="button"
          className="absolute m-2 cursor-default rounded bg-indigo-500 p-1 disabled:bg-indigo-400"
          tabIndex={-1}
          onClick={handlePrompt}
          disabled={!enablePrompt}
          title="Enter"
          aria-label="Enter prompt"
        >
          <CornerDownLeftIcon className="size-4" />
        </button>
      </div>
    </div>
  );
}

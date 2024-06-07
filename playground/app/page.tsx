'use client';

import { Editor } from '@/components/editor';
import { Terminal } from '@/components/terminal';
import { Binary, Eraser, PlayIcon } from 'lucide-react';
import type { editor } from 'monaco-editor';
import { useRef, useState } from 'react';
import Image from 'next/image';
import { useLina } from '@/hooks/use-lina';

export default function Home() {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);
  const linaRef = useLina();
  const [promptEnabled, enablePrompt] = useState(false);

  const resume = (result: string | null = null) => {
    if (!linaRef.current) return;
    const lina = linaRef.current;
    result ??= lina.resume(1000);
    window.requestAnimationFrame(function compilation(time) {
      if (result == 'executing' || result == 'will-write') {
        result = lina.resume(1000); // execute 1000 instructions max
        window.requestAnimationFrame(compilation);
      } else if (result == 'will-read') {
        enablePrompt(true);
      }
    });
  };

  const handleCompile = () => {
    const editor = editorRef.current;
    const lina = linaRef.current;
    if (editor === null || lina === null) return;
    const value = editor.getModel()?.getValue();
    if (!value) return;

    let result = lina.start(value);
    resume(result);
  };

  const handleDecompile = () => {
    const editor = editorRef.current;
    const terminal = linaRef.current;
    if (editor === null || terminal == null) return;
    const value = editor.getModel()?.getValue();
    if (!value) return;
  };

  const [terminal, setTerminal] = useState<string>('');

  const handlePrompt = (value: string) => {
    if (!promptEnabled) return;
    linaRef.current?.prompt(value);
    setTerminal((old) => old + value);
    enablePrompt(false);
    resume();
  };

  return (
    <main className="mx-4 flex min-h-dvh flex-col pb-4">
      <div className="flex flex-col gap-4 py-4 sm:flex-row">
        <Image
          className="size-10"
          src="logo.png"
          alt="lina"
          height={192}
          width={192}
        />
        <button
          onClick={handleCompile}
          className="h-10 whitespace-nowrap rounded-md bg-indigo-500 px-2 text-sm hover:bg-indigo-600"
        >
          <PlayIcon className="mr-2 inline align-middle" />
          <span className="inline align-middle">EXECUTAR</span>
        </button>
        <button
          onClick={handleDecompile}
          className="h-10 whitespace-nowrap rounded-md bg-indigo-500 px-2 text-sm hover:bg-indigo-600"
        >
          <Binary className="mr-2 inline align-middle" />
          <span className="inline align-middle">DESCOMPILAR</span>
        </button>
        <button
          onClick={() => setTerminal('')}
          className="h-10 whitespace-nowrap rounded-md bg-indigo-500 px-2 text-sm hover:bg-indigo-600 sm:ml-auto"
        >
          <Eraser className="mr-2 inline align-middle" />
          <span className="inline align-middle">Limpar</span>
        </button>
      </div>
      <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
        <Editor
          editorRef={editorRef}
          className="h-[calc(100dvh-5.5rem)] min-h-[500px] rounded-md border-4 border-slate-600"
        />
        <Terminal
          value={terminal}
          onChange={setTerminal}
          onPrompt={handlePrompt}
          enablePrompt={promptEnabled}
        />
      </div>
    </main>
  );
}

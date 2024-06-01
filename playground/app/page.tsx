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

  const resume = (result: string | null = null) => {
    if (!linaRef.current) return;
    const lina = linaRef.current;
    result ??= lina.resume(1000);
    window.requestAnimationFrame(function compilation(time) {
      if (result == 'executing' || result == 'will-write') {
        result = lina.resume(1000); // execute 1000 instructions max
        window.requestAnimationFrame(compilation);
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
    linaRef.current?.prompt(value);
    setTerminal((old) => old + value);
    resume();
  };

  return (
    <main className="min-h-dvh mx-4 pb-4 flex flex-col">
      <div className="flex flex-col sm:flex-row py-4 gap-4">
        <Image
          className="h-10 w-10"
          src="logo.png"
          alt="lina"
          height={192}
          width={192}
        />
        <button
          onClick={handleCompile}
          className="h-10 rounded-md bg-indigo-500 px-2 text-sm hover:bg-indigo-600 whitespace-nowrap"
        >
          <PlayIcon className="inline mr-2 align-middle" />
          <span className="inline align-middle">EXECUTAR</span>
        </button>
        <button
          onClick={handleDecompile}
          className="h-10 rounded-md bg-indigo-500 px-2 text-sm hover:bg-indigo-600 whitespace-nowrap"
        >
          <Binary className="inline mr-2 align-middle" />
          <span className="inline align-middle">DESCOMPILAR</span>
        </button>
        <button
          onClick={() => setTerminal('')}
          className="h-10 rounded-md bg-indigo-500 px-2 text-sm sm:ml-auto hover:bg-indigo-600 whitespace-nowrap"
        >
          <Eraser className="inline mr-2 align-middle" />
          <span className="inline align-middle">Limpar</span>
        </button>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Editor
          editorRef={editorRef}
          className="min-h-[500px] h-[calc(100dvh-5.5rem)] border-4 border-slate-600 rounded-md"
        />
        <Terminal
          value={terminal}
          onChange={setTerminal}
          onPrompt={handlePrompt}
          enablePrompt={true}
        />
      </div>
    </main>
  );
}

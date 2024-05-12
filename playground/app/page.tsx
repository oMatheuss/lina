'use client';

import { Editor } from '@/components/editor';
import { Terminal } from '@/components/terminal';
import { PlayIcon } from 'lucide-react';
import type { editor } from 'monaco-editor';
import init, { compile } from 'lina-wasm';
import { useRef } from 'react';

export default function Home() {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);

  const handleCompile = () => {
    const editor = editorRef.current;
    if (editor === null) return;
    const value = editor.getModel()?.getValue();
    if (value) init().then(() => compile(value));
  };

  const handleCreate = (editor: editor.IStandaloneCodeEditor) => {
    editorRef.current = editor;
  };

  return (
    <main className="min-h-dvh mx-4 flex flex-col">
      <div className="py-4">
        <button
          onClick={handleCompile}
          className="h-10 rounded-sm bg-indigo-500 px-2 text-sm hover:bg-indigo-600 whitespace-nowrap"
        >
          <PlayIcon className="inline mr-2 align-middle" />
          <span className="inline align-middle">EXECUTAR</span>
        </button>
      </div>
      <div className="grid grid-cols-2 gap-4">
        <Editor
          onCreate={handleCreate}
          className="min-h-[500px] h-[calc(100dvh-6.5rem)] border-4 border-indigo-500"
        />
        <Terminal />
      </div>
    </main>
  );
}

'use client';

import dynamic from 'next/dynamic';
import { JetBrains_Mono } from 'next/font/google';
import type { editor } from 'monaco-editor';

const font = JetBrains_Mono({
  display: 'swap',
  style: 'normal',
  weight: '400',
  fallback: ['system-ui', 'monospace'],
  subsets: ['latin'],
});

interface EditorProps {
  className?: string;
  onCreate?: (editor: editor.IStandaloneCodeEditor) => void;
}

const MonacoEditor = dynamic(() => import('@/components/editor-base'), {
  ssr: false,
});

export function Editor({ className, onCreate }: EditorProps) {
  return (
    <MonacoEditor
      className={className}
      style={font.style}
      fontFamily={font.style.fontFamily}
      onCreate={onCreate}
    />
  );
}

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
  editorRef: React.MutableRefObject<editor.IStandaloneCodeEditor | null>;
}

const MonacoEditor = dynamic(() => import('@/components/editor-base'), {
  ssr: false,
});

export function Editor({ className, editorRef }: EditorProps) {
  return (
    <MonacoEditor
      className={className}
      style={font.style}
      fontFamily={font.style.fontFamily}
      editorRef={editorRef}
    />
  );
}

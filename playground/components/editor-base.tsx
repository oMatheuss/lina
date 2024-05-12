'use client';

import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { AUTOCOMPLETE, CONFIG, GRAMMAR } from '@/lib/lang-def';
import { type CSSProperties, useEffect, useRef } from 'react';

const LANG = 'lina';

const initMonaco = () => {
  monaco.languages.register({ id: LANG });
  monaco.languages.setMonarchTokensProvider(LANG, GRAMMAR);
  monaco.languages.setLanguageConfiguration(LANG, CONFIG);
  monaco.languages.registerCompletionItemProvider(LANG, AUTOCOMPLETE);
};

initMonaco();

interface EditorBaseProps {
  className?: string;
  fontFamily?: string;
  style?: CSSProperties;
  onCreate?: (editor: monaco.editor.IStandaloneCodeEditor) => void;
}

export default function EditorBase(props: EditorBaseProps) {
  const container = useRef<HTMLDivElement>(null);
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);

  useEffect(() => {
    if (!container.current) return;
    const div = container.current;

    const editor = monaco.editor.create(div, {
      theme: 'vscode',
      language: 'lina',
      automaticLayout: true,
      fontFamily: props.fontFamily,
      'semanticHighlighting.enabled': true,
    });

    editorRef.current = editor;
    props.onCreate?.(editor);

    const resize = () => {
      editorRef.current?.layout();
    };

    div.addEventListener('resize', resize);

    return () => {
      div.removeEventListener('resize', resize);
      editor.dispose();
    };
  }, [props]);

  return (
    <div ref={container} className={props.className} style={props.style} />
  );
}

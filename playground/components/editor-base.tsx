'use client';

import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { AUTOCOMPLETE, CONFIG, GRAMMAR, themeVsDarkPlus } from '@/lib/lang-def';
import {
  type CSSProperties,
  useEffect,
  useRef,
  type MutableRefObject,
} from 'react';

const LANG = 'lina';

const initMonaco = () => {
  monaco.languages.register({ id: LANG });
  monaco.languages.setMonarchTokensProvider(LANG, GRAMMAR);
  monaco.languages.setLanguageConfiguration(LANG, CONFIG);
  monaco.languages.registerCompletionItemProvider(LANG, AUTOCOMPLETE);

  monaco.editor.defineTheme('vs-dark-plus', themeVsDarkPlus);
};

initMonaco();

interface EditorBaseProps {
  className?: string;
  fontFamily?: string;
  style?: CSSProperties;
  editorRef: MutableRefObject<monaco.editor.IStandaloneCodeEditor | null>;
}

export default function EditorBase({
  editorRef,
  className,
  fontFamily,
  style,
}: EditorBaseProps) {
  const container = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!container.current) return;
    const div = container.current;

    const editor = monaco.editor.create(div, {
      theme: 'vs-dark-plus',
      language: 'lina',
      automaticLayout: true,
      fontFamily: fontFamily,
      'semanticHighlighting.enabled': true,
      scrollbar: { alwaysConsumeMouseWheel: false },
      autoIndent: 'full',
    });

    editorRef.current = editor;

    const resize = () => {
      editorRef.current?.layout();
    };

    div.addEventListener('resize', resize);

    return () => {
      div.removeEventListener('resize', resize);
      editor.dispose();
    };
  }, [fontFamily]);

  return <div ref={container} className={className} style={style} />;
}

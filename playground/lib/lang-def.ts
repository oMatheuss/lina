import type { languages, editor } from 'monaco-editor';

export const CONFIG: languages.LanguageConfiguration = {
  comments: {
    lineComment: '#',
  },
  brackets: [
    ['{', '}'],
    ['[', ']'],
    ['(', ')'],
  ],
  autoClosingPairs: [
    { open: '[', close: ']' },
    { open: '{', close: '}' },
    { open: '(', close: ')' },
    { open: '"', close: '"', notIn: ['string'] },
  ],
  surroundingPairs: [
    { open: '{', close: '}' },
    { open: '[', close: ']' },
    { open: '(', close: ')' },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
  ],
  folding: {
    markers: {
      start: new RegExp('^\\s*#pragma\\s+region\\b'),
      end: new RegExp('^\\s*#pragma\\s+endregion\\b'),
    },
  },
  indentationRules: {
    // não começa com um comentário
    // tem as palavras senao, entao ou repetir
    // e não termina com fim (o que indicaria um one-liner)
    increaseIndentPattern:
      /^((?!#).)*(\b(senao|entao|repetir)\b)(((?!\bfim\b).)*)(\s*)$/,

    // opcionalmente espaço vazio seguido pela palavra fim
    decreaseIndentPattern: /^\s*\bfim\b/,
  },
};

export const GRAMMAR: languages.IMonarchLanguage = {
  // Set defaultToken to invalid to see what you do not tokenize yet
  // defaultToken: 'invalid',

  keywords: ['programa', 'seja', 'verdadeiro', 'falso'],

  controlFlowKeywords: [
    'para',
    'ate',
    'repetir',
    'incremento',
    'se',
    'entao',
    'senao',
    'enquanto',
    'fim',
  ],

  typeKeywords: ['booleano', 'real', 'inteiro', 'texto'],

  operators: [
    ':=',
    '>',
    '<',
    '=',
    '<=',
    '>=',
    '<>',
    'e',
    'ou',
    '+',
    '-',
    '*',
    '/',
    '&',
    '^',
    '%',
    '+=',
    '-=',
    '*=',
    '/=',
    '^=',
    '%=',
  ],

  // we include these common regular expressions
  symbols: /[=><!~?:&|+\-*\/\^%]+/,

  // C# style strings
  escapes:
    /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

  // The main tokenizer for our languages
  tokenizer: {
    root: [
      // identifiers and keywords
      [
        /[a-z_$][\w$]*/,
        {
          cases: {
            '@typeKeywords': 'type.identifier',
            '@keywords': 'keyword',
            '@controlFlowKeywords': 'keyword.control',
            '@default': 'variable',
          },
        },
      ],
      [/[A-Za-z][\w\$]*/, 'type.identifier'], // to show class names nicely

      // whitespace
      { include: '@whitespace' },

      // delimiters and operators
      [/[{}()\[\]]/, '@brackets'],
      [/[<>](?!@symbols)/, '@brackets'],
      [/@symbols/, { cases: { '@operators': 'operator', '@default': '' } }],

      // @ annotations.
      // As an example, we emit a debugging log message on these tokens.
      // Note: message are supressed during the first load -- change some lines to see them.
      [
        /@\s*[a-zA-Z_\$][\w\$]*/,
        { token: 'annotation', log: 'annotation token: $0' },
      ],

      // numbers
      [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
      [/0[xX][0-9a-fA-F]+/, 'number.hex'],
      [/\d+/, 'number'],

      // delimiter: after number because of .\d floats
      [/[;,.]/, 'delimiter'],

      // strings
      [/"([^"\\]|\\.)*$/, 'string.invalid'], // non-teminated string
      [/"/, { token: 'string.quote', bracket: '@open', next: '@string' }],

      // characters
      [/'[^\\']'/, 'string'],
      [/(')(@escapes)(')/, ['string', 'string.escape', 'string']],
      [/'/, 'string.invalid'],
    ],

    comment: [
      [/[^\/*]+/, 'comment'],
      [/\/\*/, 'comment', '@push'], // nested comment
      ['\\*/', 'comment', '@pop'],
      [/[\/*]/, 'comment'],
    ],

    string: [
      [/[^\\"]+/, 'string'],
      [/@escapes/, 'string.escape'],
      [/\\./, 'string.escape.invalid'],
      [/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }],
    ],

    whitespace: [
      [/[ \t\r\n]+/, 'white'],
      [/\/\*/, 'comment', '@comment'],
      [/\/\/.*$/, 'comment'],
    ],
  },
};

export const themeVsDarkPlus: editor.IStandaloneThemeData = {
  base: 'vs-dark',
  inherit: true,
  colors: {},
  rules: [
    { token: 'keyword.control', foreground: 'C586C0' },
    { token: 'string.escape', foreground: 'D7BA7D' },
    { token: 'keyword.controlFlow', foreground: 'C586C0' },
    { token: 'variable', foreground: '9CDCFE' },
    { token: 'parameter', foreground: '9CDCFE' },
    { token: 'property', foreground: '9CDCFE' },
    { token: 'support.function', foreground: 'DCDCAA' },
    { token: 'function', foreground: 'DCDCAA' },
    { token: 'variable.constant', foreground: '4FC1FF' },
    { token: 'typeParameter', foreground: '4EC9B0' },
  ],
};

export const AUTOCOMPLETE: languages.CompletionItemProvider = {
  provideCompletionItems: (model, position) => {
    const word = model.getWordUntilPosition(position);
    const range = {
      startLineNumber: position.lineNumber,
      endLineNumber: position.lineNumber,
      startColumn: word.startColumn,
      endColumn: word.endColumn,
    };
    const suggestions: languages.CompletionItem[] = [
      {
        label: 'se',
        kind: 27 as languages.CompletionItemKind.Snippet,
        insertText: 'se ${1:condicao} entao\n\t$0\nfim',
        insertTextRules:
          4 as languages.CompletionItemInsertTextRule.InsertAsSnippet,
        documentation: 'Se-Entao',
        range: range,
      },
      {
        label: 'para',
        kind: 27 as languages.CompletionItemKind.Snippet,
        insertText: 'para ${1:var} ate ${2:lim} repetir\n\t$0\nfim',
        insertTextRules:
          4 as languages.CompletionItemInsertTextRule.InsertAsSnippet,
        documentation: 'Para-Repetir',
        range: range,
      },
      {
        label: 'enquanto',
        kind: 27 as languages.CompletionItemKind.Snippet,
        insertText: 'enquanto ${1:expr} repetir\n\t$0\nfim',
        insertTextRules:
          4 as languages.CompletionItemInsertTextRule.InsertAsSnippet,
        documentation: 'Enquanto-Repetir',
        range: range,
      },
    ];

    return { suggestions };
  },
};

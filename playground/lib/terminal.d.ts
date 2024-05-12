declare global {
  interface Window {
    terminal_write: (str: string) => void;
    terminal_clear: () => void;
  }
}

export {};

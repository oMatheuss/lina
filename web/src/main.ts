import './style.css'
import init, { compile } from "web";

const app = document.querySelector<HTMLDivElement>('#app')!;

app.innerHTML = `
  <textarea id="code"></textarea>
  <textarea id="stdio"></textarea>
  <button id="run">run</button>
`
const code = document.querySelector<HTMLTextAreaElement>('#code')!;
const stdio = document.querySelector<HTMLTextAreaElement>('#stdio')!;

declare global {
  interface Window {
    terminal_write: (str: string) => void;
    terminal_clear: () => void;
  }
}

window.terminal_write = (str: string) => {
  stdio.innerHTML += str;
}

window.terminal_clear = () => {
  stdio.innerHTML = "";
}

init().then(() => {
  document.querySelector<HTMLButtonElement>('#run')
    ?.addEventListener("click", () => compile(code.value));
});


use std::collections::VecDeque;
use std::io::{Read, Result, Write};
use std::str;
use wasm_bindgen::prelude::*;

use lina::compiler::compile;
use lina::lexer::{lex, LexicalError};
use lina::parser::{parse, SyntaxError};
use lina::vm::{LinaVm, RuntimeError, VmState};

#[wasm_bindgen]
extern "C" {
    pub fn terminal_write(s: &str);
    pub fn terminal_clear();
}

struct Input(VecDeque<u8>);
struct Output();

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let value = str::from_utf8(buf).unwrap_or_default();
        terminal_write(value);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[wasm_bindgen]
struct Terminal {
    vm: LinaVm<Input, Output>,
}

#[wasm_bindgen]
impl Terminal {
    #[wasm_bindgen(constructor)]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            vm: LinaVm::empty(Input(VecDeque::new()), Output()),
        }
    }

    #[allow(dead_code)]
    pub fn prompt(&mut self, input: &str) {
        _ = self.vm.stdin.0.write(input.as_bytes());
    }

    fn lex_err(&mut self, err: LexicalError) {
        _ = writeln!(self.vm.stdout, "Erro Léxico: {}", err.msg);
        _ = writeln!(self.vm.stdout, "main.lina:{}:{}", err.row, err.col);
    }

    fn syn_err(&mut self, err: SyntaxError) {
        _ = writeln!(self.vm.stdout, "Erro Sintático: {}", err.msg);
        _ = writeln!(self.vm.stdout, "main.lina:{}:{}", err.pos.row, err.pos.col);
    }

    fn run_err(&mut self, err: RuntimeError) {
        _ = writeln!(self.vm.stdout, "Erro: {err}");
    }

    #[allow(dead_code)]
    pub fn start(&mut self, code: &str) -> String {
        self.vm.stdin.0.clear();

        let tkns = match lex(code) {
            Ok(tkns) => tkns,
            Err(err) => {
                self.lex_err(err);
                return Default::default();
            }
        };
        let prgm = match parse(tkns) {
            Ok(prgm) => prgm,
            Err(err) => {
                self.syn_err(err);
                return Default::default();
            }
        };
        let byco = compile(&prgm);
        self.vm.start(byco);

        self.resume(100)
    }

    pub fn resume(&mut self, max: i32) -> String {
        let mut count = 0;
        loop {
            match self.vm.run_single() {
                Ok(s @ VmState::Executing) => {
                    count += 1;
                    if count >= max {
                        break s.to_string();
                    }
                }
                Ok(s @ VmState::WillRead) => {
                    if self.vm.stdin.0.len() > 0 {
                        count += 1;
                    } else {
                        break s.to_string();
                    }
                }
                Ok(s) => break s.to_string(),
                Err(e) => {
                    self.run_err(e);
                    self.vm.reset();
                    break Default::default();
                }
            }
        }
    }
}

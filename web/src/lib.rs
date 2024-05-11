use wasm_bindgen::prelude::*;
use lina::run_code;
use std::fmt::Write;

pub struct Terminal;

impl std::fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        terminal_write(s);
        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn terminal_write(s: &str);
    pub fn terminal_clear();
}

#[wasm_bindgen]
pub fn compile(code: &str) {
    let terminal = &mut Terminal;

    terminal_clear();

    let _ = match run_code(code, terminal) {
        Ok(()) => writeln!(terminal, "compilação & execução finalizados com sucesso"),
        Err(()) => writeln!(terminal, "compilação & execução finalizados com erro"),
    };
}

use lina::run_code;
use std::io::{BufWriter, Write};
use wasm_bindgen::prelude::*;

pub struct Terminal;

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        terminal_write(std::str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
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
    let mut terminal = BufWriter::new(Terminal);

    terminal_clear();

    let _ = match run_code(code, &mut terminal) {
        Ok(()) => writeln!(terminal, "compilação & execução finalizados com sucesso"),
        Err(()) => writeln!(terminal, "compilação & execução finalizados com erro"),
    };
}

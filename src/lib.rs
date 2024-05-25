use std::io::Write;

mod compiler;
mod lexer;
mod parser;
mod syntax;
mod token;
mod vm;

use lexer::LexicalError;
use parser::SyntaxError;
use vm::{LinaVm, RuntimeError};

pub struct LinaExec<'a, T>
where
    T: Write,
{
    pub stdout: T,
    pub source: &'a str,
    pub path: &'a str,
}

impl<'a, T> LinaExec<'a, T>
where
    T: Write,
{
    fn lex_err(&mut self, err: LexicalError) {
        _ = writeln!(self.stdout, "Erro Léxico: {}", err.msg);
        _ = writeln!(self.stdout, "{}:{}:{}", self.path, err.row, err.col);
    }

    fn syn_err(&mut self, err: SyntaxError) {
        _ = writeln!(self.stdout, "Erro Sintático: {}", err.msg);
        _ = writeln!(self.stdout, "{}:{}:{}", self.path, err.pos.row, err.pos.col);
    }

    fn run_err(&mut self, err: RuntimeError) {
        _ = writeln!(self.stdout, "Erro: {err}");
    }

    pub fn run(&mut self) -> Result<(), ()> {
        let tokens = lexer::lex(&self.source).map_err(|err| self.lex_err(err))?;
        let program = parser::parse(tokens).map_err(|err| self.syn_err(err))?;

        let compiler = compiler::compile(&program);
        let mut vm = LinaVm::new(&compiler.bytecode, &compiler.constants);

        vm.run(&mut self.stdout).map_err(|err| self.run_err(err))?;

        Ok(())
    }

    pub fn decompile(&mut self) -> Result<(), ()> {
        let tokens = lexer::lex(&self.source).map_err(|err| self.lex_err(err))?;
        let program = parser::parse(tokens).map_err(|err| self.syn_err(err))?;

        let compiler = compiler::compile(&program);
        let mut vm = LinaVm::new(&compiler.bytecode, &compiler.constants);

        vm.decompile(&mut self.stdout)
            .map_err(|err| self.run_err(err))?;

        Ok(())
    }
}

use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Read};

use lina::compiler::compile;
use lina::lexer::{lex, LexicalError};
use lina::parser::{parse, SyntaxError};
use lina::vm::{LinaVm, RuntimeError};

pub struct LinaExec<'a> {
    pub path: &'a str,
    pub source: &'a str,
}

impl<'a> LinaExec<'a> {
    fn lex_err(&mut self, err: LexicalError) {
        eprintln!("Erro Léxico: {}", err.msg);
        eprintln!("{}:{}:{}", self.path, err.row, err.col);
    }

    fn syn_err(&mut self, err: SyntaxError) {
        eprintln!("Erro Sintático: {}", err.msg);
        eprintln!("{}:{}:{}", self.path, err.pos.row, err.pos.col);
    }

    fn run_err(&mut self, err: RuntimeError) {
        eprintln!("Erro: {err}");
    }

    pub fn run(&mut self) -> Result<(), ()> {
        let tokens = lex(&self.source).map_err(|err| self.lex_err(err))?;
        let program = parse(tokens).map_err(|err| self.syn_err(err))?;
        let bytecode = compile(&program);

        let mut vm = LinaVm::new(bytecode, stdin(), stdout());

        vm.non_stop().map_err(|err| self.run_err(err))
    }

    pub fn decompile(&mut self) -> Result<(), ()> {
        let tokens = lex(&self.source).map_err(|err| self.lex_err(err))?;
        let program = parse(tokens).map_err(|err| self.syn_err(err))?;
        let bytecode = compile(&program);

        let mut vm = LinaVm::new(bytecode, stdin(), stdout());

        vm.decompile().map_err(|err| self.run_err(err))
    }
}

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let file_name = args.get(1).expect("nenhum arquivo especificado");

    let mut file = File::open(file_name)?;

    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut exec = LinaExec {
        path: &file_name,
        source: &code,
    };

    if let Some(arg) = args.get(2) {
        if arg == "-d" {
            _ = exec.decompile();
        }
    } else {
        _ = exec.run();
    }

    Ok(())
}

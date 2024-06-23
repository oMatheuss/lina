use std::env;
use std::fs;
use std::io::{stdin, stdout};

use lina::compiler::compile;
use lina::lexer::lex;
use lina::parser::parse;
use lina::vm::LinaVm;

fn main() -> std::result::Result<(), ()> {
    let args = env::args().collect::<Vec<_>>();
    let file_path = args.get(1).ok_or(()).map_err(|_| {
        eprintln!("Erro: arquivo não fornecido");
    })?;

    let code = fs::read_to_string(file_path).map_err(|err| {
        eprintln!("Erro: não foi possivel ler o arquivo {file_path}: {err}");
    })?;

    let tokens = lex(&code).map_err(|err| {
        eprintln!("Erro Léxico: {}", err.msg);
        eprintln!("\t--> em {}:{}:{}", file_path, err.row, err.col);
    })?;

    let program = parse(tokens).map_err(|err| {
        eprintln!("Erro Sintático: {}", err.msg);
        eprintln!("\t--> em {}:{}:{}", file_path, err.pos.row, err.pos.col);
    })?;

    let mut vm = LinaVm::new(compile(&program), stdin(), stdout());

    if let Some(arg) = args.get(2) {
        if arg == "-d" {
            vm.decompile().map_err(|err| {
                eprintln!("Erro: {err}");
            })?;
        }
    } else {
        vm.non_stop().map_err(|err| {
            eprintln!("Erro: {err}");
        })?;
    }

    Ok(())
}

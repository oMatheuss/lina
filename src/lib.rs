use std::io::Write;

mod compiler;
mod lexer;
mod parser;
mod syntax;
mod token;
mod vm;

use vm::LinaVm;

pub fn run_code(code: &str, stdout: &mut dyn Write) -> Result<(), ()> {
    let tokens = lexer::lex(code).map_err(|err| writeln!(stdout, "{}", err).unwrap())?;

    let program = parser::parse(tokens).map_err(|err| writeln!(stdout, "{}", err).unwrap())?;

    let compiler = compiler::compile(&program);

    let mut vm = LinaVm::new(&compiler.bytecode, &compiler.constants);

    vm.run(stdout)
        .map_err(|err| writeln!(stdout, "{}", err).unwrap())
}

pub fn decompile_code(code: &str, stdout: &mut dyn Write) -> Result<(), ()> {
    let tokens = lexer::lex(code).map_err(|err| writeln!(stdout, "{}", err).unwrap())?;

    let program = parser::parse(tokens).map_err(|err| writeln!(stdout, "{}", err).unwrap())?;

    let compiler = compiler::compile(&program);

    let mut vm = LinaVm::new(&compiler.bytecode, &compiler.constants);

    vm.decompile(stdout)
        .map_err(|err| writeln!(stdout, "{}", err).unwrap())
}

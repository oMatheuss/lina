use std::env;
use std::fs::File;
use std::io::Read;

mod token;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::{Environment, interpret_code};

fn main() {
    let mut code = String::new();

    let args = env::args().collect::<Vec<_>>();

    let mut file = File::open(args.get(1).unwrap()).expect("Arquivo não encontrado");

    file.read_to_string(&mut code)
        .expect("Erro ao ler o arquivo");

    let mut lexer = Lexer::new(&code);
    let mut environment = Environment::new();

    interpret_code(&mut lexer, &mut environment);
}

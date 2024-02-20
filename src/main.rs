use std::env;
use std::fs::File;
use std::io::Read;

mod error;
mod lexer;
mod operator;
mod parser;
mod token;
mod value;

use lexer::Lexer;
use parser::{interpret_code, Environment};

fn main() {
    let mut code = String::new();

    let args = env::args().collect::<Vec<_>>();
    let Some(file_name) = args.get(1) else {
        panic!("file does not exist")
    };

    let mut file = File::open(file_name).expect("Arquivo não encontrado");

    file.read_to_string(&mut code)
        .expect("Erro ao ler o arquivo");

    let mut lexer = Lexer::new(file_name.to_string(), &code);
    let mut env = Environment::new();

    match interpret_code(&mut lexer, &mut env) {
        Ok(..) => {}
        Err(err) => eprint!("{err}"),
    }
}

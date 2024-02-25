mod ast;
mod error;
mod lexer;
mod operator;
mod parser;
mod token;
mod value;

use lexer::Lexer;
use parser::{interpret_code, Environment};

pub fn run_code(file_name: String, code: &str) {
    let mut lexer = Lexer::new(file_name, &code);
    let mut env = Environment::new();

    match interpret_code(&mut lexer, &mut env) {
        Ok(..) => {}
        Err(err) => eprint!("{err}"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        fs::{self, File},
        io::Cursor,
        io::Read,
    };

    #[test]
    fn run_examples() -> Result<(), String> {
        let paths = fs::read_dir("./examples").unwrap();

        for path in paths {
            let file_path = path.unwrap().path();
            let file_str = file_path.to_str().unwrap();
            let mut file = File::open(file_str).expect("Arquivo não encontrado");
            let mut code = String::new();
            file.read_to_string(&mut code)
                .expect("Erro ao ler o arquivo");

            let mut lexer = Lexer::new(String::from(file_str), &code);
            let mut env = Environment::new();
            let output = Cursor::new(Vec::new());

            env.output = Box::new(output);

            interpret_code(&mut lexer, &mut env)
                .map_err(|err| eprintln!("{}", err))
                .expect("Código ser executado normalmente");
        }

        Ok(())
    }
}

mod lexer;
mod token;
mod interpreter;

use lexer::Lexer;

pub fn run_code(file_name: String, code: &str) {

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

            let mut lexer = Lexer::new(&code);

            todo!()
        }

        Ok(())
    }
}

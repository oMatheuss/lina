mod lexer;
mod token;
mod interpreter;

use lexer::Lexer;

pub fn run_code(file_name: String, code: &str) {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::token::{Operador, Token, Valor};

    use self::interpreter::Scope;

    use super::*;
    use std::{
        fs::{self, File},
        io::Cursor,
        io::Read,
    };

    #[test]
    fn token_are_correctly_readed() {
        let input = "\
        seja foo = 1 \
        enquanto foo < 10 \
            foo += 1 \
            imprima foo \
        fim";

        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()
            .unwrap()
            .into_iter()
            .map(|def| def.kind)
            .collect::<Vec<Token<'_>>>();
        let arr_tokens = tokens.as_slice();

        assert_eq!(
            arr_tokens,
            [
                Token::Seja,
                Token::Identificador("foo"),
                Token::Operador(Operador::Atribuicao),
                Token::Valor(Valor::Numero(1f32)),

                Token::Enquanto,
                Token::Identificador("foo"),
                Token::Operador(Operador::MenorQue),
                Token::Valor(Valor::Numero(10f32)),

                Token::Identificador("foo"),
                Token::Operador(Operador::SomaAtribuicao),
                Token::Valor(Valor::Numero(1f32)),

                Token::Imprima,
                Token::Identificador("foo"),

                Token::Fim
            ]
        );
    }

    #[test]
    fn interpreter_runs_fine() {
        let input = " \
        seja b = 1
        enquanto b < 10 \
            b += 1 \
            imprima b \
        fim";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut global = Scope::new(None);
        interpreter::execute(tokens, &mut global);
    }

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

mod lexer;
mod token;
mod parser;
mod syntax;

pub fn run_code(file_name: String, code: &str) -> Result<(), ()> {
    let tokens = lexer::lex(code).map_err(|err| {
        eprintln!("{}", err);
    })?;

    println!("file: {}", file_name);
    println!("tokens: {:#?}", tokens);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{lexer::{self, Lexer}, parser, token::{Literal, Operador, Token}};

    #[test]
    fn ast_is_correctly_generated() {
        let input = "seja a = 10 + 10";
        let tokens = lexer::lex(input).expect("semantica correta");
        let ast = parser::parse(tokens).expect("sintaxe correta");

        println!("{:#?}", ast);
    }

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
                Token::Literal(Literal::Numero(1f32)),

                Token::Enquanto,
                Token::Identificador("foo"),
                Token::Operador(Operador::MenorQue),
                Token::Literal(Literal::Numero(10f32)),

                Token::Identificador("foo"),
                Token::Operador(Operador::SomaAtribuicao),
                Token::Literal(Literal::Numero(1f32)),

                Token::Imprima,
                Token::Identificador("foo"),

                Token::Fim
            ]
        );
    }

}

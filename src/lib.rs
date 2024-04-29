mod compiler;
mod lexer;
mod parser;
mod syntax;
mod token;
mod vm;

pub fn run_code(file_name: String, code: &str) -> Result<(), ()> {
    let tokens = lexer::lex(code).map_err(|err| {
        eprintln!("{}", err);
    })?;

    let program = parser::parse(tokens).map_err(|err| {
        eprintln!("{}", err);
    })?;

    compiler::execute_program(program);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{lexer, parser};

    #[test]
    fn ast_is_correctly_generated() {
        let input = r#"
        seja soma := 0 + 1 * 4 * 2 ^ 3 / 5
        seja teste := soma *= soma /= soma -= soma
        enquanto soma < 100 repetir
            soma := a + entrada / 2
            saida := soma
            
            se soma = 35 ou soma > 30 e soma < 50 entao
                soma += 20
            fim
        fim"#;
        let tokens = lexer::lex(input).expect("semantica correta");
        println!("{:#?}", tokens);

        let ast = parser::parse(tokens).expect("sintaxe correta");

        println!("{}", ast);
    }
}

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum Token {
    Seja,
    Faca,
    Entao,
    Imprima,
    Enquanto,
    Se,
    Funcao,
    Para,
    Retorne,
    Identificador(String),
    Numero(i32),
    Operador(String),
    Incremento,
    AtribuicaoIncremento,
    Decremento,
    AtribuicaoDecremento,
    Fim,
}

struct Lexer<'a> {
    code: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self { code, pos: 0 }
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.code.chars().nth(self.pos) {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.pos < self.code.len()
            && self.code.chars().nth(self.pos).unwrap().is_alphanumeric()
        {
            identifier.push(self.code.chars().nth(self.pos).unwrap());
            self.pos += 1;
        }
        identifier
    }

    pub fn next(&mut self) -> Option<Token> {
        self.consume_whitespace();
        if self.pos >= self.code.len() || None == self.code.chars().nth(self.pos) {
            return None;
        }

        let Some(c) = self.code.chars().nth(self.pos) else {
            unreachable!()
        };

        match c {
            '0'..='9' => {
                // match for number literal
                let mut num_str = String::new();
                while self.pos < self.code.len()
                    && self.code.chars().nth(self.pos).unwrap().is_numeric()
                {
                    num_str.push(self.code.chars().nth(self.pos).unwrap());
                    self.pos += 1;
                }
                Some(Token::Numero(num_str.parse().unwrap_or(0)))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' => {
                if let Some(next_c) = self.code.chars().nth(self.pos + 1) {
                    if (c == '+' || c == '-') && next_c == c {
                        // match for ++ and --
                        self.pos += 2;
                        match c {
                            '+' => Some(Token::Incremento),
                            '-' => Some(Token::Decremento),
                            _ => unreachable!(),
                        }
                    } else if next_c == '=' {
                        // match for += and -=
                        self.pos += 2;
                        match c {
                            '+' => Some(Token::AtribuicaoIncremento),
                            '-' => Some(Token::AtribuicaoDecremento),
                            _ => todo!(),
                        }
                    } else {
                        // match for any given operator +, -, *, ...
                        self.pos += 1;
                        Some(Token::Operador(c.to_string()))
                    }
                } else {
                    // match for any given operator +, -, *, ...
                    self.pos += 1;
                    Some(Token::Operador(c.to_string()))
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.consume_identifier();
                match identifier.as_str() {
                    "seja" => Some(Token::Seja),
                    "faça" => Some(Token::Faca),
                    "então" => Some(Token::Entao),
                    "imprima" => Some(Token::Imprima),
                    "enquanto" => Some(Token::Enquanto),
                    "se" => Some(Token::Se),
                    "função" => Some(Token::Funcao),
                    "para" => Some(Token::Para),
                    "retorne" => Some(Token::Retorne),
                    "fim" => Some(Token::Fim),
                    _ => Some(Token::Identificador(identifier)),
                }
            }
            _ => panic!("Token Inesperado"),
        }
    }
}

struct Environment {
    variables: HashMap<String, i32>,
}

impl Environment {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

fn interpret_code(lexer: &mut Lexer, environment: &mut Environment) -> Option<()> {
    loop {
        match lexer.next()? {
            Token::Seja => {
                let Token::Identificador(var_name) = lexer.next()? else {
                    panic!("Erro: Esperava-se um identificador após 'seja'");
                };

                let Token::Numero(var_value) = lexer.next()? else {
                    panic!("Erro: Esperava-se um valor após o identificador da declaração 'seja'");
                };

                environment.variables.insert(var_name, var_value);
            }
            Token::Identificador(name) => match lexer.next()? {
                Token::Incremento => {
                    let value = environment
                        .variables
                        .get_mut(&name)
                        .unwrap_or_else(|| panic!("Variável não definida: {}", name));
                    *value += 1;
                }
                Token::Decremento => {
                    let value = environment
                        .variables
                        .get_mut(&name)
                        .unwrap_or_else(|| panic!("Variável não definida: {}", name));
                    *value -= 1;
                }
                Token::AtribuicaoIncremento => match lexer.next()? {
                    Token::Identificador(var2) => {
                        let target = if let Some(target) = environment.variables.get(&var2) {
                            *target
                        } else {
                            panic!("Erro: Variavel inexperada '{name}'")
                        };

                        let value = environment
                            .variables
                            .get_mut(&name)
                            .unwrap_or_else(|| panic!("Variável não definida: {}", name));

                        *value += target;
                    }
                    Token::Numero(target) => {
                        let value = environment
                            .variables
                            .get_mut(&name)
                            .unwrap_or_else(|| panic!("Variável não definida: {}", name));

                        *value += target;
                    }
                    _ => panic!("Erro: Token inválido após operação de atribuição incremento"),
                },
                _ => todo!(),
            },
            Token::Imprima => match lexer.next()? {
                Token::Identificador(name) => match environment.variables.get(&name) {
                    Some(value) => println!("{value}"),
                    None => panic!("Erro: Variavel inexperada '{name}'"),
                },
                Token::Numero(value) => println!("{value}"),
                _ => panic!("Erro: Token inválido após imprima"),
            },
            Token::Enquanto => {
                let Token::Identificador(condition_var_name) = lexer.next()? else {
                    panic!("Erro: Esperava-se um identificador após 'enquanto'");
                };

                let Token::Operador(operation) = lexer.next()? else {
                    panic!("Erro: Esperava-se um operador após o identificador na condição do 'enquanto'");
                };

                let Token::Numero(condition_value) = lexer.next()? else {
                    panic!("Erro: Esperava-se um número após o operador na condição do 'enquanto'");
                };

                let Token::Faca = lexer.next()? else {
                    panic!(
                        "Erro: Esperava-se token 'faça' após expressão de condição do 'enquanto'"
                    );
                };

                let enquanto_init = lexer.pos;

                let value = *environment.variables.get(&condition_var_name)?;
                match operation.as_str() {
                    "<" => {
                        if value >= condition_value {
                            continue;
                        }
                    }
                    ">" => {
                        if value <= condition_value {
                            continue;
                        }
                    }
                    "=" => {
                        if value != condition_value {
                            continue;
                        }
                    }
                    _ => panic!("Operador inválido na condição do 'enquanto'"),
                }

                // Avaliar a condição enquanto o loop estiver em execução
                loop {
                    interpret_code(lexer, environment)?;

                    let value = *environment.variables.get(&condition_var_name)?;

                    match operation.as_str() {
                        "<" => {
                            if value >= condition_value {
                                break;
                            }
                        }
                        ">" => {
                            if value <= condition_value {
                                break;
                            }
                        }
                        "=" => {
                            if value != condition_value {
                                break;
                            }
                        }
                        _ => panic!("Operador inválido na condição do 'enquanto'"),
                    }

                    lexer.pos = enquanto_init;
                }
            }
            Token::Fim => break,
            _ => todo!(),
        }
    }

    Some(())
}

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

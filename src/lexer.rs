use crate::error::{Result, Error};
use crate::operator::Operador;
use crate::token::Token;
use crate::value::Valor;

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Position {
    pub file_name: String,
    pub line_num: usize,
    pub curr_char: usize,
    pub line_start: usize,
}

impl Position {
    fn new(file_name: String) -> Self {
        Self {
            file_name,
            line_num: 1,
            curr_char: 0,
            line_start: 0,
        }
    }

    fn incr(&mut self) {
        self.curr_char += 1;
    }

    fn incr_line(&mut self) {
        self.line_num += 1;
        self.line_start = self.curr_char;
    }
}

pub struct Lexer {
    code: Vec<char>,
    pub pos: Position,
}

impl Lexer {
    pub fn new(file_name: String, code: &str) -> Self {
        Self {
            code: code.chars().collect(),
            pos: Position::new(file_name),
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.code.get(self.pos.curr_char) {
            if c == '\n' {
                self.pos.incr();
                self.pos.incr_line();
            } else if c.is_whitespace() {
                self.pos.incr();
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(&c) = self.code.get(self.pos.curr_char) {
            if c.is_alphanumeric() {
                identifier.push(c);
                self.pos.incr();
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_number(&mut self) -> String {
        let mut num_str = String::new();
        let mut state: i8 = 1;

        while let Some(&c) = self.code.get(self.pos.curr_char) {
            match state {
                1 => {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        self.pos.incr();
                    } else {
                        state += 1;
                    }
                }
                2 => {
                    if c == '.' {
                        num_str.push(c);
                        self.pos.incr();
                        state = 3;
                    } else {
                        break;
                    }
                }
                3 => {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        self.pos.incr();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        num_str
    }

    pub fn next(&mut self) -> Result<Token> {
        self.consume_whitespace();

        let Some(&c) = self.code.get(self.pos.curr_char) else {
            return Ok(Token::FimDoArquivo);
        };

        match c {
            '0'..='9' => {
                // match for number literal
                let num = self
                    .consume_number()
                    .parse::<f32>()
                    .map_err(|err| Error::new(&err.to_string(), &self.pos))?;

                Ok(Token::Valor(Valor::Numero(num)))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' => {
                self.pos.incr();
                if let Some(&next_c) = self.code.get(self.pos.curr_char) {
                    let operador = match c {
                        '<' => {
                            if next_c == '=' {
                                self.pos.incr();
                                Operador::MenorIgualQue
                            } else {
                                Operador::MenorQue
                            }
                        }
                        '>' => {
                            if next_c == '=' {
                                self.pos.incr();
                                Operador::MaiorIgualQue
                            } else {
                                Operador::MaiorQue
                            }
                        }
                        '=' => {
                            if next_c == '=' {
                                self.pos.incr();
                                Operador::Igual
                            } else {
                                Operador::Atribuicao
                            }
                        }
                        '+' => {
                            if next_c == '+' {
                                self.pos.incr();
                                Operador::AutoAdicao
                            } else if next_c == '=' {
                                self.pos.incr();
                                Operador::SomaAtribuicao
                            } else {
                                Operador::Adicao
                            }
                        }
                        '-' => {
                            if next_c == '-' {
                                self.pos.incr();
                                Operador::AutoSubtracao
                            } else if next_c == '=' {
                                self.pos.incr();
                                Operador::SubtracaoAtribuicao
                            } else {
                                Operador::Subtracao
                            }
                        }
                        '*' => {
                            if next_c == '=' {
                                self.pos.incr();
                                Operador::MultiplicacaoAtribuicao
                            } else {
                                Operador::Multiplicacao
                            }
                        }
                        '/' => {
                            if next_c == '=' {
                                self.pos.incr();
                                Operador::DivisaoAtribuicao
                            } else {
                                Operador::Divisao
                            }
                        }
                        '%' => Operador::Resto,
                        _ => unreachable!(),
                    };

                    Ok(Token::Operador(operador))
                } else {
                    let operador = match c {
                        '<' => Operador::MenorQue,
                        '>' => Operador::MaiorQue,
                        '=' => Operador::Atribuicao,
                        '+' => Operador::Adicao,
                        '-' => Operador::Subtracao,
                        '*' => Operador::Multiplicacao,
                        '/' => Operador::Divisao,
                        '%' => Operador::Resto,
                        _ => unreachable!(),
                    };
                    Ok(Token::Operador(operador))
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.consume_identifier();
                match identifier.as_str() {
                    "seja" => Ok(Token::Seja),
                    "faça" => Ok(Token::Faca),
                    "então" => Ok(Token::Entao),
                    "imprima" => Ok(Token::Imprima),
                    "enquanto" => Ok(Token::Enquanto),
                    "se" => Ok(Token::Se),
                    "função" => Ok(Token::Funcao),
                    "para" => Ok(Token::Para),
                    "retorne" => Ok(Token::Retorne),
                    "fim" => Ok(Token::Fim),
                    _ => Ok(Token::Identificador(identifier)),
                }
            }

            _ => Err(Error::new("Token inesperado", &self.pos)),
        }
    }
}

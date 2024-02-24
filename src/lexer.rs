use crate::error::{Error, Result};
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
        self.curr_char += 1;
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

    fn consume_string(&mut self) -> String {
        let mut val_str = String::new();
        let mut state: i8 = 1;

        while let Some(&c) = self.code.get(self.pos.curr_char) {
            self.pos.incr();
            if let 1 = state {
                state += 1;
            } else {
                if c != '"' {
                    val_str.push(c);
                } else {
                    break;
                }
            }
        }

        val_str
    }

    pub fn peek(&mut self) -> Result<Token> {
        let pos = self.pos.clone();
        let next = self.next();
        self.pos = pos;
        next
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
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' | '&' | '|' => {
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
                        '%' => {
                            if next_c == '=' {
                                self.pos.incr();
                                Operador::RestoAtribuicao
                            } else {
                                Operador::Resto
                            }
                        }
                        '&' => {
                            if next_c == '&' {
                                self.pos.incr();
                                Operador::CondicionalE
                            } else {
                                todo!()
                            }
                        }
                        '|' => {
                            if next_c == '|' {
                                self.pos.incr();
                                Operador::CondicionalOu
                            } else {
                                todo!()
                            }
                        }
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
                        '&' => todo!(),
                        '|' => todo!(),
                        _ => unreachable!(),
                    };
                    Ok(Token::Operador(operador))
                }
            }
            '"' => {
                let val = self.consume_string();
                Ok(Token::Valor(Valor::Texto(val)))
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.consume_identifier();
                let token = match identifier.as_str() {
                    "seja" => Token::Seja,
                    "faca" => Token::Faca,
                    "entao" => Token::Entao,
                    "imprima" => Token::Imprima,
                    "enquanto" => Token::Enquanto,
                    "se" => Token::Se,
                    "função" => Token::Funcao,
                    "para" => Token::Para,
                    "retorne" => Token::Retorne,
                    "fim" => Token::Fim,
                    "nulo" => Token::Valor(Valor::Nulo),
                    _ => Token::Identificador(identifier),
                };

                Ok(token)
            }

            _ => Err(Error::new("Token inesperado", &self.pos)),
        }
    }
}

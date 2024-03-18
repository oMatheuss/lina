use std::char;
use std::str::Chars;

use crate::error::{Error, Result};
use crate::token::{Token, Operador, Valor};


pub struct Lexer<'a> {
    file_name: String,
    input: Chars<'a>,
    position: usize,
    line_num: usize,
    line_start: usize,
    curr_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(file_name: String, code: &str) -> Self {
        let mut chars = code.chars();
        Self {
            file_name,
            input: chars,
            position: 0,
            line_num: 0,
            line_start: 0,
            curr_char: chars.next(),
        }
    }

    fn next_char(&mut self) {
        self.curr_char = self.input.next();
        self.position += 1;
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.curr_char {
            if c.is_whitespace() {
                self.next_char();
                if c == '\n' || c == '\r' {
                    if let Some('\n') = self.curr_char {
                        self.next_char();
                    }
                    self.line_num += 1;
                    self.line_start = self.position;
                }
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.curr_char {
            if c.is_alphanumeric() {
                identifier.push(c);
            } else {
                break;
            }

            self.next_char();
        }
        identifier
    }

    fn consume_number(&mut self) -> String {
        let mut num_str = String::new();
        let mut state: i8 = 1;

        while let Some(c) = self.curr_char {
            match state {
                1 => {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                    } else {
                        state += 1;
                    }
                }
                2 => {
                    if c == '.' {
                        num_str.push(c);
                        state = 3;
                    } else {
                        break;
                    }
                }
                3 => {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
            
            self.next_char();
        }

        num_str
    }

    fn consume_string(&mut self) -> Result<String> {
        let mut value = String::new();
        let mut state: i8 = 1;
        self.next_char();

        while let Some(c) = self.curr_char {
            if c != '"' {
                value.push(c);
            } else {
                state += 1;
                break;
            }

            self.next_char();
        }

        if state != 3 {
            todo!()
            //Err(Error::new("", self.))
        }

        Ok(value)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        self.consume_whitespace();

        let Some(c) = self.curr_char else {
            return Ok(None);
        };

        match c {
            '0'..='9' => {
                // match for number literal
                let num = self
                    .consume_number()
                    .parse::<f32>()
                    .map_err(|err| Error::new(&err.to_string(), &self.position))?;

                Ok(Some(Token::Valor(Valor::Numero(num))))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' | '&' | '|' => {
                self.next_char();

                if let Some(next_c) = self.curr_char {
                    let operador = match c {
                        '<' => {
                            if next_c == '=' {
                                self.next_char();
                                Operador::MenorIgualQue
                            } else {
                                Operador::MenorQue
                            }
                        }
                        '>' => {
                            if next_c == '=' {
                                self.next_char();
                                Operador::MaiorIgualQue
                            } else {
                                Operador::MaiorQue
                            }
                        }
                        '=' => {
                            if next_c == '=' {
                                self.next_char();
                                Operador::Igual
                            } else {
                                Operador::Atribuicao
                            }
                        }
                        '+' => {
                            if next_c == '+' {
                                self.next_char();
                                Operador::AutoAdicao
                            } else if next_c == '=' {
                                self.next_char();
                                Operador::SomaAtribuicao
                            } else {
                                Operador::Adicao
                            }
                        }
                        '-' => {
                            if next_c == '-' {
                                self.next_char();
                                Operador::AutoSubtracao
                            } else if next_c == '=' {
                                self.next_char();
                                Operador::SubtracaoAtribuicao
                            } else {
                                Operador::Subtracao
                            }
                        }
                        '*' => {
                            if next_c == '=' {
                                self.next_char();
                                Operador::MultiplicacaoAtribuicao
                            } else {
                                Operador::Multiplicacao
                            }
                        }
                        '/' => {
                            if next_c == '=' {
                                self.next_char();
                                Operador::DivisaoAtribuicao
                            } else {
                                Operador::Divisao
                            }
                        }
                        '%' => {
                            if next_c == '=' {
                                self.next_char();
                                Operador::RestoAtribuicao
                            } else {
                                Operador::Resto
                            }
                        }
                        '&' => {
                            if next_c == '&' {
                                self.next_char();
                                Operador::CondicionalE
                            } else {
                                todo!()
                            }
                        }
                        '|' => {
                            if next_c == '|' {
                                self.next_char();
                                Operador::CondicionalOu
                            } else {
                                todo!()
                            }
                        }
                        _ => unreachable!(),
                    };

                    Ok(Some(Token::Operador(operador)))
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
                    Ok(Some(Token::Operador(operador)))
                }
            }
            '"' => {
                let val = self.consume_string()?;
                Ok(Some(Token::Valor(Valor::Texto(val))))
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

                Ok(Some(token))
            }

            _ => Err(Error::new("Token inesperado", &self.position)),
        }
    }

}

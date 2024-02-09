use std::{fmt::Display, str::Chars};

use crate::token::Token;

pub struct LexicalError {
    message: String,
    row: usize,
    col: usize,
}

impl LexicalError {
    fn new(message: &str, row: usize, col: usize) -> Self {
        LexicalError {
            message: String::from(message),
            row,
            col,
        }
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Erro Léxico: {}", self.message)?;
        writeln!(f, "--> {}:{}", self.row, self.col)
    }
}

pub struct Lexer<'a> {
    code: Chars<'a>,
    code_size: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code: code.chars(),
            code_size: code.len(),
            pos: 0,
        }
    }

    fn create_error(&self, message: &str) -> LexicalError {
        LexicalError::new(message, 0, 0)
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.code.nth(self.pos) {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.pos < self.code_size && self.code.nth(self.pos).unwrap().is_alphanumeric() {
            identifier.push(self.code.nth(self.pos).unwrap());
            self.pos += 1;
        }
        identifier
    }

    pub fn next(&mut self) -> Result<Token, LexicalError> {
        self.consume_whitespace();

        if self.pos >= self.code_size || None == self.code.nth(self.pos) {
            return Err(self.create_error("Erro: fim inesperado do arquivo"));
        }

        let Some(c) = self.code.nth(self.pos);

        match c {
            '0'..='9' => {
                // match for number literal
                let mut num_str = String::new();
                while self.pos < self.code_size && self.code.nth(self.pos).unwrap().is_numeric() {
                    num_str.push(self.code.nth(self.pos).unwrap());
                    self.pos += 1;
                }

                let num = num_str
                    .parse()
                    .map_err(|err| self.create_error(&format!("Erro: {}", err)))?;

                Ok(Token::Numero(num))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' => {
                if let Some(next_c) = self.code.nth(self.pos + 1) {
                    if (c == '+' || c == '-') && next_c == c {
                        // match for ++ and --
                        self.pos += 2;
                        match c {
                            '+' => Ok(Token::Incremento),
                            '-' => Ok(Token::Decremento),
                            _ => unreachable!(),
                        }
                    } else if next_c == '=' {
                        // match for += and -=
                        self.pos += 2;
                        match c {
                            '+' => Ok(Token::AtribuicaoIncremento),
                            '-' => Ok(Token::AtribuicaoDecremento),
                            _ => todo!(),
                        }
                    } else {
                        // match for any given operator +, -, *, ...
                        self.pos += 1;
                        Ok(Token::Operador(c.to_string()))
                    }
                } else {
                    // match for any given operator +, -, *, ...
                    self.pos += 1;
                    Ok(Token::Operador(c.to_string()))
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
            _ => Err(self.create_error("Token inesperado")),
        }
    }
}

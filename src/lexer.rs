use std::fmt::Display;

use crate::token::Token;

pub struct LexicalError {
    message: String,
    row: usize,
    col: usize,
	file_name: String,
}

impl LexicalError {
    pub fn new(message: &str, file: &str, row: usize, col: usize) -> Self {
        Self {
            message: String::from(message),
            row,
            col,
			file_name: String::from(file)
        }
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Erro Léxico: {}", self.message)?;
        writeln!(f, "--> {} {}:{}", self.file_name, self.row, self.col)
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Position {
    file_name: String,
    line_num: usize,
    curr_char: usize,
    line_start: usize,
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

    fn increment_char(&mut self) {
        self.curr_char += 1;
    }

    fn increment_char_by(&mut self, v: usize) {
        self.curr_char += v;
    }

    fn increment_line(&mut self) {
        self.line_num += 1;
        self.line_start = self.curr_char;
    }

    pub fn get_col(&self) -> usize {
        self.curr_char - self.line_start
    }

    pub fn get_row(&self) -> usize {
        self.line_num
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

    fn create_error(&self, message: &str) -> LexicalError {
        LexicalError::new(
            message,
			&self.pos.file_name,
            self.pos.line_num,
            self.pos.curr_char - self.pos.line_start,
        )
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.code.get(self.pos.curr_char) {
            if c == '\n' {
                self.pos.increment_char();
                self.pos.increment_line();
            } else if c.is_whitespace() {
                self.pos.increment_char();
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
                self.pos.increment_char();
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_number(&mut self) -> String {
        let mut num_str = String::new();
        while let Some(&c) = self.code.get(self.pos.curr_char) {
            if c.is_numeric() {
                num_str.push(c);
                self.pos.increment_char();
            } else {
                break;
            }
        }
        num_str
    }

    pub fn next(&mut self) -> Result<Token, LexicalError> {
        self.consume_whitespace();

        let Some(&c) = self.code.get(self.pos.curr_char) else {
            return Ok(Token::FimDoArquivo);
        };

        match c {
            '0'..='9' => {
                // match for number literal
                let num = self
                    .consume_number()
                    .parse()
                    .map_err(|err| self.create_error(&format!("Erro: {}", err)))?;

                Ok(Token::Numero(num))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' => {
                if let Some(&next_c) = self.code.get(self.pos.curr_char + 1) {
                    if (c == '+' || c == '-') && next_c == c {
                        // match for ++ and --
                        self.pos.increment_char_by(2);
                        match c {
                            '+' => Ok(Token::Incremento),
                            '-' => Ok(Token::Decremento),
                            _ => unreachable!(),
                        }
                    } else if next_c == '=' {
                        // match for += and -=
                        self.pos.increment_char_by(2);
                        match c {
                            '+' => Ok(Token::AtribuicaoIncremento),
                            '-' => Ok(Token::AtribuicaoDecremento),
                            _ => todo!(),
                        }
                    } else {
                        // match for any given operator +, -, *, ...
                        self.pos.increment_char();
                        Ok(Token::Operador(c.to_string()))
                    }
                } else {
                    // match for any given operator +, -, *, ...
                    self.pos.increment_char();
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

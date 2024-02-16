use crate::error::Error;
use crate::token::{Operador, Token, Valor};

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

    pub fn create_error(&self, message: &str) -> Error {
        Error::new(
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
        let mut state: i8 = 1;

        while let Some(&c) = self.code.get(self.pos.curr_char) {
            match state {
                1 => {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        self.pos.increment_char();
                    } else {
                        state += 1;
                    }
                }
                2 => {
                    if c == '.' {
                        num_str.push(c);
                        self.pos.increment_char();
                        state = 3;
                    } else {
                        break;
                    }
                }
                3 => {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        self.pos.increment_char();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        num_str
    }

    pub fn next(&mut self) -> Result<Token, Error> {
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
                    .map_err(|err| self.create_error(&format!("{}", err)))?;

                Ok(Token::Valor(Valor::Numero(num)))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' => {
                self.pos.increment_char();
                if let Some(&next_c) = self.code.get(self.pos.curr_char) {
                    let operador = match c {
                        '<' => {
                            if next_c == '=' {
                                self.pos.increment_char();
                                Operador::MenorIgualQue
                            } else {
                                Operador::MenorQue
                            }
                        }
                        '>' => {
                            if next_c == '=' {
                                self.pos.increment_char();
                                Operador::MaiorIgualQue
                            } else {
                                Operador::MaiorQue
                            }
                        }
                        '=' => {
                            if next_c == '=' {
                                self.pos.increment_char();
                                Operador::Igual
                            } else {
                                Operador::Atribuicao
                            }
                        }
                        '+' => {
                            if next_c == '+' {
                                self.pos.increment_char();
                                Operador::AutoAdicao
                            } else if next_c == '=' {
                                self.pos.increment_char();
                                Operador::SomaAtribuicao
                            } else {
                                Operador::Adicao
                            }
                        }
                        '-' => {
                            if next_c == '-' {
                                self.pos.increment_char();
                                Operador::AutoSubtracao
                            } else if next_c == '=' {
                                self.pos.increment_char();
                                Operador::SubtracaoAtribuicao
                            } else {
                                Operador::Subtracao
                            }
                        }
                        '*' => {
                            if next_c == '=' {
                                self.pos.increment_char();
                                Operador::MultiplicacaoAtribuicao
                            } else {
                                Operador::Multiplicacao
                            }
                        }
                        '/' => {
                            if next_c == '=' {
                                self.pos.increment_char();
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
            _ => Err(self.create_error("Token inesperado")),
        }
    }

    pub fn next_as_valor(&mut self) -> Result<Valor, Error> {
        let next_token = self.next()?;
        if let Token::Valor(valor) = next_token {
            Ok(valor)
        } else {
            Err(self.create_error(&format!("esperado valor, encontrou: {next_token}")))
        }
    }

    pub fn next_as_operador(&mut self) -> Result<Operador, Error> {
        let next_token = self.next()?;
        if let Token::Operador(operador) = next_token {
            Ok(operador)
        } else {
            Err(self.create_error(&format!("esperado operador, encontrou: {next_token}")))
        }
    }
}

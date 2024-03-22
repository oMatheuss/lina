use std::char;
use std::str::Chars;

use crate::token::{TokenDef, TokenPos, Token, Operador, Valor};

pub struct Lexer<'a> {
    input: Chars<'a>,
    position: usize,
    line_num: usize,
    line_start: usize,
    curr_char: Option<char>,
}

#[derive(Debug)]
pub struct LexicalError {
    row: usize,
    col: usize,
    msg: String,
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Erro léxico: {}", self.msg);
        writeln!(f, "Posição -> {}:{}", self.row, self.col)
    }
}

type Result<T> = std::result::Result<T, LexicalError>;

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut new = Self {
            input: input.chars(),
            position: 0,
            line_num: 0,
            line_start: 0,
            curr_char: None,
        };
        new.next_char();
        new
    }

    fn new_error<T>(&self, message: &str) -> Result<T> {
        Err(LexicalError { row: self.line_num, col: self.position - self.line_start, msg: String::from(message) })
    }

    fn next_char(&mut self) {
        self.curr_char = self.input.nth(self.position);
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

    fn consume_string(&'a mut self) -> Result<&'a str> {
        todo!()
    }

    fn next_token(&'a mut self) -> Result<Option<TokenDef<'a>>> {
        self.consume_whitespace();

        let Some(c) = self.curr_char else {
            return Ok(None);
        };

        let position = TokenPos { row: self.line_num, col: self.position - self.line_start };

        match c {
            '0'..='9' => {
                // match for number literal
                let num = self
                    .consume_number()
                    .parse::<f32>()
                    .unwrap();

                Ok(Some(TokenDef {
                    kind: Token::Valor(Valor::Numero(num)),
                    position
                }))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' => {
                self.next_char();
        
                let operador = match (c, self.curr_char) {
                    ('<', Some('=')) => {
                        self.next_char();
                        Operador::MenorIgualQue
                    }
                    ('>', Some('=')) => {
                        self.next_char();
                        Operador::MaiorIgualQue
                    }
                    ('=', Some('=')) => {
                        self.next_char();
                        Operador::Igual
                    }
                    ('+', Some('+')) => {
                        self.next_char();
                        Operador::AutoAdicao
                    }
                    ('+', Some('=')) => {
                        self.next_char();
                        Operador::SomaAtribuicao
                    }
                    ('-', Some('-')) => {
                        self.next_char();
                        Operador::AutoSubtracao
                    }
                    ('-', Some('=')) => {
                        self.next_char();
                        Operador::SubtracaoAtribuicao
                    }
                    ('*', Some('=')) => {
                        self.next_char();
                        Operador::MultiplicacaoAtribuicao
                    }
                    ('/', Some('=')) => {
                        self.next_char();
                        Operador::DivisaoAtribuicao
                    }
                    ('%', Some('=')) => {
                        self.next_char();
                        Operador::RestoAtribuicao
                    }
                    ('<', _) => Operador::MenorQue,
                    ('>', _) => Operador::MaiorQue,
                    ('=', _) => Operador::Atribuicao,
                    ('+', _) => Operador::Adicao,
                    ('-', _) => Operador::Subtracao,
                    ('*', _) => Operador::Multiplicacao,
                    ('/', _) => Operador::Divisao,
                    ('%', _) => Operador::Resto,
                    _ => unreachable!()
                };
        
                Ok(Some(TokenDef {
                    kind: Token::Operador(operador),
                    position
                }))
            }
            '&' => {
                if let Some('&') = self.curr_char {
                    self.next_char();
                    Ok(Some(TokenDef {
                        kind: Token::Operador(Operador::CondicionalE),
                        position
                    }))
                } else {
                    todo!()
                }
            }
            '|' => {
                if let Some('|') = self.curr_char {
                    self.next_char();
                    Ok(Some(TokenDef {
                        kind: Token::Operador(Operador::CondicionalOu),
                        position
                    }))
                } else {
                    todo!()
                }
            }        
            '"' => {
                let val = self.consume_string()?;
                Ok(Some(TokenDef {
                    kind: Token::Valor(Valor::Texto(val)),
                    position
                }))
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.consume_identifier();
                let kind = match identifier.as_str() {
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

                Ok(Some(TokenDef { kind, position }))
            }

            _ => self.new_error("caracter não esperado"),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<TokenDef<'a>>> {
        let mut tokens: Vec<TokenDef<'a>> = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }
}

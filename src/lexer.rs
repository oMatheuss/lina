use std::char;
use std::str::Chars;

use crate::token::{Delimitador, Literal, Operador, Token, TokenDef, TokenPos};

pub struct Lexer<'a> {
    input: &'a str,
    char_iter: Chars<'a>,
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
        writeln!(f, "Erro léxico: {}", self.msg)?;
        writeln!(f, "Posição -> {}:{}", self.row, self.col)
    }
}

type Result<T> = std::result::Result<T, LexicalError>;

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut char_iter = input.chars();
        let curr_char = char_iter.next();
        Self {
            input,
            char_iter,
            position: 0,
            line_num: 0,
            line_start: 0,
            curr_char,
        }
    }

    fn new_error<T>(&self, message: &str) -> Result<T> {
        Err(LexicalError { row: self.line_num, col: self.position - self.line_start, msg: String::from(message) })
    }

    fn next_char(&mut self) {
        self.curr_char = self.char_iter.next();
        self.position += 1;
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.curr_char {
            if !c.is_whitespace() {
                break;
            }
    
            self.next_char();
    
            match c {
                '\r' => {
                    if let Some('\n') = self.curr_char {
                        self.next_char();
                    }

                    self.line_num += 1;
                    self.line_start = self.position;
                }
                '\n' => {
                    self.line_num += 1;
                    self.line_start = self.position;
                }
                _ => {}
            }
        }
    }

    fn consume_identifier(&mut self) -> &'a str {
        let start = self.position;
        while let Some(c) = self.curr_char {
            if c.is_alphanumeric() {
                self.next_char();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }

    fn consume_number(&mut self) -> Result<&'a str> {
        let start = self.position;
        let mut state: u8 = 1;

        while let Some(c) = self.curr_char {
            match state {
                1 => {
                    if c.is_ascii_digit() {
                        self.next_char();
                    } else if c == '.' {
                        self.next_char();
                        state = 2;
                    } else {
                        break;
                    }
                }
                2 => {
                    if c.is_ascii_digit() {
                        self.next_char();
                        state = 3;
                    } else {
                        break;
                    }
                }
                3 => {
                    if c.is_ascii_digit() {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }

        if state == 2 {
            return self.new_error("esperado número após ponto (.)");
        }

        Ok(&self.input[start..self.position])
    }

    fn consume_string(&mut self) -> Result<&'a str> {
        let mut state: u8 = 1;
        self.next_char();

        let start = self.position;

        while let Some(c) = self.curr_char {
            if c == '"' {
                state = 2;
                break;
            }

            self.next_char();
        }

        if state != 2 {
            return self.new_error("aspas (\") finais correspondentes não encontradas");
        }

        let result = &self.input[start..self.position];
        self.next_char();
        
        Ok(result)
    }

    fn next_token(&mut self) -> Result<Option<TokenDef<'a>>> {
        self.consume_whitespace();

        let Some(c) = self.curr_char else {
            return Ok(None);
        };

        let position = TokenPos { row: self.line_num, col: self.position - self.line_start };

        match c {
            '0'..='9' => {
                // match for number literal
                let num = self
                    .consume_number()?
                    .parse::<f32>()
                    .unwrap();

                Ok(Some(TokenDef {
                    kind: Token::Literal(Literal::Numero(num)),
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
                    kind: Token::Literal(Literal::Texto(val)),
                    position
                }))
            }
            '(' | ')' | '{' | '}' | '[' | ']' => {
                self.next_char();
                let del = match c {
                    '(' => Delimitador::AParen,
                    ')' => Delimitador::FParen,
                    '{' => Delimitador::AChave,
                    '}' => Delimitador::FChave,
                    '[' => Delimitador::AColch,
                    ']' => Delimitador::FColch,
                    _ => unreachable!()
                };

                Ok(Some(TokenDef { kind: Token::Delimitador(del), position }))
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.consume_identifier();
                let kind = match identifier {
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
                    "nulo" => Token::Literal(Literal::Nulo),
                    _ => Token::Identificador(identifier),
                };

                Ok(Some(TokenDef { kind, position }))
            }

            _ => self.new_error("caracter não esperado"),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenDef<'a>>> {
        let mut tokens: Vec<TokenDef<'a>> = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }
}

pub fn lex(code: &str) -> Result<Vec<TokenDef>> {
    Lexer::new(code).tokenize()
}
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
            line_num: 1,
            line_start: 0,
            curr_char,
        }
    }

    fn new_error<T>(&self, message: &str) -> Result<T> {
        Err(LexicalError {
            row: self.line_num,
            col: self.position - self.line_start,
            msg: String::from(message),
        })
    }

    fn next_char(&mut self) {
        self.curr_char = self.char_iter.next();
        if let Some(ch) = self.curr_char {
            self.position += ch.len_utf8();
        }
    }

    fn get_pos(&self) -> TokenPos {
        TokenPos {
            row: self.line_num,
            col: self.position - self.line_start,
        }
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
                return &self.input[start..self.position];
            }
        }
        &self.input[start..=self.position]
    }

    fn consume_number(&mut self) -> Result<Literal<'a>> {
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

        let string = if let Some(..) = self.curr_char {
            &self.input[start..self.position]
        } else {
            &self.input[start..=self.position]
        };

        match state {
            1 => {
                let inteiro = string.parse().map_err(|err| LexicalError {
                    row: self.line_num,
                    col: start - self.line_start,
                    msg: format!("{string} não pôde ser convertido para inteiro: {err}"),
                })?;
                Ok(Literal::Inteiro(inteiro))
            }
            2 => self.new_error("esperado número após ponto (.)"),
            3 => {
                let decimal = string.parse().map_err(|err| LexicalError {
                    row: self.line_num,
                    col: start - self.line_start,
                    msg: format!("{string} não pôde ser convertido para decimal: {err}"),
                })?;
                Ok(Literal::Decimal(decimal))
            }
            _ => unreachable!(),
        }
    }

    fn consume_string(&mut self) -> Result<Literal<'a>> {
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

        Ok(Literal::Texto(result))
    }

    fn next_token(&mut self) -> Result<Option<TokenDef<'a>>> {
        self.consume_whitespace();

        let Some(c) = self.curr_char else {
            return Ok(None);
        };

        let pos = self.get_pos();

        match c {
            '0'..='9' => {
                // match for number literal
                let literal = self.consume_number()?;

                Ok(Some(TokenDef {
                    tok: Token::Literal(literal),
                    pos,
                }))
            }
            '<' | '>' | '=' | '+' | '-' | '*' | '/' | '%' | '^' | ':' => {
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
                    ('<', Some('>')) => {
                        self.next_char();
                        Operador::Diferente
                    }
                    (':', Some('=')) => {
                        self.next_char();
                        Operador::Atrib
                    }
                    ('+', Some('=')) => {
                        self.next_char();
                        Operador::AdicAtrib
                    }
                    ('-', Some('=')) => {
                        self.next_char();
                        Operador::SubtAtrib
                    }
                    ('*', Some('=')) => {
                        self.next_char();
                        Operador::MultAtrib
                    }
                    ('/', Some('=')) => {
                        self.next_char();
                        Operador::DivAtrib
                    }
                    ('%', Some('=')) => {
                        self.next_char();
                        Operador::RestoAtrib
                    }
                    ('^', Some('=')) => {
                        self.next_char();
                        Operador::ExpAtrib
                    }
                    ('=', _) => Operador::Igual,
                    ('<', _) => Operador::MenorQue,
                    ('>', _) => Operador::MaiorQue,
                    ('+', _) => Operador::Adic,
                    ('-', _) => Operador::Subt,
                    ('*', _) => Operador::Mult,
                    ('/', _) => Operador::Div,
                    ('%', _) => Operador::Resto,
                    ('^', _) => Operador::Exp,
                    _ => self.new_error("operador inválido")?,
                };

                Ok(Some(TokenDef {
                    tok: Token::Operador(operador),
                    pos,
                }))
            }
            '"' => {
                let val = self.consume_string()?;
                Ok(Some(TokenDef {
                    tok: Token::Literal(val),
                    pos,
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
                    _ => unreachable!(),
                };

                Ok(Some(TokenDef {
                    tok: Token::Delimitador(del),
                    pos,
                }))
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.consume_identifier();
                let tok = match identifier {
                    "programa" => Token::Programa,
                    "seja" => Token::Seja,
                    "inteiro" => Token::Inteiro,
                    "real" => Token::Real,
                    "texto" => Token::Texto,
                    "booleano" => Token::Booleano,
                    "repetir" => Token::Repetir,
                    "entao" => Token::Entao,
                    "enquanto" => Token::Enquanto,
                    "se" => Token::Se,
                    "função" => Token::Funcao,
                    "para" => Token::Para,
                    "incremento" => Token::Incremento,
                    "retorne" => Token::Retorne,
                    "fim" => Token::Fim,
                    "e" => Token::Operador(Operador::E),
                    "ou" => Token::Operador(Operador::Ou),
                    "ate" => Token::Ate,
                    "verdadeiro" => Token::Literal(Literal::Booleano(true)),
                    "falso" => Token::Literal(Literal::Booleano(false)),
                    _ => Token::Identificador(identifier),
                };

                Ok(Some(TokenDef { tok, pos }))
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

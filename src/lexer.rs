use crate::token::Token;

pub struct Lexer<'a> {
    code: &'a str,
    pub pos: usize,
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

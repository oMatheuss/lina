use std::fmt::Display;

use crate::{operator::Operador, value::Valor};

#[derive(Debug, PartialEq)]
pub enum Token {
    Seja,
    Faca,
    Entao,
    Imprima,
    Enquanto,
    Se,
    Funcao,
    Para,
    Retorne,
    Identificador(String),
    Valor(Valor),
    Operador(Operador),
    Fim,
    FimDoArquivo,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Seja => write!(f, "seja"),
            Token::Faca => write!(f, "faça"),
            Token::Entao => write!(f, "então"),
            Token::Imprima => write!(f, "imprima"),
            Token::Enquanto => write!(f, "enquanto"),
            Token::Se => write!(f, "se"),
            Token::Funcao => write!(f, "função"),
            Token::Para => write!(f, "para"),
            Token::Retorne => write!(f, "retorne"),
            Token::Identificador(idt) => write!(f, "idetificador: {idt}"),
            Token::Valor(val) => write!(f, "valor: {val}"),
            Token::Operador(ope) => write!(f, "operador: {ope}"),
            Token::Fim => write!(f, "fim"),
            Token::FimDoArquivo => write!(f, "eof"),
        }
    }
}
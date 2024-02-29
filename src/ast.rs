use crate::lexer::Position;
use crate::token::{Operador, Valor};

#[derive(Debug, Clone)]
pub struct WithPosition<T> {
    pub item: T,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub enum Expressao {
    Var(WithPosition<String>),
    Val(WithPosition<Valor>),
    Ope(Box<Expressao>, WithPosition<Operador>, Box<Expressao>),
}

#[derive(Debug, Clone)]
pub struct Declaracao {
    pub nome: WithPosition<String>,
    pub valor: Option<Expressao>,
}

#[derive(Debug, Clone)]
pub struct Atribuicao {
    pub nome: WithPosition<String>,
    pub operador: WithPosition<Operador>,
    pub expressao: Expressao,
}

#[derive(Debug, Clone)]
pub struct Incremento {
    pub nome: WithPosition<String>,
}

#[derive(Debug, Clone)]
pub struct Enquanto {
    pub condicao: WithPosition<Expressao>,
    pub corpo: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Imprima {
    pub expressao: Expressao,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaracao(Declaracao),
    Atribuicao(Atribuicao),
    Incremento(Incremento),
    Enquanto(Enquanto),
    Imprima(Imprima),
    Fim,
}

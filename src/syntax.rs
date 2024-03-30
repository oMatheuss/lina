use crate::token::{Token, Operador, Literal, TokenDef, TokenPos};

#[derive(Debug, Clone)]
pub enum SyntaxTree<'a> {
    Program(Vec<SyntaxTree<'a>>),
    Statement(Statement<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Assignment {
        ident: &'a str,
        exprs: Expression<'a>,
    },
}


#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Literal(Literal<'a>),
    Identifier(&'a str),
    BinOp {
        ope: Operador,
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
}
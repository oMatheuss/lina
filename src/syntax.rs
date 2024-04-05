use crate::token::{Operador, Literal};

pub type Block<'a> = Vec<SyntaxTree<'a>>;

#[derive(Debug, Clone)]
pub enum SyntaxTree<'a> {
    Program(Block<'a>),
    Assign {
        ident: &'a str,
        ope: Operador,
        exprs: Expression<'a>,
    },
    SeStmt {
        expr: Expression<'a>,
        block: Block<'a>
    },
    EnquantoStmt {
        expr: Expression<'a>,
        block: Block<'a>
    },
    ParaStmt {
        ident: &'a str,
        expr: Expression<'a>,
        block: Block<'a>
    },
    Imprima {
        expr: Expression<'a>,
    }
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
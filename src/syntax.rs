use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::token::{Literal, Operador};

#[derive(Debug, Clone)]
pub struct Program<'a> {
    pub name: &'a str,
    pub block: Block<'a>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Integer,
    Real,
    Text,
    Boolean,
    Vector,
}

#[derive(Debug, Clone)]
pub enum SyntaxTree<'a> {
    Assign {
        ident: &'a str,
        exprs: Expression<'a>,
    },
    Expr(Expression<'a>),
    SeStmt {
        expr: Expression<'a>,
        block: Block<'a>,
    },
    EnquantoStmt {
        expr: Expression<'a>,
        block: Block<'a>,
    },
    ParaStmt {
        ident: &'a str,
        expr: Expression<'a>,
        block: Block<'a>,
    },
}

#[derive(Debug, Clone)]
pub struct Block<'a>(Vec<SyntaxTree<'a>>);

impl<'a> Deref for Block<'a> {
    type Target = Vec<SyntaxTree<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Block<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Vec<SyntaxTree<'a>>> for Block<'a> {
    fn from(value: Vec<SyntaxTree<'a>>) -> Self {
        Block(value)
    }
}

impl<'a> Block<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
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

impl<'a> Display for Program<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "programa {}", self.name)?;

        for node in self.block.iter() {
            write!(f, "{}", node)?;
        }

        writeln!(f, "fim {}", self.name)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Integer => write!(f, "inteiro"),
            Type::Real => write!(f, "real"),
            Type::Text => write!(f, "texto"),
            Type::Boolean => write!(f, "booleano"),
            Type::Vector => write!(f, "vetor"),
        }
    }
}

impl<'a> Display for SyntaxTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxTree::Assign { ident, exprs } => {
                writeln!(f, "seja {ident} := {exprs}")
            }
            SyntaxTree::SeStmt { expr, block } => {
                writeln!(f, "se {expr} entao")?;
                write!(f, "{block}")?;
                writeln!(f, "fim")
            }
            SyntaxTree::EnquantoStmt { expr, block } => {
                writeln!(f, "enquanto {expr} faca")?;
                write!(f, "{block}")?;
                writeln!(f, "fim")
            }
            SyntaxTree::ParaStmt { ident, expr, block } => {
                writeln!(f, "para {ident}, {expr} faca")?;
                write!(f, "{block}")?;
                writeln!(f, "fim")
            }
            SyntaxTree::Expr(expr) => {
                writeln!(f, "{expr}")
            }
        }
    }
}

impl<'a> Display for Block<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub_stmts = self
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .trim_end()
            .split('\n')
            .map(|stmt| format!("{:ident$}{stmt}\n", "", ident = 4))
            .collect::<String>();
        write!(f, "{}", sub_stmts)
    }
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::BinOp { ope, lhs, rhs } => {
                write!(f, "({} {} {})", lhs, ope, rhs)
            }
        }
    }
}

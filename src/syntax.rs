use std::fmt::Display;

use crate::token::{Literal, Operador, TokenPos};

#[derive(Debug)]
pub struct Program<'a> {
    pub name: &'a str,
    pub block: Block<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Real,
    Text,
    Boolean,
    Vector,
}

#[derive(Debug)]
pub enum SyntaxTree<'a> {
    Assign {
        pos: TokenPos,
        typ: Type,
        idt: &'a str,
        exp: Expression<'a>,
    },
    Print(Expression<'a>),
    Expr(Expression<'a>),
    SeStmt {
        exp: Expression<'a>,
        blk: Block<'a>,
    },
    EnquantoStmt {
        exp: Expression<'a>,
        blk: Block<'a>,
    },
    ParaStmt {
        idt: &'a str,
        sta: Option<Literal<'a>>,
        lmt: Literal<'a>,
        stp: Option<Literal<'a>>,
        blk: Block<'a>,
    },
}

#[derive(Debug)]
pub struct Block<'a> {
    stmts: Vec<SyntaxTree<'a>>,
}

impl<'a> Block<'a> {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }
    pub fn push_stmt(&mut self, stmt: SyntaxTree<'a>) {
        self.stmts.push(stmt);
    }

    pub fn iter_stmts(&self) -> std::slice::Iter<'_, SyntaxTree> {
        self.stmts.iter()
    }
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Literal(Literal<'a>),
    Identifier(&'a str, Type),
    BinOp {
        typ: Type,
        ope: Operador,
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Cast(Box<Expression<'a>>, Type),
}

impl<'a> Expression<'a> {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Literal(ltr) => match ltr {
                Literal::Decimal(_) => Type::Real,
                Literal::Inteiro(_) => Type::Integer,
                Literal::Texto(_) => Type::Text,
                Literal::Booleano(_) => Type::Boolean,
                Literal::Nulo => todo!(),
            },
            Self::Identifier(_, typ) => typ.clone(),
            Self::BinOp { typ, .. } => typ.clone(),
            Self::Cast(_, typ) => typ.clone(),
        }
    }
}

impl<'a> Display for Program<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "programa {}", self.name)?;

        for node in self.block.iter_stmts() {
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
            SyntaxTree::Assign {
                pos: _,
                typ,
                idt,
                exp,
            } => {
                writeln!(f, "{typ} {idt} := {exp}")
            }
            SyntaxTree::SeStmt { exp, blk } => {
                writeln!(f, "se {exp} entao")?;
                write!(f, "{blk}")?;
                writeln!(f, "fim")
            }
            SyntaxTree::EnquantoStmt { exp, blk } => {
                writeln!(f, "enquanto {exp} faca")?;
                write!(f, "{blk}")?;
                writeln!(f, "fim")
            }
            SyntaxTree::ParaStmt {
                idt,
                sta,
                lmt,
                stp,
                blk,
            } => {
                let sta = sta.as_ref().unwrap_or(&Literal::Inteiro(0));
                let stp = stp.as_ref().unwrap_or(&Literal::Inteiro(1));
                writeln!(f, "para {idt} := {sta} ate {lmt} incremento {stp} repetir")?;
                write!(f, "{blk}")?;
                writeln!(f, "fim")
            }
            SyntaxTree::Expr(expr) => {
                writeln!(f, "{expr}")
            }
            SyntaxTree::Print(expr) => {
                writeln!(f, "saida := {expr}")
            }
        }
    }
}

impl<'a> Display for Block<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub_stmts = self
            .iter_stmts()
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
            Expression::Identifier(idt, typ) => write!(f, "({typ}){idt}"),
            Expression::BinOp { ope, lhs, rhs, .. } => {
                write!(f, "({} {} {})", lhs, ope, rhs)
            }
            Expression::Cast(exp, typ) => {
                write!(f, "({typ}){exp}")
            }
        }
    }
}

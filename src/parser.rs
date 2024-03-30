use std::iter::Peekable;
use std::mem;
use std::vec::IntoIter;

use crate::token::{Operador, Token, TokenDef, TokenPos, Literal};
use crate::syntax::{Expression, Statement, SyntaxTree};

#[derive(Debug)]
pub struct SyntaxError {
    pos: TokenPos,
    msg: String,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Erro de sintaxe: {}", self.msg)?;
        writeln!(f, "Posição -> {}:{}", self.pos.row, self.pos.col)
    }
}

type Result<T> = std::result::Result<T, SyntaxError>;

pub struct Parser<'a> {
    tokens: Peekable<IntoIter<TokenDef<'a>>>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<TokenDef<'a>>) -> Self {
        Parser { tokens: tokens.into_iter().peekable() }
    }

    fn peek(&mut self) -> Option<&TokenDef<'a>> {
        self.tokens.peek()
    }

    fn advance(&mut self) -> Result<TokenDef<'a>> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(SyntaxError { pos: TokenPos::default(), msg: "fim inesperado do arquivo".into() })
        }
    }

    fn consume_invariant(&mut self, expected: Token<'a>) -> Result<()> {
        let TokenDef { kind, position } = self.advance()?;
        if mem::discriminant(&kind) == mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(SyntaxError { pos: position, msg: format!("esperado {}, encontrou {}", expected, kind) })
        }
    }

    fn consume_identifier(&mut self) -> Result<&'a str> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Identificador(ident) = kind {
            Ok(ident)
        } else {
            Err(SyntaxError { pos: position.clone(), msg: format!("esperado identificador, encontrou {:?}", kind) })
        }
    }

    fn consume_literal(&mut self) -> Result<Literal<'a>> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Literal(literal) = kind {
            Ok(literal)
        } else {
            Err(SyntaxError { pos: position.clone(), msg: format!("esperado literal, encontrou {:?}", kind) })
        }
    }

    fn consume_operator(&mut self) -> Result<Operador> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Operador(operator) = kind {
            Ok(operator)
        } else {
            Err(SyntaxError { pos: position.clone(), msg: format!("esperado operador, encontrou {:?}", kind) })
        }
    }

    fn parse_program(&mut self) -> Result<SyntaxTree<'a>> {
        let mut program = Vec::new();
        while let Some(_) = self.peek() {
            program.push(self.parse_statement()?);
        }
        Ok(SyntaxTree::Program(program))
    }

    fn parse_statement(&mut self) -> Result<SyntaxTree<'a>> {
        self.consume_invariant(Token::Seja)?;
        let ident = self.consume_identifier()?;
        self.consume_invariant(Token::Operador(Operador::Atribuicao))?;
        let exprs = self.parse_expression()?;

        Ok(SyntaxTree::Statement(Statement::Assignment { ident, exprs }))
    }

    fn parse_expression(&mut self) -> Result<Expression<'a>> {
        let rhs = Expression::Literal(self.consume_literal()?);
        let ope = self.consume_operator()?;
        let lhs = Expression::Literal(self.consume_literal()?);


        Ok(Expression::BinOp { ope, lhs: Box::new(lhs), rhs: Box::new(rhs) })
    }

    pub fn parse(&mut self) -> Result<SyntaxTree<'a>> {
        self.parse_program()
    }
}

pub fn parse(tokens: Vec<TokenDef>) -> Result<SyntaxTree> {
    Parser::new(tokens).parse()
}
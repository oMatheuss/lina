use std::iter::Peekable;
use std::mem;
use std::vec::IntoIter;

use crate::token::{Delimitador, Literal, Operador, Token, TokenDef, TokenPos};
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

enum OpAssoc { R, L }
struct OpInfo(u8, OpAssoc);

fn operator_precedence(operator: &Operador) -> OpInfo {
    match operator {
        Operador::Adicao => OpInfo(1, OpAssoc::L),
        Operador::Subtracao => OpInfo(1, OpAssoc::L),
        Operador::Multiplicacao => OpInfo(2, OpAssoc::L),
        Operador::Divisao => OpInfo(2, OpAssoc::L),
        _ => unimplemented!()
    }
}

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
        self.tokens.next().ok_or_else(|| {
            SyntaxError { pos: TokenPos::default(), msg: "fim inesperado do arquivo".into() }
        })
    }

    fn new_error<T>(&self, message: &str, position: TokenPos) -> Result<T> {
        Err(SyntaxError { pos: position, msg: String::from(message) })
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
            Err(SyntaxError { pos: position, msg: format!("esperado identificador, encontrou {:?}", kind) })
        }
    }

    fn consume_literal(&mut self) -> Result<Literal<'a>> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Literal(literal) = kind {
            Ok(literal)
        } else {
            Err(SyntaxError { pos: position, msg: format!("esperado literal, encontrou {:?}", kind) })
        }
    }

    fn consume_operator(&mut self) -> Result<Operador> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Operador(operator) = kind {
            Ok(operator)
        } else {
            Err(SyntaxError { pos: position, msg: format!("esperado operador, encontrou {:?}", kind) })
        }
    }

    fn parse_program(&mut self) -> Result<SyntaxTree<'a>> {
        let mut program = Vec::new();
        while let Some(..) = self.peek() {
            program.push(self.parse_statement()?);
        }
        Ok(SyntaxTree::Program(program))
    }

    fn parse_statement(&mut self) -> Result<SyntaxTree<'a>> {
        let token_ref = self.peek().unwrap();
        let position = token_ref.position.clone();

        let statement = match token_ref.kind {
            Token::Seja => {
                self.consume_invariant(Token::Seja)?;
                let ident = self.consume_identifier()?;
                self.consume_invariant(Token::Operador(Operador::Atribuicao))?;
                let exprs = self.parse_expression(1)?;
                Statement::Assignment { ident, exprs }
            },
            Token::Faca => todo!(),
            Token::Imprima => todo!(),
            Token::Enquanto => todo!(),
            Token::Se => todo!(),
            Token::Funcao => todo!(),
            Token::Para => todo!(),
            Token::Retorne => todo!(),
            Token::Identificador(_) => todo!(),
            Token::Literal(_) => todo!(),
            _ => self.new_error("token inesperado", position)?
        };

        Ok(SyntaxTree::Statement(statement))
    }

    fn parse_atom(&mut self) -> Result<Expression<'a>> {
        let TokenDef { kind, position } = self.advance()?;
        let expression = match kind {
            Token::Identificador(ident) => Expression::Identifier(ident),
            Token::Literal(literal) => Expression::Literal(literal),
            Token::Delimitador(Delimitador::AParen) => {
                let inner_expr = self.parse_expression(1)?;
                self.consume_invariant(Token::Delimitador(Delimitador::FParen))?;
                inner_expr
            },
            _ => self.new_error("token inesperado", position)?
        };
        Ok(expression)
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression<'a>> {
        let mut lhs = self.parse_atom()?;

        loop {
            let Some(TokenDef { kind, position: _ }) = self.peek() else { break; };
            let Token::Operador(ope) = kind else { break; };
            let OpInfo(prec, assoc) = operator_precedence(ope);

            if prec < min_prec { break; }

            let ope = self.consume_operator()?;
            let min_prec = if let OpAssoc::L = assoc {prec + 1} else {prec};

            let rhs = self.parse_expression(min_prec)?;

            lhs = Expression::BinOp { ope, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        
        Ok(lhs)
    }

    pub fn parse(&mut self) -> Result<SyntaxTree<'a>> {
        self.parse_program()
    }
}

pub fn parse(tokens: Vec<TokenDef>) -> Result<SyntaxTree> {
    Parser::new(tokens).parse()
}
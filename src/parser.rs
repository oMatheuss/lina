use std::iter::Peekable;
use std::mem;
use std::vec::IntoIter;

use crate::syntax::{Block, Expression, Program, SyntaxTree, Type};
use crate::token::{Delimitador, Literal, OpAssoc, OpInfo, Operador, Token, TokenDef, TokenPos};

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
    tokens: Peekable<IntoIter<TokenDef<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<TokenDef<'a>>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&TokenDef<'a>> {
        self.tokens.peek()
    }

    fn advance(&mut self) -> Result<TokenDef<'a>> {
        self.tokens.next().ok_or_else(|| SyntaxError {
            pos: TokenPos::default(),
            msg: "fim inesperado do arquivo".into(),
        })
    }

    fn new_error<T>(&self, message: &str, position: TokenPos) -> Result<T> {
        Err(SyntaxError {
            pos: position,
            msg: String::from(message),
        })
    }

    fn consume_invariant(&mut self, expected: Token<'a>) -> Result<()> {
        let TokenDef { kind, position } = self.advance()?;
        if mem::discriminant(&kind) == mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(SyntaxError {
                pos: position,
                msg: format!("esperado {}, encontrou {}", expected, kind),
            })
        }
    }

    fn consume_identifier(&mut self) -> Result<&'a str> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Identificador(ident) = kind {
            Ok(ident)
        } else {
            Err(SyntaxError {
                pos: position,
                msg: format!("esperado identificador, encontrou {:?}", kind),
            })
        }
    }

    fn consume_literal(&mut self) -> Result<Literal<'a>> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Literal(literal) = kind {
            Ok(literal)
        } else {
            Err(SyntaxError {
                pos: position,
                msg: format!("esperado literal, encontrou {:?}", kind),
            })
        }
    }

    fn consume_operator(&mut self) -> Result<Operador> {
        let TokenDef { kind, position } = self.advance()?;
        if let Token::Operador(operator) = kind {
            Ok(operator)
        } else {
            Err(SyntaxError {
                pos: position,
                msg: format!("esperado operador, encontrou {:?}", kind),
            })
        }
    }

    fn parse_statement(&mut self) -> Result<SyntaxTree<'a>> {
        let token_ref = self.peek().unwrap();
        let position = token_ref.position.clone();

        let stmt = match token_ref.kind {
            Token::Seja | Token::Inteiro | Token::Real | Token::Booleano | Token::Texto => {
                let decl = self.advance()?;
                let ident = self.consume_identifier()?;
                self.consume_invariant(Token::Operador(Operador::Atrib))?;
                let expr = self.parse_expression(1)?;

                let vtype = match decl.kind {
                    Token::Seja => todo!(),
                    Token::Inteiro => Type::Integer,
                    Token::Real => Type::Real,
                    Token::Texto => Type::Text,
                    Token::Booleano => Type::Boolean,
                    _ => unreachable!(),
                };

                SyntaxTree::Assign {
                    pos: position,
                    ident,
                    expr,
                    vtype,
                }
            }
            Token::Identificador("saida") => {
                let _ = self.consume_identifier()?;
                self.consume_invariant(Token::Operador(Operador::Atrib))?;
                let expr = self.parse_expression(1)?;
                SyntaxTree::Print(expr)
            }
            Token::Enquanto => {
                self.consume_invariant(Token::Enquanto)?;
                let expr = self.parse_expression(1)?;
                self.consume_invariant(Token::Repetir)?;
                let block = self.parse_block()?;
                SyntaxTree::EnquantoStmt { expr, block }
            }
            Token::Se => {
                self.consume_invariant(Token::Se)?;
                let expr = self.parse_expression(1)?;
                self.consume_invariant(Token::Entao)?;
                let block = self.parse_block()?;
                SyntaxTree::SeStmt { expr, block }
            }
            Token::Para => {
                self.consume_invariant(Token::Para)?;
                let ident = self.consume_identifier()?;
                self.consume_invariant(Token::Ate)?;
                let limit = self.consume_literal()?;
                self.consume_invariant(Token::Repetir)?;
                let block = self.parse_block()?;
                SyntaxTree::ParaStmt {
                    ident,
                    limit,
                    block,
                }
            }
            Token::Funcao => todo!(),
            Token::Retorne => todo!(),
            Token::Identificador(..) | Token::Literal(..) | Token::Delimitador(..) => {
                let expression = self.parse_expression(1)?;
                SyntaxTree::Expr(expression)
            }
            _ => self.new_error("token inesperado", position)?,
        };

        Ok(stmt)
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
            }
            _ => self.new_error("token inesperado", position)?,
        };
        Ok(expression)
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression<'a>> {
        let mut lhs = self.parse_atom()?;

        loop {
            let Some(TokenDef {
                kind,
                position: op_pos,
            }) = self.peek()
            else {
                break;
            };
            let pos = op_pos.clone();
            let Token::Operador(ope) = kind else {
                break;
            };
            let OpInfo(prec, assoc) = ope.precedence();

            if prec < min_prec {
                break;
            }

            let ope = self.consume_operator()?;
            let min_prec = if let OpAssoc::L = assoc {
                prec + 1
            } else {
                prec
            };

            if ope.is_atrib() {
                if let Expression::Literal(..) = lhs {
                    self.new_error(
                        "lado esquerdo de um operador de atribuição não pode ser um literal",
                        pos,
                    )?;
                }
            }

            let rhs = self.parse_expression(min_prec)?;

            lhs = Expression::BinOp {
                ope,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_block(&mut self) -> Result<Block<'a>> {
        let mut block = Block::new();

        while let Some(token) = self.peek() {
            if token.kind == Token::Fim {
                break;
            }
            let stmt = self.parse_statement()?;
            block.push_stmt(stmt);
        }
        self.consume_invariant(Token::Fim)?;

        Ok(block)
    }

    fn parse_program(&mut self) -> Result<Program<'a>> {
        let mut block = Block::new();

        self.consume_invariant(Token::Programa)?;
        let name = self.consume_identifier()?;

        while let Some(..) = self.peek() {
            let stmt = self.parse_statement()?;
            block.push_stmt(stmt);
        }

        Ok(Program { name, block })
    }

    pub fn parse(&mut self) -> Result<Program<'a>> {
        self.parse_program()
    }
}

pub fn parse(tokens: Vec<TokenDef>) -> Result<Program> {
    Parser::new(tokens).parse()
}

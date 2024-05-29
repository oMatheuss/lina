use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::syntax::{Block, Expression, Program, SyntaxTree, Type};
use crate::token::{Delimitador, Literal, OpAssoc, OpInfo, Operador, Token, TokenDef, TokenPos};

#[derive(Debug)]
pub struct SyntaxError {
    pub pos: TokenPos,
    pub msg: String,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Erro de sintaxe: {}", self.msg)?;
        writeln!(f, "Posição -> {}:{}", self.pos.row, self.pos.col)
    }
}

type Result<T> = std::result::Result<T, SyntaxError>;

struct Symbol {
    pos: TokenPos,
    typ: Type,
}

type TokenTable<'a> = HashMap<&'a str, Symbol>;

pub struct Parser<'a> {
    tokens: Peekable<IntoIter<TokenDef<'a>>>,
    symbols: Vec<TokenTable<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<TokenDef<'a>>) -> Self {
        let mut globals = TokenTable::new();

        globals.insert(
            "saida",
            Symbol {
                pos: TokenPos::default(),
                typ: Type::Void,
            },
        );

        globals.insert(
            "entrada",
            Symbol {
                pos: TokenPos::default(),
                typ: Type::Void,
            },
        );

        Parser {
            tokens: tokens.into_iter().peekable(),
            symbols: vec![globals],
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

    fn set_symbol(&mut self, name: &'a str, pos: TokenPos, typ: Type) {
        let scope = self.symbols.last_mut().unwrap();
        scope.insert(name, Symbol { pos, typ });
    }

    fn get_symbol(&mut self, name: &str) -> Option<&Symbol> {
        let scope = self.symbols.last_mut().unwrap();
        scope.get(name)
    }

    fn find_symbol(&mut self, name: &str) -> Option<&Symbol> {
        self.symbols.iter().rev().find_map(|scope| scope.get(name))
    }

    fn enter_scope(&mut self) {
        self.symbols.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.symbols.pop();
    }

    fn consume_invariant(&mut self, expected: Token<'a>) -> Result<()> {
        let TokenDef { tok, pos } = self.advance()?;
        if tok == expected {
            Ok(())
        } else {
            Err(SyntaxError {
                pos,
                msg: format!("esperado {}, encontrou {}", expected, tok),
            })
        }
    }

    fn consume_identifier(&mut self) -> Result<&'a str> {
        let TokenDef { tok, pos } = self.advance()?;
        if let Token::Identificador(ident) = tok {
            Ok(ident)
        } else {
            Err(SyntaxError {
                pos,
                msg: format!("esperado identificador, encontrou {}", tok),
            })
        }
    }

    fn consume_literal(&mut self) -> Result<Literal<'a>> {
        let TokenDef { tok, pos } = self.advance()?;
        if let Token::Literal(literal) = tok {
            Ok(literal)
        } else {
            Err(SyntaxError {
                pos,
                msg: format!("esperado literal, encontrou {}", tok),
            })
        }
    }

    fn consume_operator(&mut self) -> Result<Operador> {
        let TokenDef { tok, pos } = self.advance()?;
        if let Token::Operador(operator) = tok {
            Ok(operator)
        } else {
            Err(SyntaxError {
                pos,
                msg: format!("esperado operador, encontrou {:?}", tok),
            })
        }
    }

    fn parse_statement(&mut self) -> Result<SyntaxTree<'a>> {
        let token_ref = self.peek().unwrap();
        let pos = token_ref.pos.clone();

        let stmt = match token_ref.tok {
            Token::Seja | Token::Inteiro | Token::Real | Token::Booleano | Token::Texto => {
                let decl = self.advance()?;
                let idt = self.consume_identifier()?;

                if self.get_symbol(idt).is_some() {
                    Err(SyntaxError {
                        msg: format!("redeclaração da variável {idt}"),
                        pos: decl.pos.clone(),
                    })?
                }

                if let Some(TokenDef {
                    tok: Token::Operador(Operador::Atrib),
                    pos: _,
                }) = self.peek()
                {
                    self.advance()?;
                    let exp_pos = self.peek().and_then(|v| Some(v.pos.clone()));
                    let exp = self.parse_expression(1)?;

                    let typ = match decl.tok {
                        Token::Seja => exp.get_type(),
                        Token::Inteiro => Type::Integer,
                        Token::Real => Type::Real,
                        Token::Texto => Type::Text,
                        Token::Booleano => Type::Boolean,
                        _ => unreachable!(),
                    };

                    let exp_typ = exp.get_type();
                    if typ != exp_typ {
                        Err(SyntaxError {
                            msg: format!("{exp_typ} não pode ser convertido para {typ}"),
                            pos: exp_pos.unwrap(),
                        })?
                    }

                    self.set_symbol(idt, pos.clone(), typ.clone());

                    SyntaxTree::Assign { pos, idt, exp, typ }
                } else {
                    let (typ, ini) = match decl.tok {
                        Token::Seja => Err(SyntaxError {
                            msg: format!("declarador seja não pode ser usado sem inicializador"),
                            pos: decl.pos,
                        })?,
                        Token::Inteiro => (Type::Integer, Literal::Inteiro(0)),
                        Token::Real => (Type::Real, Literal::Decimal(0.0)),
                        Token::Texto => (Type::Text, Literal::Texto("")),
                        Token::Booleano => (Type::Boolean, Literal::Booleano(false)),
                        _ => unreachable!(),
                    };
                    let exp = Expression::Literal(ini);

                    self.set_symbol(idt, pos.clone(), typ.clone());

                    SyntaxTree::Assign { pos, idt, exp, typ }
                }
            }
            Token::Enquanto => {
                self.consume_invariant(Token::Enquanto)?;
                let exp = self.parse_expression(1)?;
                self.consume_invariant(Token::Repetir)?;
                let blk = self.parse_block()?;
                SyntaxTree::EnquantoStmt { exp, blk }
            }
            Token::Se => {
                self.consume_invariant(Token::Se)?;
                let exp = self.parse_expression(1)?;
                self.consume_invariant(Token::Entao)?;
                let blk = self.parse_block()?;
                SyntaxTree::SeStmt { exp, blk }
            }
            Token::Para => {
                self.consume_invariant(Token::Para)?;
                let idt = self.consume_identifier()?;

                if let Some(Symbol { pos, typ }) = self.find_symbol(idt) {
                    if *typ != Type::Integer {
                        Err(SyntaxError {
                            msg: format!("variavel {idt} não é do tipo inteiro"),
                            pos: pos.clone(),
                        })?
                    }
                } else {
                    self.set_symbol(idt, pos.clone(), Type::Integer);
                }

                let TokenDef { tok, pos } = self.advance()?;
                let sta = match tok {
                    Token::Operador(Operador::Atrib) => {
                        let sta = self.consume_literal()?;
                        self.consume_invariant(Token::Ate)?;
                        Some(sta)
                    }
                    Token::Ate => None,
                    _ => Err(SyntaxError {
                        msg: format!("esperado atribuição ou ate, encontrou {tok}"),
                        pos,
                    })?,
                };

                let lmt = self.consume_literal()?;

                let TokenDef { tok, pos } = self.advance()?;

                let stp = match tok {
                    Token::Incremento => {
                        let stp = self.consume_literal()?;
                        self.consume_invariant(Token::Repetir)?;
                        Some(stp)
                    }
                    Token::Repetir => None,
                    _ => Err(SyntaxError {
                        msg: format!("esperado incremento ou repetir, encontrou {tok}"),
                        pos,
                    })?,
                };

                let blk = self.parse_block()?;
                SyntaxTree::ParaStmt {
                    idt,
                    sta,
                    lmt,
                    stp,
                    blk,
                }
            }
            Token::Funcao => todo!(),
            Token::Retorne => todo!(),
            Token::Identificador(..) | Token::Literal(..) | Token::Delimitador(..) => {
                let expression = self.parse_expression(1)?;
                SyntaxTree::Expr(expression)
            }
            _ => {
                let msg = format!("token inesperado {}", token_ref.tok);
                Err(SyntaxError { msg, pos })?
            }
        };

        Ok(stmt)
    }

    fn parse_atrib(
        &mut self,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        ope: Operador,
    ) -> std::result::Result<Expression<'a>, String> {
        if let Expression::Identifier(..) = &lhs {
            let lhs_typ = lhs.get_type();
            let rhs_typ = rhs.get_type();

            let mut right = rhs;

            let typ = match (&lhs_typ, &rhs_typ) {
                (x, y) if x == y => x.clone(),
                (Type::Integer, Type::Real) => {
                    right = Expression::Cast(Box::new(right), Type::Integer);
                    Type::Integer
                }
                (Type::Real, Type::Integer) => {
                    right = Expression::Cast(Box::new(right), Type::Real);
                    Type::Real
                }
                (Type::Text, Type::Integer | Type::Real | Type::Boolean) => {
                    right = Expression::Cast(Box::new(right), Type::Text);
                    Type::Text
                }
                _ => Err(format!("tipos incompatíveis {lhs_typ} e {rhs_typ}"))?,
            };

            Ok(Expression::BinOp {
                typ,
                ope,
                lhs: Box::new(lhs),
                rhs: Box::new(right),
            })
        } else {
            Err(format!("lado esquerdo deve ser um identificador"))
        }
    }

    fn parse_binop(
        &mut self,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        ope: Operador,
    ) -> std::result::Result<Expression<'a>, String> {
        let lhs_typ = lhs.get_type();
        let rhs_typ = rhs.get_type();

        use Operador::*;
        use Type::*;

        let mut left = lhs;
        let mut right = rhs;

        let result_typ = match (&lhs_typ, &rhs_typ) {
            // default case
            (x, y) if x == y => x.clone(),

            // implicit casts
            (Real, Integer) => {
                right = Expression::Cast(Box::new(right), Real);
                Real
            }
            (Integer, Real) => {
                left = Expression::Cast(Box::new(left), Real);
                Real
            }
            (Text, _) => {
                right = Expression::Cast(Box::new(right), Text);
                Text
            }
            (_, Text) => {
                left = Expression::Cast(Box::new(left), Text);
                Text
            }

            // not supported
            _ => Err(format!("tipos incompatíveis {lhs_typ} e {rhs_typ}"))?,
        };

        let typ = match (&ope, result_typ) {
            (MaiorQue | MenorQue | MaiorIgualQue | MenorIgualQue, Integer | Real) => Boolean,
            (Igual | Diferente | Atrib, _) => Boolean,
            (E | Ou, r @ (Boolean | Integer)) => r.clone(),
            (
                Adic | Subt | Mult | Div | Resto | Exp | AdicAtrib | SubtAtrib | MultAtrib
                | DivAtrib | RestoAtrib | ExpAtrib,
                r @ (Integer | Real),
            ) => r.clone(),
            (Adic | AdicAtrib, Text) => Text,

            _ => Err(format!(
                "operação {ope} não suportada entre {lhs_typ} e {rhs_typ}"
            ))?,
        };

        Ok(Expression::BinOp {
            typ,
            ope,
            lhs: Box::new(left),
            rhs: Box::new(right),
        })
    }

    fn parse_args(&mut self) -> Result<Vec<Expression<'a>>> {
        let open_paren = self.advance()?;

        enum States {
            S1,
            S2,
            S3,
            S4,
        }

        let mut state: States = States::S1;
        let mut arg = Vec::new();
        while let Some(lookahead) = self.peek() {
            match (&state, &lookahead.tok) {
                (States::S1 | States::S2, Token::Delimitador(Delimitador::FParen)) => {
                    _ = self.advance(); // discard parenthesis
                    state = States::S4;
                    break;
                }
                (States::S1 | States::S3, _) => {
                    arg.push(self.parse_expression(1)?);
                    state = States::S2;
                }
                (States::S2, Token::Delimitador(Delimitador::Virgula)) => {
                    _ = self.advance(); // discard comma
                    state = States::S3;
                }
                (States::S2, ..) => Err(SyntaxError {
                    pos: lookahead.pos.clone(),
                    msg: "experado parênteses de fechamento".into(),
                })?,
                (States::S4, ..) => unreachable!(),
            }
        }

        match state {
            States::S4 => Ok(arg),
            _ => Err(SyntaxError {
                pos: open_paren.pos,
                msg: "experado parênteses de fechamento".into(),
            }),
        }
    }

    fn parse_atom(&mut self) -> Result<Expression<'a>> {
        let TokenDef { tok, pos } = self.advance()?;
        let lookahead = self.peek().and_then(|x| Some(&x.tok));

        let expression = match tok {
            Token::Identificador(idt)
                if matches!(lookahead, Some(Token::Delimitador(Delimitador::AParen))) =>
            {
                let arg = self.parse_args()?;

                let symb = self.find_symbol(idt).ok_or_else(|| SyntaxError {
                    pos,
                    msg: format!("função não definida {idt}"),
                })?;

                Expression::Function {
                    idt,
                    arg,
                    ret: symb.typ.clone(),
                }
            }
            Token::Identificador(idt) => {
                let symb = self.find_symbol(idt).ok_or_else(|| SyntaxError {
                    pos,
                    msg: format!("variavel não definida {idt}"),
                })?;

                Expression::Identifier(idt, symb.typ.clone())
            }
            Token::Literal(literal) => Expression::Literal(literal),
            Token::Delimitador(Delimitador::AParen) => {
                let inner_expr = self.parse_expression(1)?;
                self.consume_invariant(Token::Delimitador(Delimitador::FParen))?;
                inner_expr
            }
            _ => Err(SyntaxError {
                pos,
                msg: format!("token inesperado {tok}"),
            })?,
        };
        Ok(expression)
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression<'a>> {
        let mut lhs = self.parse_atom()?;

        loop {
            let Some(def_ope) = self.peek() else {
                break;
            };

            let Token::Operador(ope) = &def_ope.tok else {
                break;
            };
            let OpInfo(prec, assoc) = ope.precedence();

            if prec < min_prec {
                break;
            }

            let pos = def_ope.pos.clone();
            let ope = self.consume_operator()?;
            let min_prec = if let OpAssoc::L = assoc {
                prec + 1
            } else {
                prec
            };

            let rhs = self.parse_expression(min_prec)?;

            lhs = if ope.is_atrib() {
                self.parse_atrib(lhs, rhs, ope)
                    .map_err(|msg| SyntaxError { pos, msg })?
            } else {
                self.parse_binop(lhs, rhs, ope)
                    .map_err(|msg| SyntaxError { pos, msg })?
            }
        }

        Ok(lhs)
    }

    fn parse_block(&mut self) -> Result<Block<'a>> {
        let mut block = Block::new();

        self.enter_scope();
        while let Some(token) = self.peek() {
            if token.tok == Token::Fim {
                break;
            }
            let stmt = self.parse_statement()?;
            block.push_stmt(stmt);
        }
        self.consume_invariant(Token::Fim)?;
        self.exit_scope();

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

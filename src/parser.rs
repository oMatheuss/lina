use crate::error::{Error, Result};
use crate::lexer::Position;
use crate::operator::Operador;
use crate::token::Token;
use crate::value::Valor;
use crate::Lexer;

use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, Valor>,
    depth: usize,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            depth: 0,
        }
    }

    fn get(&self, k: &String, pos: Position) -> Result<&Valor> {
        let val = self.variables.get(k);
        if let Some(val) = val {
            Ok(val)
        } else {
            Err(Error::new(&format!("variavel não definida {k}"), &pos))
        }
    }

    fn get_mut(&mut self, k: &String, pos: Position) -> Result<&mut Valor> {
        let val = self.variables.get_mut(k);
        if let Some(val) = val {
            Ok(val)
        } else {
            Err(Error::new(&format!("variavel não definida {k}"), &pos))
        }
    }
}

pub fn interpret_code(lexer: &mut Lexer, env: &mut Environment) -> Result<()> {
    loop {
        let pos_init = lexer.pos.clone();
        let token = lexer.next()?;
        match token {
            Token::Seja => {
                let ident = get_identificador(lexer)?;
                let value = get_valor(lexer, env)?;

                env.variables.insert(ident, value);
            }
            Token::Identificador(name) => {
                let op = get_operador(lexer)?;

                match op {
                    Operador::Atribuicao => {
                        let rhs = get_valor(lexer, env)?;
                        let lhs = env.get_mut(&name, pos_init)?;

                        *lhs = rhs;
                    }
                    Operador::SomaAtribuicao => {
                        let rhs = get_valor(lexer, env)?;

                        match rhs {
                            Valor::Numero(val2) => {
                                let lhs = env.get_mut(&name, pos_init)?;

                                if let Valor::Numero(val1) = lhs {
                                    *val1 += val2;
                                } else {
                                    Err(Error::new(
                                        &format!("numero não pode ser adicionado a {lhs}"),
                                        &lexer.pos,
                                    ))?;
                                }
                            }
                            Valor::Texto(val) => todo!(),
                            Valor::Booleano(val) => todo!(),
                            Valor::Vetor(val) => todo!(),
                            Valor::Nulo => todo!(),
                        }
                    }
                    Operador::SubtracaoAtribuicao => todo!(),
                    Operador::MultiplicacaoAtribuicao => todo!(),
                    Operador::DivisaoAtribuicao => todo!(),
                    Operador::RestoAtribuicao => todo!(),
                    Operador::AutoAdicao => {
                        let lhs = env.get_mut(&name, pos_init)?;
                        if let Valor::Numero(val) = lhs {
                            *val += 1.0;
                        } else {
                            Err(Error::new(
                                "1 não pode ser adicionado a um tipo não numerico",
                                &lexer.pos,
                            ))?;
                        }
                    }
                    Operador::AutoSubtracao => {
                        let lhs = env.get_mut(&name, pos_init)?;
                        if let Valor::Numero(val) = lhs {
                            *val -= 1.0;
                        } else {
                            Err(Error::new(
                                "1 não pode ser subtraido de um tipo não numerico",
                                &lexer.pos,
                            ))?;
                        }
                    }
                    _ => Err(Error::new(&format!("operador inválido {op}"), &lexer.pos))?,
                }
            }
            Token::Imprima => {
                let value = get_valor(lexer, env)?;
				match value {
					Valor::Numero(value) => print!("{value}"),
					Valor::Texto(value) => print!("{value}"),
					Valor::Booleano(value) => print!("{value}"),
					Valor::Nulo => print!("nulo"),
					_ => todo!()
				}
            }
            Token::Enquanto => {}
            Token::Fim => break,
            Token::FimDoArquivo => break,
            _ => Err(Error::new(&format!("token inesperado {token}"), &pos_init))?,
        }
    }

    Ok(())
}

fn get_operador(lexer: &mut Lexer) -> Result<Operador> {
    let pos = lexer.pos.clone();
    let token = lexer.next()?;
    if let Token::Operador(ope) = token {
        Ok(ope)
    } else {
        Err(Error::new(
            &format!("esperado operador, obteve: {token}"),
            &pos,
        ))
    }
}

fn get_valor(lexer: &mut Lexer, env: &mut Environment) -> Result<Valor> {
    let pos = lexer.pos.clone();
    let token = lexer.next()?;

    match token {
        Token::Valor(value) => Ok(value),
        Token::Identificador(name) => {
            let value = env.get_mut(&name, pos)?.clone();
            Ok(value)
        }
        _ => Err(Error::new(
            &format!("esperava-se um valor, encontrou {token}"),
            &pos,
        ))?,
    }
}

fn get_identificador(lexer: &mut Lexer) -> Result<String> {
    let pos = lexer.pos.clone();
    let token = lexer.next()?;

    if let Token::Identificador(name) = token {
        Ok(name)
    } else {
        Err(Error::new(
            &format!("esperava-se um identificador, encontrou {token}"),
            &pos,
        ))
    }
}

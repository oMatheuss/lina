use crate::error::Error;
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

    pub fn get_val(&self, name: String) -> Option<&Valor> {
        self.variables.get(&name)
    }

    pub fn get_val_mut(&mut self, name: String) -> Option<&mut Valor> {
        self.variables.get_mut(&name)
    }

    pub fn interpret_code(&mut self, lexer: &mut Lexer) -> Result<(), Error> {
        loop {
            match lexer.next()? {
                Token::Seja => {
                    let Token::Identificador(var_name) = lexer.next()? else {
                        Err(Error::new(
                            "esperava-se um identificador após 'seja'",
                            &lexer.pos,
                        ))?
                    };
                    let next_token = lexer.next()?;
                    let new_value = match next_token {
                        Token::Valor(value) => value,
                        Token::Identificador(name) => self
                            .get_val(name)
                            .ok_or_else(|| Error::new("variavel não definida", &lexer.pos))?
                            .clone(),
                        _ => Err(Error::new(
                            &format!("esperava-se um valor, encontrou {next_token}"),
                            &lexer.pos,
                        ))?,
                    };

                    self.variables.insert(var_name, new_value);
                }
                Token::Identificador(name) => {
                    let op = lexer.next_as_operador()?;
                    match op {
                        Operador::Atribuicao => {
                            let next_token = lexer.next()?;

                            let rhs = match next_token {
                                Token::Valor(value) => value,
                                Token::Identificador(name) => self
                                    .get_val(name)
                                    .ok_or_else(|| Error::new("variavel não definida", &lexer.pos))?
                                    .clone(),
                                _ => Err(Error::new(
                                    &format!("esperava-se um valor, encontrou {next_token}"),
                                    &lexer.pos,
                                ))?,
                            };

                            let lhs = self
                                .get_val_mut(name)
                                .ok_or_else(|| Error::new("variavel não definida", &lexer.pos))?;
                            
                            *lhs = rhs;
                        }
                        Operador::Adicao => todo!(),
                        Operador::Subtracao => todo!(),
                        Operador::Multiplicacao => todo!(),
                        Operador::Divisao => todo!(),
                        Operador::Resto => todo!(),
                        Operador::SomaAtribuicao => {
                            let next_token = lexer.next()?;

                            let rhs = match next_token {
                                Token::Valor(value) => value,
                                Token::Identificador(name) => self
                                    .get_val(name)
                                    .ok_or_else(|| Error::new("variavel não definida", &lexer.pos))?
                                    .clone(),
                                _ => Err(Error::new(
                                    &format!("esperava-se um valor, encontrou {next_token}"),
                                    &lexer.pos,
                                ))?,
                            };

                            match rhs {
                                Valor::Numero(val2) => {
                                    let lhs = self.get_val_mut(name).ok_or_else(|| {
                                        Error::new("variavel não definida", &lexer.pos)
                                    })?;
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
                            let lhs = self.get_val_mut(name).unwrap();
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
                            let lhs = self.get_val_mut(name).unwrap();
                            if let Valor::Numero(val) = lhs {
                                *val -= 1.0;
                            } else {
                                Err(Error::new(
                                    "1 não pode ser subtraido de um tipo não numerico",
                                    &lexer.pos,
                                ))?;
                            }
                        }
                        _ => panic!("operador inválido '{op}'"),
                    }
                }
                Token::Imprima => match lexer.next()? {
                    Token::Identificador(name) => match self.variables.get(&name) {
                        Some(value) => println!("{value}"),
                        None => panic!("variavel inexperada '{name}'"),
                    },
                    Token::Valor(value) => println!("{value}"),
                    _ => panic!("token inválido após imprima"),
                },
                Token::Enquanto => {}
                Token::Fim => break,
                Token::FimDoArquivo => break,
                _ => todo!(),
            }
        }

        Ok(())
    }
}

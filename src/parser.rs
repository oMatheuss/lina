use crate::error::Error;
use crate::token::{Operador, Token, Valor};
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
}

pub fn interpret_code(lexer: &mut Lexer, environment: &mut Environment) -> Result<(), Error> {
    loop {
        match lexer.next()? {
            Token::Seja => {
                let Token::Identificador(var_name) = lexer.next()? else {
                    panic!("esperava-se um identificador após 'seja'");
                };
                let next_token = lexer.next()?;
                let new_value = match next_token {
                    Token::Valor(value) => value,
                    Token::Identificador(name) => environment
                        .get_val(name)
                        .ok_or_else(|| lexer.create_error("variavel não definida"))?
                        .clone(),
                    _ => Err(lexer
                        .create_error(&format!("esperava-se um valor, encontrou {next_token}")))?,
                };

                environment.variables.insert(var_name, new_value);
            }
            Token::Identificador(name) => {
                let op = lexer.next_as_operador()?;
                match op {
                    Operador::Atribuicao => {
                        let next_token = lexer.next()?;

                        let rhs = match next_token {
                            Token::Valor(value) => value,
                            Token::Identificador(name) => environment
                                .get_val(name)
                                .ok_or_else(|| lexer.create_error("variavel não definida"))?
                                .clone(),
                            _ => Err(lexer.create_error(&format!(
                                "esperava-se um valor, encontrou {next_token}"
                            )))?,
                        };

                        let lhs = environment.get_val_mut(name).unwrap();

                        if lhs.variant_eq(&rhs) {
                            *lhs = rhs;
                        } else {
                            Err(lexer
                                .create_error(&format!("não é possivel atribuir {rhs} à {lhs}")))?
                        }
                    }
                    Operador::Adicao => todo!(),
                    Operador::Subtracao => todo!(),
                    Operador::Multiplicacao => todo!(),
                    Operador::Divisao => todo!(),
                    Operador::Resto => todo!(),
                    Operador::SomaAtribuicao => todo!(),
                    Operador::SubtracaoAtribuicao => todo!(),
                    Operador::MultiplicacaoAtribuicao => todo!(),
                    Operador::DivisaoAtribuicao => todo!(),
                    Operador::RestoAtribuicao => todo!(),
                    Operador::AutoAdicao => {
                        let lhs = environment.get_val_mut(name).unwrap();
                        if let Valor::Numero(val) = lhs {
                            *val += 1.0;
                        } else {
                            Err(lexer
                                .create_error("1 não pode ser adicionado a um tipo não numerico"))?;
                        }
                    }
                    Operador::AutoSubtracao => {
                        let lhs = environment.get_val_mut(name).unwrap();
                        if let Valor::Numero(val) = lhs {
                            *val -= 1.0;
                        } else {
                            Err(lexer
                                .create_error("1 não pode ser subtraido de um tipo não numerico"))?;
                        }
                    }
                    _ => panic!("operador inválido '{op}'"),
                }
            }
            Token::Imprima => match lexer.next()? {
                Token::Identificador(name) => match environment.variables.get(&name) {
                    Some(value) => println!("{value}"),
                    None => panic!("variavel inexperada '{name}'"),
                },
                Token::Valor(value) => println!("{value}"),
                _ => panic!("token inválido após imprima"),
            },
            Token::Fim => break,
            Token::FimDoArquivo => break,
            _ => todo!(),
        }
    }

    Ok(())
}

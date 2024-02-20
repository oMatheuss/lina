use crate::error::{Result, Error};
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
}

pub fn interpret_code(lexer: &mut Lexer, env: &mut Environment) -> Result<()> {
	loop {
		match lexer.next()? {
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

						let lhs = env.variables
							.get_mut(&name)
							.ok_or_else(|| Error::new("variavel não definida", &lexer.pos))?;
						
						*lhs = rhs;
					}
					Operador::Adicao => todo!(),
					Operador::Subtracao => todo!(),
					Operador::Multiplicacao => todo!(),
					Operador::Divisao => todo!(),
					Operador::Resto => todo!(),
					Operador::SomaAtribuicao => {
						let rhs = get_valor(lexer, env)?;

						match rhs {
							Valor::Numero(val2) => {
								let lhs = env.variables.get_mut(&name).ok_or_else(|| {
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
						let lhs = env.variables.get_mut(&name).unwrap();
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
						let lhs = env.variables.get_mut(&name).unwrap();
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
			Token::Imprima => {
				let value = get_valor(lexer, env)?;
				print!("{value}");
			},
			Token::Enquanto => {}
			Token::Fim => break,
			Token::FimDoArquivo => break,
			_ => todo!(),
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
		Err(Error::new(&format!("esperado operador, obteve: {token}"), &pos))
	}
}

fn get_valor(lexer: &mut Lexer, env: &mut Environment) -> Result<Valor> {
	let pos = lexer.pos.clone();
	let token = lexer.next()?;
	
	match token {
		Token::Valor(value) => Ok(value),
		Token::Identificador(name) => {
			let value = env.variables
			.get(&name)
			.ok_or_else(|| Error::new("variavel não definida", &pos))?
			.clone();
			
			Ok(value)
		},
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
use crate::error::{Error, Result};
use crate::lexer::Position;
use crate::operator::Operador;
use crate::token::Token;
use crate::value::Valor;
use crate::Lexer;

use std::collections::HashMap;
use std::io::Write;

pub struct Environment {
    variables: HashMap<String, Valor>,
    output: Box<dyn Write>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            output: Box::new(std::io::stdout()),
        }
    }

    fn get(&self, k: &String, pos: &Position) -> Result<&Valor> {
        self.variables
            .get(k)
            .ok_or_else(|| Error::new(&format!("variavel não definida {k}"), pos))
    }

    fn get_mut(&mut self, k: &String, pos: &Position) -> Result<&mut Valor> {
        self.variables
            .get_mut(k)
            .ok_or_else(|| Error::new(&format!("variavel não definida {k}"), pos))
    }
}

pub fn interpret_code(lexer: &mut Lexer, env: &mut Environment) -> Result<()> {
    loop {
        let pos_init = lexer.pos.clone();
        let token = lexer.peek()?;
        match token {
            Token::Seja => {
                lexer.next()?; // ignore token seja
                let ident = get_identificador(lexer)?; // read identifier
                let value = get_valor(lexer, env)?; // read value literal or identifier

                env.variables.insert(ident, value); // set value
            }
            Token::Identificador(..) => atribuicao(lexer, env)?,
            Token::Imprima => {
                lexer.next()?; // ignore token imprima
                let value = get_valor(lexer, env)?;
                match value {
                    Valor::Numero(value) => writeln!(env.output, "{value}"),
                    Valor::Texto(value) => writeln!(env.output, "{value}"),
                    Valor::Booleano(value) => writeln!(env.output, "{value}"),
                    Valor::Nulo => writeln!(env.output, "nulo"),
                    _ => todo!(),
                }
                .unwrap()
            }
            Token::Enquanto => {
                lexer.next()?; // ignore token enquanto
            }
            Token::Para => {
                lexer.next()?; // ignore token para
            }
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

fn get_valor(lexer: &mut Lexer, env: &Environment) -> Result<Valor> {
    let pos = lexer.pos.clone();
    let token = lexer.next()?;

    match token {
        Token::Valor(value) => Ok(value),
        Token::Identificador(name) => {
            let value = env.get(&name, &pos)?.clone();
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

fn atribuicao(lexer: &mut Lexer, env: &mut Environment) -> Result<()> {
    let lhs_pos = lexer.pos.clone(); // get lhs position
    let lhs_idt = get_identificador(lexer)?;
    let lhs = env.get_mut(&lhs_idt, &lhs_pos)?; // read left hand side

    let op_pos = lexer.pos.clone();
    let op = get_operador(lexer)?; // read operator

    // check if operator is of auto assigniment and execute
    match op {
        Operador::AutoAdicao => match lhs {
            Valor::Numero(value) => *value += 1.0,
            _ => Err(Error::new(
                &format!("operador {op} só pode ser usado em números"),
                &lhs_pos,
            ))?,
        },
        Operador::AutoSubtracao => match lhs {
            Valor::Numero(value) => *value -= 1.0,
            _ => Err(Error::new(
                &format!("operador {op} só pode ser usado em números"),
                &lhs_pos,
            ))?,
        },
        _ => {} // do nothing
    }

    //let rhs_pos = lexer.pos.clone();
    // TODO: make a expr match
    let rhs = get_valor(lexer, env)?; // read right hand side

    // need to read again because of rust
    let lhs = env.get_mut(&lhs_idt, &lhs_pos)?; // read left hand side

    match op {
        Operador::Atribuicao => *lhs = rhs,
        Operador::SomaAtribuicao => match lhs {
            Valor::Numero(val1) => match rhs {
                Valor::Numero(val2) => *val1 += val2,
                _ => operacao_nao_suportada(lhs, &rhs, &op, &lhs_pos)?,
            },
            Valor::Texto(val1) => match rhs {
                Valor::Numero(val2) => val1.push_str(&val2.to_string()),
                Valor::Texto(val2) => val1.push_str(&val2.to_string()),
                Valor::Booleano(val2) => val1.push_str(&val2.to_string()),
                Valor::Nulo => {}
                _ => operacao_nao_suportada(lhs, &rhs, &op, &lhs_pos)?,
            },
            Valor::Nulo => Err(Error::new("impossivel incrementar um valor nulo", &lhs_pos))?,
            _ => operacao_nao_suportada(lhs, &rhs, &op, &lhs_pos)?,
        },
        Operador::SubtracaoAtribuicao => todo!(),
        Operador::MultiplicacaoAtribuicao => todo!(),
        Operador::DivisaoAtribuicao => todo!(),
        Operador::RestoAtribuicao => todo!(),
        _ => Err(Error::new(&format!("operador inválido {op}"), &op_pos))?,
    };

    Ok(())
}

fn operacao_nao_suportada(lhs: &Valor, rhs: &Valor, op: &Operador, pos: &Position) -> Result<()> {
    Err(Error::new(
        &format!("não é possivel usar o operador {op} entre {lhs} e {rhs}"),
        pos,
    ))
}

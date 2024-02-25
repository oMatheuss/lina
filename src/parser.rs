use crate::ast::{
    Atribuicao, Declaracao, Enquanto, Expressao, Imprima, Incremento, Statement, WithPosition,
};
use crate::error::{Error, Result};
use crate::lexer::Position;
use crate::operator::Operador;
use crate::token::Token;
use crate::value::Valor;
use crate::Lexer;

use std::collections::HashMap;
use std::io::Write;

pub trait TEnvironment {
    fn get(&self, k: &String, pos: &Position) -> Result<&Valor>;
    fn get_mut(&mut self, k: &String, pos: &Position) -> Result<&mut Valor>;
    fn set(&mut self, k: &String, v: Valor) -> Result<()>;
}

pub struct Environment {
    variables: HashMap<String, Valor>,
    pub output: Box<dyn Write>,
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

pub fn next_statement(lexer: &mut Lexer) -> Result<Statement> {
    let pos_init = lexer.pos.clone();
    let token = lexer.peek()?;
    let statement = match token {
        Token::Seja => {
            lexer.next()?; // ignore token seja
            let nome = get_identificador(lexer)?; // read identifier
            if let Some(Token::Operador(Operador::Atribuicao)) = lexer.peek().ok() {
                lexer.next()?; // ignore token igual
                let expressao = find_expr(lexer)?;
                Statement::Declaracao(Declaracao {
                    nome,
                    valor: Some(expressao),
                })
            } else {
                Statement::Declaracao(Declaracao { nome, valor: None })
            }
        }
        Token::Identificador(..) => {
            let nome = get_identificador(lexer)?; // read identifier
            let operador = get_operador(lexer)?;
            if operador.item == Operador::AutoAdicao || operador.item == Operador::AutoSubtracao {
                Statement::Incremento(Incremento { nome })
            } else {
                let expressao = find_expr(lexer)?;
                Statement::Atribuicao(Atribuicao {
                    nome,
                    operador,
                    expressao,
                })
            }
        }
        Token::Imprima => {
            lexer.next()?; // ignore token imprima
            let expressao = find_expr(lexer)?;
            Statement::Imprima(Imprima { expressao })
        }
        Token::Enquanto => {
            lexer.next()?; // ignore token
            let pos = lexer.pos.clone();
            let condicao = WithPosition {
                item: find_expr(lexer)?,
                pos,
            };
            let mut corpo = Vec::new();
            loop {
                match lexer.peek()? {
                    Token::Fim => {
                        lexer.next()?; // ignore token fim
                        break;
                    }
                    _ => corpo.push(next_statement(lexer)?),
                }
            }
            Statement::Enquanto(Enquanto { corpo, condicao })
        }
        Token::Para => {
            lexer.next()?; // ignore token para
            todo!()
        }
        Token::FimDoArquivo => Statement::Fim,
        _ => Err(Error::new(&format!("token inesperado {token}"), &pos_init))?,
    };

    Ok(statement)
}

pub fn interpret_code(lexer: &mut Lexer, env: &mut Environment) -> Result<()> {
    let mut statements = Vec::new();

    loop {
        let statement = next_statement(lexer)?;
        match statement {
            Statement::Fim => break,
            _ => statements.push(statement),
        }
    }

    eval_code(statements, env)?;

    Ok(())
}

fn eval_code(code: Vec<Statement>, env: &mut Environment) -> Result<()> {
    let mut statements = code.into_iter();

    while let Some(statement) = statements.next() {
        match statement {
            Statement::Declaracao(dcl) => {
                if let Some(expr) = dcl.valor {
                    let value = traverse_expr(expr, env)?;
                    env.variables.insert(dcl.nome.item, value);
                } else {
                    env.variables.insert(dcl.nome.item, Valor::Nulo);
                }
            }
            Statement::Atribuicao(atb) => atribuicao(atb, env)?,
            Statement::Incremento(Incremento {
                nome: WithPosition { item, pos },
            }) => match env.get_mut(&item, &pos)? {
                Valor::Numero(val) => *val += 1.0,
                _ => Err(Error::new(&"operacao não suportada", &pos))?,
            },
            Statement::Enquanto(Enquanto { condicao, corpo }) => loop {
                let expr = traverse_expr(condicao.item.clone(), env)?;
                if let Valor::Booleano(result) = expr {
                    if result {
                        eval_code(corpo.clone(), env)?;
                    } else {
                        break;
                    }
                } else {
                    Err(Error::new(&"operacao não suportada", &condicao.pos))?;
                }
            },
            Statement::Imprima(Imprima { expressao }) => {
                let value = traverse_expr(expressao, env)?;
                match value {
                    Valor::Numero(value) => writeln!(env.output, "{value}"),
                    Valor::Texto(value) => writeln!(env.output, "{value}"),
                    Valor::Booleano(value) => writeln!(env.output, "{value}"),
                    Valor::Nulo => writeln!(env.output, "nulo"),
                    _ => todo!(),
                }
                .unwrap();
            }
            Statement::Fim => break,
        }
    }

    Ok(())
}

fn get_operador(lexer: &mut Lexer) -> Result<WithPosition<Operador>> {
    let pos = lexer.pos.clone();
    let token = lexer.next()?;
    if let Token::Operador(item) = token {
        Ok(WithPosition { item, pos })
    } else {
        Err(Error::new(
            &format!("esperado operador, obteve: {token}"),
            &pos,
        ))
    }
}

fn get_identificador(lexer: &mut Lexer) -> Result<WithPosition<String>> {
    let pos = lexer.pos.clone();
    let token = lexer.next()?;

    if let Token::Identificador(name) = token {
        Ok(WithPosition { item: name, pos })
    } else {
        Err(Error::new(
            &format!("esperava-se um identificador, encontrou {token}"),
            &pos,
        ))
    }
}

fn atribuicao(statement: Atribuicao, env: &mut Environment) -> Result<()> {
    let ident = statement.nome;
    let lhs = env.get_mut(&ident.item, &ident.pos)?; // read left hand side

    let op = statement.operador; // read operator

    // check if operator is of auto assigniment and execute
    match op.item {
        Operador::AutoAdicao => match lhs {
            Valor::Numero(value) => *value += 1.0,
            _ => Err(Error::new(
                &format!("operador {} só pode ser usado em números", op.item),
                &op.pos,
            ))?,
        },
        Operador::AutoSubtracao => match lhs {
            Valor::Numero(value) => *value -= 1.0,
            _ => Err(Error::new(
                &format!("operador {} só pode ser usado em números", op.item),
                &op.pos,
            ))?,
        },
        _ => {} // do nothing
    }

    //let rhs_pos = lexer.pos.clone();
    let rhs = traverse_expr(statement.expressao, env)?; // read right hand side

    // need to read again because of rust
    let lhs = env.get_mut(&ident.item, &ident.pos)?; // read left hand side

    match op.item {
        Operador::Atribuicao => *lhs = rhs,
        Operador::SomaAtribuicao => match lhs {
            Valor::Numero(val1) => match rhs {
                Valor::Numero(val2) => *val1 += val2,
                _ => operacao_nao_suportada(lhs, &rhs, &op.item, &ident.pos)?,
            },
            Valor::Texto(val1) => match rhs {
                Valor::Numero(val2) => val1.push_str(&val2.to_string()),
                Valor::Texto(val2) => val1.push_str(&val2.to_string()),
                Valor::Booleano(val2) => val1.push_str(&val2.to_string()),
                Valor::Nulo => {}
                _ => operacao_nao_suportada(lhs, &rhs, &op.item, &ident.pos)?,
            },
            Valor::Nulo => Err(Error::new(
                "impossivel incrementar um valor nulo",
                &ident.pos,
            ))?,
            _ => operacao_nao_suportada(lhs, &rhs, &op.item, &ident.pos)?,
        },
        Operador::SubtracaoAtribuicao => todo!(),
        Operador::MultiplicacaoAtribuicao => todo!(),
        Operador::DivisaoAtribuicao => todo!(),
        Operador::RestoAtribuicao => todo!(),
        _ => Err(Error::new(
            &format!("operador inválido {}", op.item),
            &op.pos,
        ))?,
    };

    Ok(())
}

fn operacao_nao_suportada<T>(lhs: &Valor, rhs: &Valor, op: &Operador, pos: &Position) -> Result<T> {
    Err(Error::new(
        &format!("não é possivel usar o operador {op} entre {lhs} e {rhs}"),
        pos,
    ))
}

fn find_expr(lexer: &mut Lexer) -> Result<Expressao> {
    let pos_lhs = lexer.pos.clone();
    let lhs = lexer.next()?;

    let node_lhs = match lhs {
        Token::Identificador(nome) => Expressao::Var(WithPosition {
            item: nome,
            pos: pos_lhs,
        }),
        Token::Valor(val) => Expressao::Val(WithPosition {
            item: val,
            pos: pos_lhs,
        }),
        _ => Err(Error::new(
            &format!("esperava-se valor ou variavel, encontrou {lhs}"),
            &pos_lhs,
        ))?,
    };

    let pos_op = lexer.pos.clone();
    let Ok(op) = get_operador(lexer) else {
        lexer.pos = pos_op;
        return Ok(node_lhs);
    };

    let node_rhs = find_expr(lexer)?;

    let result = Expressao::Ope(
        Box::new(node_lhs),
        WithPosition {
            item: op.item,
            pos: pos_op,
        },
        Box::new(node_rhs),
    );

    Ok(result)
}

fn traverse_expr(node: Expressao, env: &mut Environment) -> Result<Valor> {
    let result = match node {
        Expressao::Var(WithPosition { item, pos }) => env.get(&item, &pos)?.clone(),
        Expressao::Val(n) => n.item.clone(),
        Expressao::Ope(lhs, WithPosition { item, pos }, rhs) => {
            let result_lhs = traverse_expr(*lhs, env)?;
            let result_rhs = traverse_expr(*rhs, env)?;

            match item {
                Operador::Adicao => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Numero(val1 + val2),
                        Valor::Texto(ref val2) => Valor::Texto(val1.to_string() + &val2.to_owned()),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    Valor::Texto(ref val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Texto(val1.to_owned() + &val2.to_string()),
                        Valor::Texto(val2) => Valor::Texto(val1.to_owned() + &val2),
                        Valor::Nulo => Valor::Texto(val1.to_owned()),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::Subtracao => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Numero(val1 - val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::Multiplicacao => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Numero(val1 * val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::Divisao => todo!(),
                Operador::Resto => todo!(),

                Operador::MaiorQue => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Booleano(val1 > val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::MenorQue => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Booleano(val1 < val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::MaiorIgualQue => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Booleano(val1 >= val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::MenorIgualQue => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Booleano(val1 <= val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::Igual => match result_lhs {
                    Valor::Numero(val1) => match result_rhs {
                        Valor::Numero(val2) => Valor::Booleano(val1 == val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    Valor::Texto(ref val1) => match result_rhs {
                        Valor::Texto(ref val2) => Valor::Booleano(val1 == val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::CondicionalE => match result_lhs {
                    Valor::Booleano(val1) => match result_rhs {
                        Valor::Booleano(val2) => Valor::Booleano(val1 && val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                Operador::CondicionalOu => match result_lhs {
                    Valor::Booleano(val1) => match result_rhs {
                        Valor::Booleano(val2) => Valor::Booleano(val1 || val2),
                        _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                    },
                    _ => operacao_nao_suportada(&result_lhs, &result_rhs, &item, &pos)?,
                },
                _ => Err(Error::new(&format!("operador inválido {item}"), &pos))?,
            }
        }
    };

    Ok(result)
}

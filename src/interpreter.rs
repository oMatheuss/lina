use crate::token::{Operador, Token, TokenDef, Valor};
use std::{cell::RefCell, collections::HashMap, vec::IntoIter};

#[derive(Debug)]
pub struct Scope<'a, 'b> {
    variables: HashMap<&'a str, Valor<'a>>,
    parent: Option<&'a mut Scope<'b, 'b>>,
}

impl<'a, 'b> Scope<'a, 'a> where 'a: 'b {
    pub fn new(parent: Option<&'a mut Scope<'b, 'b>>) -> Self {
        Self { variables: HashMap::new(), parent }
    }

    fn set(&mut self, name: &'a str, value: Valor<'a>) {
        self.variables.insert(name, value);
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Valor<'a>> {
        if let Some(value) = self.variables.get_mut(name) {
            Some(value)
        } else if let Some(parent) = &mut self.parent {
            parent.get_mut(name)
        } else {
            None
        }
    }

    fn get(&self, name: &str) -> Option<&Valor<'b>> {
        if let Some(value) = self.variables.get(name) {
            Some(value)
        } else if let Some(scope) = &self.parent {
            scope.get(name)
        } else {
            None
        }
    }
}

fn execute_stmt<'a, 'b>(tokens: &mut IntoIter<TokenDef<'b>>, scope: &mut Scope<'a, 'b>) where 'a: 'b {
    if let Some(token) = tokens.next() {
        match token.kind {
            Token::Seja => {
                let Token::Identificador(ident) = tokens.next().unwrap().kind else { todo!() };
                let Token::Operador(Operador::Atribuicao) = tokens.next().unwrap().kind else { todo!() };
                let Token::Valor(valor) = tokens.next().unwrap().kind else { todo!() };

                scope.set(ident, valor);
            },
            Token::Faca => todo!(),
            Token::Entao => todo!(),
            Token::Imprima => todo!(),
            Token::Enquanto => {
                let Token::Identificador(ident) = tokens.next().unwrap().kind else { todo!() };
                let Token::Operador(_) = tokens.next().unwrap().kind else { todo!() };
                let Token::Valor(Valor::Numero(num)) = tokens.next().unwrap().kind else { todo!() };
                
                let inner_code = tokens
                    .take_while(|x| x.kind != Token::Fim)
                    .collect::<Vec<TokenDef>>();
                
                loop {
                    let Valor::Numero(var) = scope.get(ident).unwrap().clone() else {todo!()};
                    if var.ge(&num) { break; }
                    execute(inner_code.clone(), scope);
                }
            },
            Token::Se => todo!(),
            Token::Funcao => todo!(),
            Token::Para => todo!(),
            Token::Retorne => todo!(),
            Token::Identificador(_) => todo!(),
            Token::Valor(_) => todo!(),
            Token::Operador(_) => todo!(),
            Token::Fim => todo!(),
        }
    }
}

pub fn execute<'a, 'b>(code: Vec<TokenDef<'b>>, scope: &mut Scope<'a, 'b>) where 'a: 'b {
    let mut tokens: IntoIter<TokenDef<'b>> = code.into_iter();
    let mut inner = Scope::new(Some(scope));
    while tokens.len() > 0 {
        execute_stmt(&mut tokens, &mut inner);
    }
}
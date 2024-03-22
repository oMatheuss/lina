use crate::token::{Token, Valor, Operador};
use crate::Lexer;

use std::collections::HashMap;
use std::io::Write;

pub struct Scope<'a> {
    variables: HashMap<String, Valor<'a>>,
    parent: Option<Box<Scope<'a>>>,
}

impl<'a> Scope<'a> {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
        }
    }

    fn with_parent(parent: Scope<'a>) -> Self {
        Scope {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    fn add_variable(&mut self, name: String, value: Valor<'a>) {
        self.variables.insert(name, value);
    }

    fn get_variable(&'a mut self, name: &str) -> Option<&'a mut Valor<'a>> {
        if let Some(value) = self.variables.get_mut(name) {
            Some(value)
        } else if let Some(parent_scope) = &mut self.parent {
            parent_scope.get_variable(name)
        } else {
            None
        }
    }
}
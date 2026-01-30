use crate::{ast::LiteralValue, interpreter::RuntimeError};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Environment {
    variables: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&LiteralValue> {
        self.variables.get(name)
    }

    pub fn assign(&mut self, name: &str, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.variables.contains_key(name) {
            self.define(name.to_owned(), value);
            return Ok(());
        }

        Err(RuntimeError {
            line: 0,
            message: format!("Undefined variable '{name}'"),
        })
    }
}

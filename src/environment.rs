use crate::{ast::LiteralValue, interpreter::RuntimeError};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn into_enclosing(self) -> Option<Environment> {
        self.enclosing.map(|boxed| *boxed)
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<LiteralValue> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }

        self.enclosing.as_ref()?.get(name)
    }

    pub fn assign(&mut self, name: &str, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_owned(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }

        Err(RuntimeError {
            line: 0,
            message: format!("Undefined variable '{name}'"),
        })
    }
}

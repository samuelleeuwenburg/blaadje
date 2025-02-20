use super::{Blad, BladError};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Blad>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn child_from(env: &Environment) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(env.clone().into()),
        }
    }

    pub fn set(&mut self, key: &str, value: Blad) -> Result<(), BladError> {
        if self.values.get(key).is_some() {
            return Err(BladError::AttemptToRedefineVariable);
        }

        self.values.insert(key.into(), value);

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&Blad> {
        match (self.values.get(key), &self.parent) {
            (None, Some(parent)) => parent.get(key),
            (v, _) => v,
        }
    }

    pub fn values(&self) -> Vec<(&String, &Blad)> {
        self.values.iter().collect()
    }
}

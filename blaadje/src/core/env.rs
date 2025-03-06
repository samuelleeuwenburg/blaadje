use super::{Blad, BladError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Blad>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn child_from(env: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(env),
        }
    }

    pub fn set_parent(&mut self, env: Rc<RefCell<Environment>>) {
        self.parent = Some(env);
    }

    pub fn set(&mut self, key: &str, value: Blad) -> Result<(), BladError> {
        if self.values.get(key).is_some() {
            return Err(BladError::AttemptToRedefineVariable(key.into()));
        }

        self.values.insert(key.into(), value);

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<Blad> {
        match (self.values.get(key), &self.parent) {
            (None, Some(p)) => {
                let parent = p.borrow();
                parent.get(key)
            }
            (v, _) => v.cloned(),
        }
    }

    pub fn values(&self) -> Vec<(&String, &Blad)> {
        self.values.iter().collect()
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Environment")
    }
}

#[cfg(test)]
mod tests {
    use super::super::Literal;
    use super::*;

    #[test]
    fn test_child_scope_should_inherit_root() {
        let root = Rc::new(RefCell::new(Environment::new()));
        let child = Rc::new(RefCell::new(Environment::child_from(root.clone())));

        let _ = root.borrow_mut().set("x", Blad::Literal(Literal::Usize(5)));

        assert_eq!(
            child.borrow().get("x").unwrap(),
            Blad::Literal(Literal::Usize(5))
        );
    }
}

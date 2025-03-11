use super::{Blad, BladError};
use crate::{Channel, Message};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Blad>,
    parent: Option<Rc<RefCell<Environment>>>,
    pub channel: Arc<Mutex<Channel>>,
}

impl Environment {
    pub fn new() -> (Self, Arc<Mutex<Channel>>) {
        let channel = Arc::new(Mutex::new(Channel::new()));

        (
            Self {
                values: HashMap::new(),
                parent: None,
                channel: channel.clone(),
            },
            channel,
        )
    }

    pub fn child_from(env: Rc<RefCell<Environment>>) -> Self {
        let channel = env.borrow().channel.clone();

        Self {
            values: HashMap::new(),
            parent: Some(env),
            channel,
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

    pub fn channel_cast(&mut self, message: Message) {
        let mut channel = self.channel.lock().unwrap();
        channel.send(message);
    }

    pub fn channel_call(&mut self, message: Message) -> Message {
        self.channel_cast(message);

        loop {
            let mut channel = self.channel.lock().unwrap();
            if let Some(response) = channel.take_reply() {
                return response;
            }
        }
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
        let (root, _) = Environment::new();
        let root = Rc::new(RefCell::new(root));
        let child = Rc::new(RefCell::new(Environment::child_from(root.clone())));

        let _ = root.borrow_mut().set("x", Blad::Literal(Literal::Usize(5)));

        matches!(
            child.borrow().get("x").unwrap(),
            Blad::Literal(Literal::Usize(5))
        );
    }
}

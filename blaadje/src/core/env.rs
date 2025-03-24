use super::{Blad, Channel, Error, Screech};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Normal,
    Live,
}

#[derive(Clone)]
pub struct Environment {
    mode: Mode,
    values: HashMap<String, Blad>,
    parent: Option<Arc<Mutex<Environment>>>,
    pub channel: Arc<Mutex<Channel>>,
}

impl Environment {
    pub fn new() -> (Self, Arc<Mutex<Channel>>) {
        let channel = Arc::new(Mutex::new(Channel::new()));

        (
            Self {
                mode: Mode::Normal,
                values: HashMap::new(),
                parent: None,
                channel: channel.clone(),
            },
            channel,
        )
    }

    pub fn child_from(env: Arc<Mutex<Environment>>) -> Self {
        let channel = {
            let env = env.lock().unwrap();
            env.channel.clone()
        };

        Self {
            mode: Mode::Normal,
            values: HashMap::new(),
            parent: Some(env),
            channel,
        }
    }

    pub fn set_parent(&mut self, env: Arc<Mutex<Environment>>) {
        self.parent = Some(env);
    }

    pub fn live_mode(&mut self) {
        self.mode = Mode::Live;
    }

    pub fn set(&mut self, key: &str, value: Blad) -> Result<(), Error> {
        match (self.mode, self.values.get(key)) {
            // Immutability in normal mode
            (Mode::Normal, Some(_)) => Err(Error::AttemptToRedefineVariable(key.into())),
            _ => {
                self.values.insert(key.into(), value);
                Ok(())
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<Blad> {
        match (self.values.get(key), &self.parent) {
            (None, Some(p)) => {
                let parent = p.lock().unwrap();
                parent.get(key)
            }
            (v, _) => v.cloned(),
        }
    }

    pub fn values(&self) -> Vec<(&String, &Blad)> {
        self.values.iter().collect()
    }

    pub fn channel_cast(&mut self, message: Blad) {
        let mut channel = self.channel.lock().unwrap();
        channel.send(message);
    }

    pub fn channel_call(&mut self, message: Blad) -> Result<Blad, Error> {
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
    fn child_scope_should_inherit_root() {
        let (root, _) = Environment::new();
        let root = Arc::new(Mutex::new(root));
        let child = Arc::new(Mutex::new(Environment::child_from(root.clone())));

        {
            let mut env = root.lock().unwrap();
            let _ = env.set("x", Blad::Literal(Literal::Usize(5)));
        }

        let env = child.lock().unwrap();
        assert_eq!(env.get("x").unwrap(), Blad::Literal(Literal::Usize(5)));
    }

    #[test]
    fn immutability() {
        let (mut env, _) = Environment::new();

        matches!(env.set("x", Blad::Literal(Literal::Usize(10))), Ok(_));
        matches!(
            env.set("x", Blad::Unit),
            Err(Error::AttemptToRedefineVariable(_))
        );
    }

    #[test]
    fn live_mode() {
        let (mut env, _) = Environment::new();

        env.set("x", Blad::Literal(Literal::Usize(10))).unwrap();

        env.live_mode();

        env.set("x", Blad::Literal(Literal::Usize(2))).unwrap();

        assert_eq!(env.get("x").unwrap(), Blad::Literal(Literal::Usize(2)));
    }
}

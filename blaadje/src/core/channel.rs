use crate::{Blad, Error};

pub struct Channel {
    messages: Vec<Blad>,
    reply: Option<Result<Blad, Error>>,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            reply: None,
        }
    }

    pub fn send(&mut self, message: Blad) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &[Blad] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn reply(&mut self, result: Result<Blad, Error>) {
        self.reply = Some(result);
    }

    pub fn take_reply(&mut self) -> Option<Result<Blad, Error>> {
        self.reply.take()
    }
}

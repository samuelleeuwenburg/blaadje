use screech::Signal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Message {
    AddOscillator,
    ModuleId(usize),
    GetSignal(usize),
    Signal(Signal),
    AddSignalToMainOut(usize, Signal),
    ModuleNotFound,
}

pub struct Channel {
    messages: Vec<Message>,
    reply: Option<Message>,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            reply: None,
        }
    }

    pub fn send(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn reply(&mut self, message: Message) {
        self.reply = Some(message);
    }

    pub fn take_reply(&mut self) -> Option<Message> {
        self.reply.take()
    }
}

use irc::proto::message;
use std::collections::BTreeSet;

type MessageResult = Result<Message, String>;

#[derive(Debug, PartialEq)]
pub struct Message {
    handled_by: Vec<String>,
    original: message::Message,
    edited: message::Message,
    output: Option<message::Message>,
    exclusive: bool,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Requirements {
    DB,
    SSL,
    Config(String),
}

pub struct Handler(Box<Fn(Message) -> MessageResult>);

impl Handler {
    pub fn new<F>(function: F) -> Self
    where
        F: 'static + Fn(Message) -> MessageResult,
    {
        Handler(Box::new(function))
    }

    pub fn process(&self, msg: Message) -> MessageResult {
        self.0(msg)
    }
}

pub struct Middleware {
    name: String,
    handlers: Vec<Handler>,
    requires: BTreeSet<Requirements>,
}

impl Middleware {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Middleware {
            name: name.into(),
            handlers: vec![],
            requires: BTreeSet::new(),
        }
    }

    pub fn handler(&mut self, handler: Handler) {
        self.handlers.push(handler);
    }

    pub fn process(&mut self, mut msg: Message) -> MessageResult {
        for handler in self.handlers.iter() {
            msg = handler.process(msg)?;
        }
        Ok(msg)
    }

    pub fn requires(&mut self, requires: Requirements) {
        self.requires.insert(requires);
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

use irc::proto::message::Message;

pub enum Requirements {
    DB,
    SSL,
    Config(String),
}

pub enum Handler {
    Active(Box<Fn(Message) -> Option<Message>>),
    Passive(Box<Fn(Message) -> Option<Message>>),
}

impl Handler {
    pub fn active<F>(function: F) -> Self
    where
        F: 'static + Fn(Message) -> Option<Message>,
    {
        Handler::Active(Box::new(function))
    }

    pub fn passive<F>(function: F) -> Self
    where
        F: 'static + Fn(Message) -> Option<Message>,
    {
        Handler::Passive(Box::new(function))
    }
}

pub struct Middleware {
    name: String,
    handlers: Vec<Handler>,
    requires: Vec<Requirements>,
}

impl Middleware {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Middleware {
            name: name.into(),
            handlers: vec![],
            requires: vec![],
        }
    }

    pub fn handler(&mut self, handler: Handler) {
        self.handlers.push(handler);
    }

    pub fn requires(&mut self, requires: Requirements) {
        self.requires.push(requires);
    }
}

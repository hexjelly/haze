use irc::proto::message;

pub type MessageResult = Result<Message, String>;

#[derive(Default, Debug, PartialEq)]
pub struct Message {
    handled_by: Vec<String>,
    original: Option<message::Message>,
    edited: Option<message::Message>,
    output: Option<message::Message>,
    exclusive: bool,
}

impl Message {
    pub fn new() -> Self {
        Message::default()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Requirements {
    DB,
    SSL,
    Config(String),
}

pub trait Middleware {
    fn name(&self) -> &str;
    fn process(&self) -> MessageResult;
    fn requires(&self) -> Option<&[Requirements]>;
}

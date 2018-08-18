use failure::Error;
pub use irc::proto::command::Command;
pub use irc::proto::message::Message as IrcMessage;

pub type MessageResult = Result<Option<String>, Error>;

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
	pub handled_by: Vec<String>,
	pub original: IrcMessage,
	pub chain: Vec<(String, IrcMessage)>,
	pub exclusive: bool,
}

impl Message {
	pub fn from(message: &IrcMessage) -> Self {
		Message {
			handled_by: vec![],
			original: message.clone(),
			chain: vec![],
			exclusive: false,
		}
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Requirements {
	DB,
	SSL,
	Config(Vec<String>),
}

pub trait Middleware {
	fn name(&self) -> String;
	fn process(&self, msg: &mut Message) -> MessageResult;
	fn requires(&self) -> Vec<Requirements>;
}

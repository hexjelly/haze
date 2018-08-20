pub use irc::proto::command::Command;
pub use irc::proto::message::Message as IrcMessage;

pub type MessageResult = Result<Option<String>, MWError>;

#[derive(Debug, Fail)]
pub enum MWError {
	#[fail(display = "Plugin error [{}]", name)]
	ProcessError { name: String, error: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
	pub chain: Vec<(String, IrcMessage)>,
	pub exclusive: bool,
}

impl Message {
	pub fn from(message: &IrcMessage) -> Self {
		Message {
			chain: vec![("Original".into(), message.clone())],
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

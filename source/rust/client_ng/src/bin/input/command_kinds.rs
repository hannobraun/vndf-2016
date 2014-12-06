use std::hash;

use super::Command;


pub static BROADCAST     : Broadcast     = Broadcast;
pub static STOP_BROADCAST: StopBroadcast = StopBroadcast;


pub trait CommandKind {
	fn name(&self) -> &'static str;
	fn parse(&self, args: Option<&str>) -> Result<Command, &'static str>;
}

impl PartialEq for CommandKind + 'static {
	fn eq(&self, other: &CommandKind) -> bool {
		self.name().eq(other.name())
	}
}

impl Eq for CommandKind + 'static {}

impl<H: hash::Writer> hash::Hash<H> for CommandKind + 'static {
	fn hash(&self, hasher: &mut H) {
		self.name().hash(hasher)
	}
}


pub struct Broadcast;

impl CommandKind for Broadcast {
	fn name(&self) -> &'static str {
		"broadcast"
	}

	fn parse(&self, args: Option<&str>) -> Result<Command, &'static str> {
		let message = match args {
			Some(message) => message,
			None          => return Err("Broadcast message is missing"),
		};

		Ok(Command::Broadcast(message.to_string()))
	}
}


pub struct StopBroadcast;

impl CommandKind for StopBroadcast {
	fn name(&self) -> &'static str {
		"stop-broadcast"
	}

	fn parse(&self, args: Option<&str>) -> Result<Command, &'static str> {
		match args {
			Some(_) => return Err("stop-broadcast has no arguments"),
			None    => (),
		}

		Ok(Command::StopBroadcast)
	}
}

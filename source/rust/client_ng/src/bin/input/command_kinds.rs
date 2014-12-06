use super::Command;


pub static BROADCAST     : Broadcast     = Broadcast;
pub static STOP_BROADCAST: StopBroadcast = StopBroadcast;


pub trait CommandKind {
	fn parse(&self, args: Option<&str>) -> Result<Command, &'static str>;
}


pub struct Broadcast;

impl CommandKind for Broadcast {
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
	fn parse(&self, args: Option<&str>) -> Result<Command, &'static str> {
		match args {
			Some(_) => return Err("stop-broadcast has no arguments"),
			None    => (),
		}

		Ok(Command::StopBroadcast)
	}
}

use super::Command;


pub static BROADCAST: Broadcast = Broadcast;


pub struct Broadcast;

impl Broadcast {
	pub fn parse(&self, args: Option<&str>) -> Result<Command, &'static str> {
		let message = match args {
			Some(message) => message,
			None          => return Err("Broadcast message is missing"),
		};

		Ok(Command::Broadcast(message.to_string()))
	}
}

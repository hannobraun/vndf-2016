use client::platform::Input;

use self::command_kinds::{
	CommandKind,
	CommandKinds,
};


pub use self::headless::HeadlessInputReader;
pub use self::reader::InputReader;


mod command_kinds;
mod headless;
mod reader;


pub trait ReadInput {
	fn input(&mut self) -> Input;
}


#[deriving(Clone, Eq, PartialEq, Show)]
pub enum Command {
	Broadcast(String),
	StopBroadcast,
}

impl Command {
	fn parse(kinds: &CommandKinds, full_command: String) -> CommandResult {
		let mut splits = full_command.splitn(1, ' ');
		
		let command = match splits.next() {
			Some(command) =>
				command,
			None =>
				return Err(CommandError::Invalid(
					"Invalid command",
					full_command.clone(),
				)),
		};

		let args = splits.next();

		let kind = match kinds.get(command) {
			Some(kind) =>
				kind,
			None =>
				return Err(CommandError::Invalid(
					"Unknown command",
					full_command.clone()
				)),
		};

		match kind.parse(args, kinds) {
			Ok(command) =>
				Ok(command),
			Err(error) =>
				return Err(CommandError::Invalid(
					error,
					full_command.clone()
				)),
		}
	}
}


pub type CommandResult = Result<Command, CommandError>;


#[deriving(Clone, Eq, PartialEq, Show)]
pub enum CommandError {
	Incomplete(String, Vec<String>),
	Invalid(&'static str, String),
}

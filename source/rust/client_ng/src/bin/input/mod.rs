use std::comm::TryRecvError;
use std::io::stdin;

use self::command_kinds::{
	CommandKind,
	CommandKinds,
};


mod command_kinds;


pub struct Input {
	receiver     : Receiver<char>,
	current      : String,
	command_kinds: CommandKinds,
}

impl Input {
	pub fn new() -> Input {
		let (sender, receiver) = channel();

		spawn(proc() {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_char() {
					Ok(c) =>
						sender.send(c),
					Err(error) =>
						panic!("Error reading from stdint: {}", error),
				}
			}
		});

		Input {
			receiver     : receiver,
			current      : String::new(),
			command_kinds: CommandKinds::new(),
		}
	}

	pub fn read_commands(&mut self) -> Vec<CommandResult> {
		let mut commands = Vec::new();

		loop {
			match self.receiver.try_recv() {
				Ok(c) => {
					if c == '\x7f' { // Backspace
						self.current.pop();
					}
					else if c == '\n' {
						commands.push(Command::parse(
							&self.command_kinds,
							self.current.clone(),
						));
						self.current.clear();
					}
					else if c.is_control() {
						// ignore other control characters
					}
					else {
						self.current.push(c);
					}
				},

				Err(error) => match error {
					TryRecvError::Empty =>
						break,
					TryRecvError::Disconnected =>
						panic!("Channel disconnected"),
				}
			}
		}

		commands.push(Err(CommandError::Incomplete(self.current.clone())));

		commands
	}
}


#[deriving(Show)]
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

		match kind.parse(args) {
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


#[deriving(Show)]
pub enum CommandError {
	Incomplete(String),
	Invalid(&'static str, String),
}

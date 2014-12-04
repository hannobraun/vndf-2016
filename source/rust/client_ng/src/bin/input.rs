use std::comm::TryRecvError;
use std::io::stdin;


pub struct Input {
	receiver: Receiver<char>,
	current : String,
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
			receiver: receiver,
			current : String::new(),
		}
	}

	pub fn read_commands(&mut self) -> Vec<Command> {
		let mut commands = Vec::new();

		loop {
			match self.receiver.try_recv() {
				Ok(c) => {
					if c == '\n' {
						commands.push(Command::parse(self.current.clone()));
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

		commands.push(Command::Incomplete(self.current.clone()));

		commands
	}
}


#[deriving(Show)]
pub enum Command {
	Broadcast(String),
	StopBroadcast,

	Incomplete(String),
	Invalid(&'static str, String),
}

impl Command {
	fn parse(full_command: String) -> Command {
		let mut splits = full_command.splitn(1, ' ');
		
		let command = match splits.next() {
			Some(command) =>
				command,
			None =>
				return Command::Invalid(
					"Invalid command",
					full_command.clone(),
				),
		};

		let args = splits.next();

		match command {
			"broadcast" => {
				let message = match args {
					Some(message) =>
						message,
					None =>
						return Command::Invalid(
							"Broadcast message is missing",
							full_command.clone(),
						),
				};

				Command::Broadcast(message.to_string())
			},
			"stop-broadcast" => {
				match args {
					Some(_) =>
						return Command::Invalid(
							"stop-broadcast has no arguments",
							full_command.clone()
						),
					None =>
						(),
				}

				Command::StopBroadcast
			}

			_ =>
				Command::Invalid("Unknown command", full_command.clone()),
		}
	}
}

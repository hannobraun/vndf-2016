use std::comm::TryRecvError;
use std::io::stdin;


pub struct Input {
	receiver: Receiver<String>,
}

impl Input {
	pub fn new() -> Input {
		let (sender, receiver) = channel();

		spawn(proc() {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_line() {
					Ok(line) =>
						sender.send(line[.. line.len() - 1].to_string()),
					Err(error) =>
						panic!("Error reading from stdint: {}", error),
				}
			}
		});

		Input {
			receiver: receiver,
		}
	}

	pub fn read_commands(&self) -> Vec<Command> {
		let mut commands = Vec::new();

		loop {
			match self.receiver.try_recv() {
				Ok(line) =>
					commands.push(Command::parse(line)),

				Err(error) => match error {
					TryRecvError::Empty =>
						break,
					TryRecvError::Disconnected =>
						panic!("Channel disconnected"),
				}
			}
		}

		commands
	}
}


pub enum Command {
	Broadcast(String),
}

impl Command {
	fn parse(command: String) -> Command {
		let mut splits = command.splitn(1, ' ');
		
		let command = match splits.next() {
			Some(command) => command,
			// TODO: Handle error
			None          => panic!("Invalid command"),
		};

		let args = splits.next();

		match command {
			"broadcast" => {
				let message = match args {
					Some(message) => message,
					// TODO: Handle error
					None          => panic!("Broadcast message is missing")
				};

				Command::Broadcast(message.to_string())
			},

			_ =>
				// TODO: Handle error
				panic!("Unknown command")
		}
	}
}

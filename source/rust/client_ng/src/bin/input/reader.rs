use std::comm::TryRecvError;
use std::io::stdin;

use client::input::Input;

use super::{
	Command,
	CommandError,
	ReadInput,
};
use super::command_kinds::CommandKinds;


pub struct InputReader {
	receiver     : Receiver<char>,
	current      : String,
	command_kinds: CommandKinds,
	start_with   : Vec<String>,
	broadcast    : Option<String>,
}

impl InputReader {
	pub fn new() -> InputReader {
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
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		InputReader {
			receiver     : receiver,
			current      : String::new(),
			command_kinds: CommandKinds::new(),
			start_with   : Vec::new(),
			broadcast    : None,
		}
	}
}

impl ReadInput for InputReader {
	fn input(&mut self) -> Input {
		let mut commands = Vec::new();

		loop {
			match self.receiver.try_recv() {
				Ok(c) => {
					if c == '\x7f' { // Backspace
						self.current.pop();
					}
					else if c == '\x09' { // Tab
						if self.start_with.len() == 1 {
							self.current = self.start_with[0].clone();
							self.current.push(' ');
						}
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

		self.start_with = self.command_kinds
			.start_with(self.current.as_slice())
			.iter()
			.map(|kind|
				kind.name().to_string()
			)
			.collect();

		commands.push(Err(CommandError::Incomplete(
			self.current.clone(),
			self.start_with.clone(),
		)));

		let mut invalid_error = None;
		for command in commands.into_iter() {
			let command = match command {
				Ok(command) =>
					command,
				Err(error) => match error {
					CommandError::Invalid(error, command) => {
						invalid_error = Some((error.to_string(), command));
						continue;
					},
					_ =>
						continue,
				},
			};

			match command {
				Command::Broadcast(message) =>
					self.broadcast = Some(message),
				Command::StopBroadcast =>
					self.broadcast = None,
			}
		}

		Input {
			broadcast: self.broadcast.clone(),
			command  : (self.current.clone(), self.start_with.clone()),
			error    : invalid_error,
		}
	}
}

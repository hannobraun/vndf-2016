use std::old_io::{
	stdin,
	IoResult,
};
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread;
use std::vec::Drain;

use client::interface::{
	Frame,
	InputEvent,
	Message,
};


pub struct Cli {
	events      : Vec<InputEvent>,
	lines       : Receiver<String>,
	last_message: Message,
}

impl Cli {
	pub fn new() -> Cli {
		let (sender, receiver) = channel();

		thread::spawn(move || {
			let mut stdin = stdin();

			loop {
				match stdin.read_line() {
					Ok(line) =>
						match sender.send(line) {
							Ok(()) =>
								(),
							Err(error) =>
								panic!("Error sending line: {:?}", error),
						},
					Err(error) =>
						panic!("Error reading line: {:?}", error),
				}
			}
		});

		print!("VNDF Ship Control System\n");
		print!("Enter command\n");

		Cli {
			events      : Vec::new(),
			lines       : receiver,
			last_message: Message::None,
		}
	}

	pub fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>> {
		if frame.message != self.last_message {
			match frame.message {
				Message::Notice(ref message) => print!("Notice: {}\n", message),
				Message::Error(ref message)  => print!("Error: {}\n", message),
				Message::None            => (),
			}

			self.last_message = frame.message.clone();
		}

		loop {
			match self.lines.try_recv() {
				Ok(line) => {
					self.handle_line(line.trim_right_matches('\n'), frame)
				},

				Err(error) => match error {
					TryRecvError::Empty =>
						break,
					TryRecvError::Disconnected =>
						panic!("Channel disconnected"),
				}
			}
		}

		Ok(self.events.drain())
	}

	fn handle_line(&mut self, line: &str, frame: &Frame) {
		let mut splits = line.splitn(1, ' ');

		let command = splits.next().unwrap();
		let args    = splits.next().unwrap_or("");

		match command {
			"list-broadcasts" => {
				print!("{} broadcasts\n", frame.broadcasts.len());
				for broadcast in &frame.broadcasts {
					print!("{}: {}\n", broadcast.sender, broadcast.message);
				}
			},
			"start-broadcast" => {
				self.events.push(InputEvent::StartBroadcast(args.to_string()));
			},
			"stop-broadcast" => {
				self.events.push(InputEvent::StopBroadcast);
			},
			"nav-data" => {
				print!(
					"Position: ({}, {}); Velocity: ({}, {})\n",
					frame.position.x, frame.position.y,
					frame.velocity.x, frame.velocity.y,
				);
			},

			_ => print!("Unknown command: {}\n", command),
		}
	}
}

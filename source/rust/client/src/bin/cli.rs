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
use render::Screen;


pub struct Cli {
	events      : Vec<InputEvent>,
	input       : Receiver<char>,
	input_buffer: String,
	last_message: Message,
	text        : Vec<String>,
	screen      : Screen,
	height      : u16,
}

impl Cli {
	pub fn new() -> IoResult<Cli> {
		let (sender, receiver) = channel();

		thread::spawn(move || {
			let mut stdin = stdin();

			loop {
				match stdin.read_char() {
					Ok(c) =>
						match sender.send(c) {
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

		let mut text = Vec::new();
		text.push(format!("VNDF Ship Control System"));
		text.push(format!("Enter command"));

		let width  = 80;
		let height = 24;

		let screen = try!(Screen::new(width, height));

		Ok(Cli {
			events      : Vec::new(),
			input       : receiver,
			input_buffer: String::new(),
			last_message: Message::None,
			text        : text,
			screen      : screen,
			height      : height,
		})
	}

	pub fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>> {
		self.screen.cursor(None);

		if frame.message != self.last_message {
			match frame.message {
				Message::Notice(ref message) => self.text.push(format!("Notice: {}", message)),
				Message::Error(ref message)  => self.text.push(format!("Error: {}", message)),
				Message::None            => (),
			}

			self.last_message = frame.message.clone();
		}

		loop {
			match self.input.try_recv() {
				Ok(c) => {
					if c == '\n' {
						let command = self.input_buffer.clone();
						self.input_buffer.clear();

						try!(self.handle_line(
							command.as_slice(),
							frame,
						));
					}
					else {
						self.input_buffer.push(c)
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

		while self.text.len() > (self.height - 2) as usize {
			self.text.remove(0);
		}

		for (y, line) in self.text.iter().enumerate() {
			try!(
				self.screen
					.buffer()
					.writer(0, y as u16)
					.write_str(line.as_slice())
			);
		}

		try!(
			self.screen
				.buffer()
				.writer(0, self.height - 1)
				.write_str("> ")
			);
		try!(
			self.screen
				.buffer()
				.writer(2, self.height - 1)
				.write_str(self.input_buffer.as_slice())
		);
		self.screen.cursor(
			Some(((self.input_buffer.len() + 2) as u16, self.height -1))
		);

		try!(self.screen.submit());

		Ok(self.events.drain())
	}

	fn handle_line(&mut self, line: &str, frame: &Frame) -> IoResult<()> {
		let mut splits = line.splitn(1, ' ');

		let command = splits.next().unwrap();
		let args    = splits.next().unwrap_or("");

		match command {
			"list-broadcasts" => {
				self.text.push(format!("{} broadcasts", frame.broadcasts.len()));
				for broadcast in &frame.broadcasts {
					self.text.push(format!("{}: {}", broadcast.sender, broadcast.message));
				}
			},
			"start-broadcast" => {
				self.events.push(InputEvent::StartBroadcast(args.to_string()));
			},
			"stop-broadcast" => {
				self.events.push(InputEvent::StopBroadcast);
			},
			"nav-data" => {
				self.text.push(format!(
					"Position: ({}, {}); Velocity: ({}, {})\n",
					frame.position.x, frame.position.y,
					frame.velocity.x, frame.velocity.y,
				));
			},

			_ => self.text.push(format!("Unknown command: {}\n", command)),
		}

		Ok(())
	}
}

use std::io::{
	self,
	stdin,
};
use std::io::prelude::*;
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread;

use client::interface::{
	Frame,
	InputEvent,
	Message,
};
use render::{
	Renderer,
	Screen,
};
use window::Window;


pub struct Cli {
	input       : Receiver<char>,
	input_buffer: String,
	last_message: Message,
	text        : Vec<String>,
	screen      : Screen,
	height      : u16,

	pub renderer: Renderer,
}

impl Cli {
	pub fn new(window: &Window) -> io::Result<Cli> {
		let (sender, receiver) = channel();

		thread::spawn(move || {
			let stdin = stdin().chars();

			for c in stdin {
				match c {
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

		let renderer = Renderer::new(
			window.new_device(),
			window.width(),
			window.height(),
		);

		Ok(Cli {
			input       : receiver,
			input_buffer: String::new(),
			last_message: Message::None,
			text        : text,
			screen      : screen,
			height      : height,
			renderer    : renderer,
		})
	}

	pub fn update(&mut self, events: &mut Vec<InputEvent>, frame: &Frame) -> io::Result<()> {
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
							events,
							command.as_slice(),
							frame,
						));
					}
					else if c == '\x7f' { // Backspace
						self.input_buffer.pop();
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
			try!(write!(
				&mut self.screen.buffer().writer(0, y as u16),
				"{}",
				line,
			));
		}

		// TODO: Consolidate the following two write calls into one.
		try!(write!(
			&mut self.screen.buffer().writer(0, self.height - 1),
			"> ",
		));
		try!(write!(
			&mut self.screen.buffer().writer(2, self.height - 1),
			"{}",
			self.input_buffer.as_slice(),
		));
		self.screen.cursor(
			Some(((self.input_buffer.len() + 2) as u16, self.height -1))
		);

		self.renderer.render(self.screen.buffer());
		try!(self.screen.submit());

		Ok(())
	}

	fn handle_line(&mut self, events: &mut Vec<InputEvent>, line: &str, frame: &Frame) -> io::Result<()> {
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
				events.push(InputEvent::StartBroadcast(args.to_string()));
			},
			"stop-broadcast" => {
				events.push(InputEvent::StopBroadcast);
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

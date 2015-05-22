use std::io;

use glutin::VirtualKeyCode;
use glutin::ElementState::Pressed;
use glutin::Event::{
	KeyboardInput,
	ReceivedCharacter,
};

use client::interface::{
	Frame,
	InputEvent,
	Message,
};
use window::Window;


pub struct Cli {
	input_buffer: String,
	last_message: Message,
	text        : Vec<String>,
	height      : u16,
}

impl Cli {
	pub fn new() -> Cli {
		let mut text = Vec::new();
		text.push(format!("VNDF Ship Control System"));
		text.push(format!("Enter command"));

		let height = 24;

		Cli {
			input_buffer: String::new(),
			last_message: Message::None,
			text        : text,
			height      : height,
		}
	}

	pub fn update(&mut self, events: &mut Vec<InputEvent>, frame: &Frame, window: &Window) -> io::Result<()> {
		if frame.message != self.last_message {
			match frame.message {
				Message::Notice(ref message) => self.text.push(format!("Notice: {}", message)),
				Message::Error(ref message)  => self.text.push(format!("Error: {}", message)),
				Message::None            => (),
			}

			self.last_message = frame.message.clone();
		}

		for event in window.poll_events() {
			match event {
				ReceivedCharacter(c) =>
					if !c.is_control() {
						self.input_buffer.push(c)
					},

				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Back)) => {
					self.input_buffer.pop();
				},
				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Return)) => {
					let command = self.input_buffer.clone();
					self.input_buffer.clear();

					try!(self.handle_line(
						events,
						command.as_ref(),
						frame,
					));
				},

				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) =>
					events.push(InputEvent::Quit),

				_ => (), // ignore other events
			}
		}

		while self.text.len() > (self.height - 2) as usize {
			self.text.remove(0);
		}

		Ok(())
	}

	pub fn text(&self) -> &[String] {
		self.text.as_ref()
	}

	pub fn input(&self) -> &str {
		self.input_buffer.as_ref()
	}

	fn handle_line(&mut self, events: &mut Vec<InputEvent>, line: &str, frame: &Frame) -> io::Result<()> {
		self.text.push(format!("> {}", line));

		let mut splits = line.splitn(2, ' ');

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
			"comm-data" => {
				self.text.push(format!("Your Comm Id: {}", frame.self_id));
			},

			_ => self.text.push(format!("Unknown command: {}\n", command)),
		}

		Ok(())
	}
}

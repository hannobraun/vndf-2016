use std::io;

use glutin::{
	Event,
	VirtualKeyCode,
};
use glutin::ElementState::Pressed;

use client::interface::{
	Frame,
	InputEvent,
	Message,
};
use render::Renderer;
use window::Window;


// TODO: The name doesn't fit. Either it needs to be renamed, or the call to
//       Renderer needs to be moved somewhere else.
pub struct Cli {
	input_buffer: String,
	last_message: Message,
	text        : Vec<String>,
	height      : u16,
	renderer    : Renderer,
}

impl Cli {
	pub fn new(window: &Window) -> io::Result<Cli> {
		let mut text = Vec::new();
		text.push(format!("VNDF Ship Control System"));
		text.push(format!("Enter command"));

		let height = 24;

		let renderer = Renderer::new(
			window.create_graphics(),
			(window.width() as f32, window.height() as f32),
		);

		Ok(Cli {
			input_buffer: String::new(),
			last_message: Message::None,
			text        : text,
			height      : height,
			renderer    : renderer,
		})
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
				Event::ReceivedCharacter(c) =>
					if !c.is_control() {
						self.input_buffer.push(c)
					},

				Event::KeyboardInput(Pressed, _, Some(VirtualKeyCode::Back)) => {
					self.input_buffer.pop();
				},
				Event::KeyboardInput(Pressed, _, Some(VirtualKeyCode::Return)) => {
					let command = self.input_buffer.clone();
					self.input_buffer.clear();

					try!(self.handle_line(
						events,
						command.as_ref(),
						frame,
					));
				},

				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) =>
					events.push(InputEvent::Quit),

				_ => (), // ignore other events
			}
		}

		while self.text.len() > (self.height - 2) as usize {
			self.text.remove(0);
		}

		self.renderer.render(
			self.text.as_ref(),
			self.input_buffer.as_ref(),
			frame,
		);

		Ok(())
	}

	fn handle_line(&mut self, events: &mut Vec<InputEvent>, line: &str, frame: &Frame) -> io::Result<()> {
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

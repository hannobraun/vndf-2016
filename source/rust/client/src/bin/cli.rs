use glutin::VirtualKeyCode;
use glutin::ElementState::Pressed;
use glutin::Event::{
	Closed,
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
	text        : Vec<String>,
	height      : u16,
}

impl Cli {
	pub fn new() -> Cli {
		let mut text = Vec::new();
		text.push(format!("VNDF Ship Control System"));
		text.push(format!("Type \"help\" to list available commands"));

		let height = 24;

		Cli {
			input_buffer: String::new(),
			text        : text,
			height      : height,
		}
	}

	pub fn update(
		&mut self,
		events: &mut Vec<InputEvent>,
		frame : &Frame,
		window: &Window
	) {
		match frame.message {
			Message::Notice(ref message) => self.text.push(format!("Notice: {}", message)),
			Message::Error(ref message)  => self.text.push(format!("Error: {}", message)),
			Message::None                => (),
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

					self.handle_line(
						events,
						command.as_ref(),
						frame,
					);
				},

				// Those events aren't really related to the CLI. It feels wrong
				// to handle them here.
				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) =>
					events.push(InputEvent::Quit),
				Closed =>
					events.push(InputEvent::Quit),

				_ => (), // ignore other events
			}
		}

		while self.text.len() > (self.height - 2) as usize {
			self.text.remove(0);
		}
	}

	pub fn text(&self) -> &[String] {
		self.text.as_ref()
	}

	pub fn input(&self) -> &str {
		self.input_buffer.as_ref()
	}

	fn handle_line(
		&mut self,
		events: &mut Vec<InputEvent>,
		line  : &str,
		frame : &Frame
	) {
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
				match frame.ship_id {
					Some(ship_id) => {
						let ship = frame.ships[&ship_id];

						self.text.push(format!(
							"Position: ({}, {}); Velocity: ({}, {})\n",
							ship.position.x, ship.position.y,
							ship.velocity.x, ship.velocity.y,
						));
					},
					None => {
						self.text.push(format!("No data available."));
					},
				}
			},
			"comm-data" => {
				let message = match frame.ship_id {
					Some(ref id) => format!("Your Comm Id: {}", id),
					None         => format!("Comm Id is currently unknown"),
				};
				
				self.text.push(message);
			},

			"schedule-maneuver" => {
				let result = scan_fmt!(
					args,
					"{} {}",
					f64, i16
				);

				match result {
					(Some(_), Some(direction_deg)) => {
						let direction_rad = (direction_deg as f64).to_radians();

						events.push(
							InputEvent::ScheduleManeuver(direction_rad)
						)
					},

					_ =>
						self.text.push(format!("Error parsing arguments")),
				}
			},

			"help" => {
				let help = [
					"list-broadcasts - Lists all received broadcasts",
					"start-broadcast <text> - Start sending a broadcast",
					"stop-broadcast - Stop sending the current broadcast",
					"nav-data - Print navigation data",
					"comm-data - Print communication data",
					"schedule-maneuver <degrees> - Schedule a maneuver",
				];

				self.text.push(format!("Available commands:"));
				for line in &help {
					self.text.push(format!("{}", line))
				}
			}

			_ => self.text.push(format!("Unknown command: {}\n", command)),
		}
	}
}

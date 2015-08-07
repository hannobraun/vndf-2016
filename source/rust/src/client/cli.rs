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
use client::window::Window;
use shared::game::ManeuverData;


pub struct Cli {
	input_buffer: String,
	text        : Vec<String>,
	height      : u16,

	cmd_history: Vec<String>,
	cmd_idx: usize, //history index/cursor
	is_cmd_history: bool,
	tmp_cmd_buffer: String, //used to temporarily store typed characters
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
			cmd_history: vec!(),
			cmd_idx: 0,
			is_cmd_history: false,
			tmp_cmd_buffer: String::new(),
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
						self.input_buffer.push(c);
						self.is_cmd_history = false;
					},

				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Back)) => {
					self.input_buffer.pop();
					self.is_cmd_history = false;
				},
				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Return)) => {
					let command = self.input_buffer.clone();
					if command != "" {
						let mut found = (false,0);
						for (i,_cmd) in self.cmd_history.iter().enumerate() {
							if *_cmd == command { found = (true,i); }
						}

						if found.0 { self.cmd_history.swap_remove(found.1); }
						
						self.cmd_history.insert(0,command.clone());
						
						self.input_buffer.clear();
						self.tmp_cmd_buffer.clear();
						self.is_cmd_history = false;
						self.cmd_idx = 0; //optionally reset idx

						self.handle_line(
							events,
							command.as_ref(),
							frame,
						);
					}
				},
				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Up)) => {
					let cmd = self.get_history(true);
					self.input_buffer.clear();
					self.input_buffer.push_str(&cmd);
				},
				KeyboardInput(Pressed, _, Some(VirtualKeyCode::Down)) => {
					let cmd = self.get_history(false);
					self.input_buffer.clear();
					self.input_buffer.push_str(&cmd);
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

	fn get_history (&mut self, rev: bool) -> String {
		if self.cmd_history.len() == 0 { return "".to_string() }

		if (self.cmd_idx < 1) && rev && !self.is_cmd_history {
			self.tmp_cmd_buffer = self.input_buffer.clone();
		}
		
		//shift cursor based on direction
		if self.is_cmd_history {
			if rev {
				if self.cmd_idx < (self.cmd_history.len()-1) {
					self.cmd_idx += 1;
				}
			}
			else {
				if self.cmd_idx > 0 {
					self.cmd_idx -= 1;
				}
				else {
					self.is_cmd_history = false;
					let _tmp = self.tmp_cmd_buffer.clone();
					self.tmp_cmd_buffer.clear();
					return _tmp
				}
			}
		}
		else { self.is_cmd_history = true; }
		
		self.cmd_history[self.cmd_idx].clone()
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
				for (eid,msg) in &frame.broadcasts {
					self.text.push(format!("{}: {}", eid, msg));
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
					"{} {} {}",
					f64, f64, f64
				);

				match result {
					(Some(delay_s), Some(duration_s), Some(direction_deg)) => {
						let direction_rad = (direction_deg as f64).to_radians();

						let game_time_s =
							if let Some(game_time_s) = frame.game_time_s {
								game_time_s
							}
							else {
								self.text.push(format!(
									"{} {}",
									"Server connection not fully established ",
									"yet. Please try again in a moment."
								));
								return;
							};

						// This uses the game time from the last frame as the
						// base time. If this ever turns out to be too
						// imprecise, we could count the time since the last
						// frame arrived.
						let data = ManeuverData {
							start_s   : game_time_s + delay_s,
							duration_s: duration_s,
							angle     : direction_rad,
						};

						events.push(
							InputEvent::ScheduleManeuver(data)
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
					"schedule-maneuver <delay (s)> <duration (s)> <degrees> - Schedule a maneuver",
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

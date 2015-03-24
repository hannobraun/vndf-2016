use std::io::{
	self,
	stdin,
};
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread::spawn;
use std::vec::Drain;

use glutin::Event::KeyboardInput;
use glutin::VirtualKeyCode::Escape;

use cli::Cli;
use client::interface::{
	Frame,
	InputEvent,
};
use ui::Ui;
use window::Window;


pub trait Interface {
	fn new() -> io::Result<Self>;
	fn update(&mut self, frame: &Frame) -> io::Result<Drain<InputEvent>>;
}


pub struct Player {
	ui: Ui,
}

impl Interface for Player {
	fn new() -> io::Result<Player> {
		let ui = try!(Ui::new());

		Ok(Player {
			ui: ui,
		})
	}

	fn update(&mut self, frame: &Frame) -> io::Result<Drain<InputEvent>> {
		self.ui.update(frame)
	}
}


pub struct CommandLine {
	events: Vec<InputEvent>,
	cli   : Cli,
	window: Window,
}

impl Interface for CommandLine {
	fn new() -> io::Result<CommandLine> {
		let window = Window::new();
		let cli    = try!(Cli::new(&window));

		Ok(CommandLine {
			events: Vec::new(),
			cli   : cli,
			window: window,
		})
	}

	fn update(&mut self, frame: &Frame) -> io::Result<Drain<InputEvent>> {
		try!(self.cli.update(&mut self.events, frame, &self.window));

		for event in self.window.poll_events() {
			match event {
				KeyboardInput(_, _, Some(Escape)) =>
					self.events.push(InputEvent::Quit),

				_ => (),
			}
		}

		if self.window.is_closed() {
			self.events.push(InputEvent::Quit);
		}

		self.window.swap_buffers();

		Ok(self.events.drain())
	}
}


pub struct Headless {
	events  : Vec<InputEvent>,
	receiver: Receiver<InputEvent>,
}

impl Interface for Headless {
	fn new() -> io::Result<Headless> {
		let (sender, receiver) = channel();

		spawn(move || -> () {
			let mut stdin = stdin();

			loop {
				let mut line = String::new();
				match stdin.read_line(&mut line) {
					Ok(_) => match InputEvent::from_json(line.as_slice()) {
						Ok(event) =>
							match sender.send(event) {
								Ok(()) =>
									(),
								Err(error) =>
									panic!("Error sending input: {:?}", error),
							},
						Err(error) =>
							panic!("Error decoding input: {:?}", error),
					},
					Err(error) =>
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		Ok(Headless {
			events  : Vec::new(),
			receiver: receiver,
		})
	}

	fn update(&mut self, frame: &Frame) -> io::Result<Drain<InputEvent>> {
		loop {
			match self.receiver.try_recv() {
				Ok(event) =>
					self.events.push(event),
				Err(error) => match error {
					TryRecvError::Empty        => break,
					TryRecvError::Disconnected => panic!("Channel disconnected"),
				}
			}
		}

		print!("{}\n", frame.to_json());

		Ok(self.events.drain())
	}
}

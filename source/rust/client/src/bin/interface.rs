use std::old_io::{
	stdin,
	IoResult,
};
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread::spawn;
use std::vec::Drain;

use client::platform::{
	Frame,
	InputEvent,
};
use ui::Ui;

use super::input::InputReader;


pub trait Interface {
	fn new() -> IoResult<Self>;
	fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>>;
}


pub struct Player {
	input_reader: InputReader,
	ui          : Ui,
	chars       : Vec<char>,
}

impl Interface for Player {
	fn new() -> IoResult<Player> {
		let ui = try!(Ui::new());

		Ok(Player {
			input_reader: InputReader::new(),
			ui          : ui,
			chars       : Vec::new(),
		})
	}

	fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>> {
		self.chars.clear();
		self.input_reader.input(&mut self.chars);
		self.ui.update(frame, self.chars.as_slice())
	}
}


pub struct HeadlessInterface {
	events  : Vec<InputEvent>,
	receiver: Receiver<InputEvent>,
}

impl Interface for HeadlessInterface {
	fn new() -> IoResult<HeadlessInterface> {
		let (sender, receiver) = channel();

		spawn(move || -> () {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_line() {
					Ok(line) => match InputEvent::from_json(line.as_slice()) {
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

		Ok(HeadlessInterface {
			events  : Vec::new(),
			receiver: receiver,
		})
	}

	fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>> {
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

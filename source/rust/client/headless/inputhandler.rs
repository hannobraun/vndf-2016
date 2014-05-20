use std;
use std::comm::{
	Disconnected,
	Empty,
	Receiver
};

use common::io;
use common::io::Input;
use common::physics::Radians;

use error::exit;


pub struct InputHandler {
	input     : Receiver<~str>,
	last_input: Input
}

impl InputHandler {
	pub fn new() -> InputHandler {
		let (sender, receiver) = channel();

		spawn(proc() {
			let mut stdin = std::io::stdin();
			loop {
				match stdin.read_line() {
					Ok(message) => sender.send(message),
					Err(error)  =>
						exit(format!("Error reading from stdin: {}", error))
				}
			}
		});

		InputHandler {
			input: receiver,

			last_input: Input {
				exit    : false,
				attitude: Radians(0.0),
				send    : false
			}
		}
	}
}

impl io::InputHandler for InputHandler {
	fn input(&mut self) -> Input {
		let message = match self.input.try_recv() {
			Ok(message) => message,
			Err(error)  => match error {
				Empty =>
					return self.last_input,

				Disconnected =>
					exit(format!("Error receiving input: {}", error))
			}
		};

		let input = match Input::from_json(message) {
			Ok(input)  => input,
			Err(error) => exit(format!("Error decoding input: {}", error))
		};

		self.last_input = input;

		input
	}
}

use std;
use std::comm::{
	Disconnected,
	Empty,
	Receiver
};

use common::io;
use common::io::Input;

use client::error::exit;


pub struct InputHandler {
	input     : Receiver<String>,
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
						exit(format!("Error reading from stdin: {}", error).as_slice())
				}
			}
		});

		InputHandler {
			input     : receiver,
			last_input: Input::default()
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
					exit(format!("Error receiving input: {}", error).as_slice())
			}
		};

		let input = match Input::from_json(message.as_slice()) {
			Ok(input)  => input,
			Err(error) => exit(format!("Error decoding input: {}", error).as_slice())
		};

		self.last_input = input;

		input
	}
}

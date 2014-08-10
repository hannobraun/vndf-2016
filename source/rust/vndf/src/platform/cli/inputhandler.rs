use std;
use std::comm::{
	Disconnected,
	Empty,
	Receiver
};

use client::error::exit;
use platform::Input;


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

	pub fn input(&mut self) -> Result<Input, String> {
		let message = match self.input.try_recv() {
			Ok(message) => message,
			Err(error)  => match error {
				Empty =>
					return Ok(self.last_input),

				Disconnected =>
					return Err(format!("Error receiving input: {}", error))
			}
		};

		let input = match Input::from_json(message.as_slice()) {
			Ok(input)  => input,
			Err(error) => exit(format!("Error decoding input: {}", error).as_slice())
		};

		self.last_input = input;

		Ok(input)
	}
}

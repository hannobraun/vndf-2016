use std;
use std::comm::{
	Disconnected,
	Empty,
	Receiver
};

use common::headless::Input;

use components::Control;
use entities::Components;
use io;


pub struct InputHandler {
	input: Receiver<~str>
}

impl InputHandler {
	pub fn new() -> InputHandler {
		let (sender, receiver) = channel();

		spawn(proc() {
			let mut stdin = std::io::stdin();
			loop {
				match stdin.read_line() {
					Ok(message) => sender.send(message),
					Err(error)  => fail!("Error reading from stdin: {}", error)
				}
			}
		});

		InputHandler {
			input: receiver
		}
	}
}

impl io::Input for InputHandler {
	fn apply(&self, controls: &mut Components<Control>) -> bool {
		let message = match self.input.try_recv() {
			Ok(message) => message,
			Err(error)  => match error {
				Empty        => return false,
				Disconnected => fail!("Error receiving input: {}", error)
			}
		};

		let input = Input::from_json(message);

		for (_, control) in controls.mut_iter() {
			control.attitude = input.attitude;
			control.send     = input.send;
		}

		false
	}
}

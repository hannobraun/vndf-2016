use std;
use std::comm::{
	Disconnected,
	Empty,
	Receiver
};

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
	fn apply(&self, _: &mut Components<Control>) -> bool {
		let message = match self.input.try_recv() {
			Ok(message) => message,
			Err(error)  => match error {
				Empty        => return false,
				Disconnected => fail!("Error receiving input: {}", error)
			}
		};

		print!("stdin: {}\n", message);

		false
	}
}

use std;
use std::comm::{
	Disconnected,
	Empty,
	Receiver
};

use components::Control;
use entities::Components;
use io;


pub struct Input {
	input: Receiver<~str>
}

impl Input {
	pub fn new() -> Input {
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

		Input {
			input: receiver
		}
	}
}

impl io::Input for Input {
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

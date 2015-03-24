use std::io::stdin;
use std::io::prelude::*;
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread::spawn;


pub struct InputReader {
	receiver: Receiver<char>,
}

impl InputReader {
	pub fn new() -> InputReader {
		let (sender, receiver) = channel();

		spawn(move || -> () {
			let stdin = stdin().chars();

			for c in stdin {
				match c {
					Ok(c) =>
						match sender.send(c) {
							Ok(()) =>
								(),
							Err(error) =>
								panic!(
									"Error sending character: {:?}",
									error,
								),
						},
					Err(error) =>
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		InputReader {
			receiver: receiver,
		}
	}

	pub fn input(&mut self, chars: &mut Vec<char>) {
		loop {
			match self.receiver.try_recv() {
				Ok(c) =>
					chars.push(c),

				Err(error) => match error {
					TryRecvError::Empty =>
						break,
					TryRecvError::Disconnected =>
						panic!("Channel disconnected"),
				}
			}
		}
	}
}

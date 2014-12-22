use std::comm::TryRecvError;
use std::io::stdin;

use client::platform::Input;

use super::ReadInput;


pub struct HeadlessInputReader {
	last_input: Input,
	receiver  : Receiver<Input>,
}

impl HeadlessInputReader {
	pub fn new() -> HeadlessInputReader {
		let (sender, receiver) = channel();

		spawn(move || {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_line() {
					Ok(line) => match Input::from_json(line.as_slice()) {
						Ok(input) =>
							sender.send(input),
						Err(error) =>
							panic!("Error decoding input: {}\n", error),
					},
					Err(error) =>
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		HeadlessInputReader {
			receiver  : receiver,
			last_input: Input::new(),
		}
	}
}

impl ReadInput for HeadlessInputReader {
	fn input(&mut self) -> Input {
		match self.receiver.try_recv() {
			Ok(input) => {
				self.last_input = input.clone();
				input
			},
			Err(error) => match error {
				TryRecvError::Empty        => self.last_input.clone(),
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}

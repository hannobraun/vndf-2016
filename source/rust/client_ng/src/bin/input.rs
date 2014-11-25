use std::comm::TryRecvError;
use std::io::stdin;


pub struct Input {
	pub receiver: Receiver<String>,
}

impl Input {
	pub fn new() -> Input {
		let (sender, receiver) = channel();

		spawn(proc() {
			let mut stdin = stdin();

			loop {
				match stdin.read_line() {
					Ok(line) =>
						sender.send(line[.. line.len() - 1].to_string()),
					Err(error) =>
						panic!("Error reading from stdint: {}", error),
				}
			}
		});

		Input {
			receiver: receiver,
		}
	}

	pub fn read_line(&self) -> Option<String> {
		match self.receiver.try_recv() {
			Ok(line) => Some(line),

			Err(error) => match error {
				TryRecvError::Empty        => None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}

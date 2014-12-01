use std::comm::TryRecvError;
use std::io::stdin;


pub struct Input {
	receiver: Receiver<String>,
}

impl Input {
	pub fn new() -> Input {
		let (sender, receiver) = channel();

		spawn(proc() {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
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

	pub fn read_line(&self) -> Vec<String> {
		match self.receiver.try_recv() {
			Ok(line) => vec![line],

			Err(error) => match error {
				TryRecvError::Empty        => vec![],
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}

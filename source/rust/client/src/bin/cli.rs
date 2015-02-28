use std::old_io::{
	stdin,
	IoResult,
};
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread;
use std::vec::Drain;

use client::interface::{
	Frame,
	InputEvent,
};


pub struct Cli {
	events: Vec<InputEvent>,
	lines : Receiver<String>,
}

impl Cli {
	pub fn new() -> Cli {
		let (sender, receiver) = channel();

		thread::spawn(move || {
			let mut stdin = stdin();

			loop {
				match stdin.read_line() {
					Ok(line) =>
						match sender.send(line) {
							Ok(()) =>
								(),
							Err(error) =>
								panic!("Error sending line: {:?}", error),
						},
					Err(error) =>
						panic!("Error reading line: {:?}", error),
				}
			}
		});

		Cli {
			events: Vec::new(),
			lines : receiver,
		}
	}

	pub fn update(&mut self, _: &Frame) -> IoResult<Drain<InputEvent>> {
		// TODO: Read lines and display information or generate input events as
		//       appropriate
		loop {
			match self.lines.try_recv() {
				Ok(line) => print!("{}", line),

				Err(error) => match error {
					TryRecvError::Empty =>
						break,
					TryRecvError::Disconnected =>
						panic!("Channel disconnected"),
				}
			}
		}
		Ok(self.events.drain())
	}
}

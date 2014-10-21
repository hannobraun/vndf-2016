use acceptance::Process;

use platform::{
	Frame,
	Input
};


pub struct Client {
	process: Process
}

impl Client {
	pub fn start(port: u16) -> Client {
		let process = Process::start(
			"vndf-client",
			[
				format!("--headless").as_slice(),
				format!("--address=localhost").as_slice(),
				format!("--port={}", port).as_slice(),
				format!("--period=10").as_slice(),
			]
		);

		Client {
			process: process
		}
	}

	pub fn stop(&mut self) {
		self.process.kill();
	}

	pub fn input(&mut self, input: Input) {
		let line = input.to_json();
		self.process.write_stdin_line(line.as_slice());
	}

	pub fn frame(&mut self) -> Frame {
		let line = self.process.read_stdout_line();
		match Frame::from_json(line.as_slice()) {
			Ok(frame)  => frame,
			Err(error) => fail!(
				"Error decoding frame. Error: {}; Frame: {}",
				error, line
			)
		}
	}
}

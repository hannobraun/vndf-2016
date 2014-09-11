use platform::{
	Frame,
	Input
};
use test_infra::Process;


pub struct Client {
	process: Process
}

impl Client {
	pub fn start(port: u16) -> Client {
		let process = Process::start(
			"vndf-client",
			[
				format!("--headless"),
				format!("--address=localhost"),
				format!("--port={}", port),
				format!("--period=10")
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

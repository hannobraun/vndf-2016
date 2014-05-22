use io::{
	Frame,
	Input
};
use testing::Process;


pub struct Client {
	process: Process
}

impl Client {
	pub fn start(port: u16) -> Client {
		let process = Process::start(
			"output/bin/vndf-client",
			[
				format!("--headless"),
				format!("--address=localhost"),
				format!("--port={}", port),
				format!("--period=10")]);

		Client {
			process: process
		}
	}

	pub fn stop(&mut self) {
		self.process.kill();
	}

	pub fn input(&mut self, input: Input) {
		let line = input.to_json();
		self.process.write_stdin_line(line);
	}

	pub fn frame(&mut self) -> Frame {
		let line = self.process.read_stdout_line();
		match Frame::from_json(line) {
			Ok(frame)  => frame,
			Err(error) => fail!("Error decoding frame: {}", error)
		}
	}
}

use std::io::net::ip::Port;

use acceptance::Process;
use client_ng::Frame;


pub struct Client {
	process: Process,
}

impl Client {
	pub fn start(port: Port) -> Client {
		let process = Process::start(
			"vndf-client-ng",
			&[
				format!("--headless").as_slice(),
				format!("--server-port={}", port).as_slice(),
			]
		);

		Client {
			process: process,
		}
	}

	pub fn command(&mut self, _command: &str) {

	}

	pub fn frame(&mut self) -> Frame {
		let line = self.process.read_stdout_line();
		match Frame::from_json(line.as_slice()) {
			Ok(frame)  => frame,
			Err(error) => panic!(
				"Error decoding frame. Error: {}; Frame: {}",
				error, line,
			)
		}
	}

	pub fn wait_while(&mut self, condition: |&Frame| -> bool) -> Frame {
		let mut frame = self.frame();

		while condition(&frame) {
			frame = self.frame();
		}

		frame
	}
}

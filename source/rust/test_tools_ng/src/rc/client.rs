use std::io::net::ip::Port;
use time::precise_time_s;

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
				"--headless",
				"--server-host=localhost",
				format!("--server-port={}", port).as_slice(),
			]
		);

		Client {
			process: process,
		}
	}

	pub fn command(&mut self, command: &str) {
		self.process.write_stdin_line(command);
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
		let start_s= precise_time_s();

		let mut frame = self.frame();

		while condition(&frame) {
			if precise_time_s() - start_s > 0.5 {
				panic!("Condition not satisfied after waiting");
			}

			frame = self.frame();
		}

		frame
	}
}

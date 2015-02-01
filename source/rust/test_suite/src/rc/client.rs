use std::old_io::net::ip::Port;
use time::precise_time_s;

use acceptance::Process;

use client::platform::{
	Frame,
	InputEvent,
};


pub struct Client {
	process: Process,
}

impl Client {
	pub fn start(port: Port) -> Client {
		let process = Process::start(
			"vndf-client",
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

	pub fn stop(&mut self) {
		self.process.kill()
	}

	pub fn input(&mut self, event: InputEvent) {
		self.process.write_stdin_line(event.to_json().as_slice());
	}

	pub fn broadcast(&mut self, broadcast: &str) {
		self.input(InputEvent::StartBroadcast(broadcast.to_string()))
	}

	pub fn stop_broadcast(&mut self) {
		self.input(InputEvent::StopBroadcast)
	}

	pub fn frame(&mut self) -> Frame {
		let line = self.process.read_stdout_line();
		match Frame::from_json(line.as_slice()) {
			Ok(frame)  => frame,
			Err(error) => panic!(
				"Error decoding frame. Error: {:?}; Frame: {}",
				error, line,
			)
		}
	}

	pub fn wait_until<F>(&mut self, condition: F) -> Frame
		where F: Fn(&Frame) -> bool
	{
		let start_s = precise_time_s();

		let mut frame = self.frame();

		while !condition(&frame) {
			if precise_time_s() - start_s > 0.5 {
				panic!("Condition not satisfied after waiting");
			}

			frame = self.frame();
		}

		frame
	}
}

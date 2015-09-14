use time::precise_time_s;

use client::interface::{
	Frame,
	InputEvent,
};
use testing::process::Process;


pub struct Client {
	process: Process,
}

// TODO(5rKZ3HPd): Maybe it would be smarter to remove the specific input
//                 methods (start_broadcast, stop_broadcast) in favor of input.
//                 This would open the door towards factoring out generic test
//                 infrastructure from this.
impl Client {
	pub fn start(port: u16) -> Client {
		let process = Process::start(
			"vndf-client",
			&[
				"--headless",
				"--server-host=localhost",
				format!("--server-port={}", port).as_ref(),
				format!("--network-timeout={}", 0.05).as_ref(),
			]
		);

		Client {
			process: process,
		}
	}

	pub fn input(&mut self, event: InputEvent) {
		self.process.write_stdin_line(event.to_json().as_ref());
	}

	pub fn start_broadcast(&mut self, broadcast: &str) {
		self.input(InputEvent::StartBroadcast(broadcast.to_string()))
	}

	pub fn stop_broadcast(&mut self) {
		self.input(InputEvent::StopBroadcast)
	}

	pub fn frame(&mut self) -> Frame {
		let line = self.process.read_stdout_line();
		match Frame::from_json(line.as_ref()) {
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

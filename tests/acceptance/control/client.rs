use common::headless::Frame;

use util::Process;


pub struct Client {
	process: Process
}

impl Client {
	pub fn start(port: u16) -> Client {
		let process = Process::start(
			"output/bin/vndf-client",
			[
				~"--headless",
				~"--address", ~"localhost",
				~"--port", port.to_str()]);

		Client {
			process: process
		}
	}

	pub fn frame(&mut self) -> Frame {
		let line = self.process.read_stdout_line();
		Frame::from_json(line)
	}
}

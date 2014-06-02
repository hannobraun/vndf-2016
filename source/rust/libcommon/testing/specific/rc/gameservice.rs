use std::rand::random;

use testing::Process;


pub struct GameService {
	pub port   : u16,
	pub process: Process
}

impl GameService {
	pub fn start() -> GameService {
		let port = random::<u16>() % 10000 + 40000;

		let mut process = Process::start(
			"output/bin/vndf-game-service",
			[
				"--port".to_str(), port.to_str(),
				"--frame-time".to_str(), "10".to_str()]);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}

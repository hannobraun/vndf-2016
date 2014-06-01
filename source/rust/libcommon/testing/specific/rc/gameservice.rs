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
				"--port".to_owned(), port.to_str(),
				"--frame-time".to_owned(), "10".to_owned()]);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}

use std::io::net::ip::Port;

use acceptance::{
	random_port,
	Process,
};


pub struct GameService {
	port    : Port,
	_process: Process,
}

impl GameService {
	pub fn start() -> GameService {
		let port = random_port(40000, 50000);

		let mut process = Process::start(
			"vndf-game-service-ng",
			&[
				format!("--port={}", port).as_slice(),
			]
		);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port    : port,
			_process: process,
		}
	}

	pub fn port(&self) -> Port {
		self.port
	}
}

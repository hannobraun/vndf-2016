use rand;

use util::{Process, Update};


pub struct GameService {
	port   : uint,
	process: Process
}

impl GameService {
	pub fn start() -> GameService {
		let port = rand::random::<uint>() % 10000 + 40000;

		let mut process = Process::start(
			"output/bin/vndf-game-service", [port.to_str()]);
		process.read_stdout_line(); // Make sure it's ready.

		GameService {
			port   : port,
			process: process
		}
	}
}


pub struct ClientCore {
	process: Process
}

impl ClientCore {
	pub fn start(port: uint) -> ClientCore {
		ClientCore {
			process: Process::start(
				"output/bin/vndf-client-core", [~"localhost", port.to_str()])
		}
	}

	pub fn expect_update(&mut self) -> Update {
		let message = self.process.read_stdout_line();

		match Update::from_str(message) {
			Some(update) => update,
			None         => fail!("Expected UPDATE but got: {}", message)
		}
	}
}

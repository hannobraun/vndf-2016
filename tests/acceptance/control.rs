use util::{Process, Update};


pub struct GameService {
	process: Process
}

impl GameService {
	pub fn start() -> GameService {
		GameService {
			process: Process::start("output/bin/vndf-game-service", [~"34481"])
		}
	}
}


pub struct ClientCore {
	process: Process
}

impl ClientCore {
	pub fn start() -> ClientCore {
		ClientCore {
			process: Process::start(
				"output/bin/vndf-client-core", [~"localhost", ~"34481"])
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

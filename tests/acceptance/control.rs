use util::Process;


pub struct GameService {
	process: Process
}

impl GameService {
	pub fn start() -> GameService {
		GameService {
			process: Process::start("output/bin/vndf-game-service", [])
		}
	}

	pub fn stop(&mut self) {
		self.process.kill();
	}
}


pub struct ClientCore {
	process: Process
}

impl ClientCore {
	pub fn start() -> ClientCore {
		ClientCore {
			process: Process::start(
				"output/bin/vndf-client-core", [~"localhost"])
		}
	}
}

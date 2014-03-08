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

	pub fn message(&mut self) -> ~str {
		self.process.read_stdout_line()
	}
}

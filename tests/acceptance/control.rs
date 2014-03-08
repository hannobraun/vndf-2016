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

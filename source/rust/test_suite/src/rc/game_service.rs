use process::Process;
use util::random_port;


pub struct GameService {
	port    : u16,
	_process: Process,
}

impl GameService {
	pub fn start() -> GameService {
		let port = random_port(40000, 50000);

		let mut process = Process::start(
			"vndf-game-service",
			&[
				format!("--port={}", port).as_ref(),
				format!("--client-timeout={}", 0.05).as_ref(),
			]
		);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port    : port,
			_process: process,
		}
	}

	pub fn port(&self) -> u16 {
		self.port
	}
}

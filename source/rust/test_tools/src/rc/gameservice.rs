use std::os;
use std::rand::random;

use test_infra::Process;


pub struct GameService {
	pub port   : u16,
	pub process: Process
}

impl GameService {
	pub fn start() -> GameService {
		let port = random::<u16>() % 10000 + 40000;

		let mut initial_state = "".to_string();
		for &(ref key, ref value) in os::env().iter() {
			if *key == "INITIAL_STATE".to_string() {
				initial_state = value.clone();
			}
		}

		let mut process = Process::start(
			"vndf-game-service",
			[
				"--port".to_string()         , port.to_string(),
				"--frame-time".to_string()   , "10".to_string(),
				"--initial-state".to_string(), initial_state,
			]
		);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}

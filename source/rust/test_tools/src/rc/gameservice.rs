use std::os;
use std::rand::random;

use game_service::initialstate::InitialState;
use test_infra::Process;


pub struct GameService {
	pub port   : u16,
	pub process: Process
}

impl GameService {
	pub fn start(initial_state: &InitialState) -> GameService {
		let port = random::<u16>() % 10000 + 40000;

		let mut state_file_name = "initial-state-".to_string();
		state_file_name.push_str(port.to_string().as_slice());
		state_file_name.push_str(".json");

		let mut state_file_path = os::tmpdir();
		state_file_path.push(state_file_name);

		initial_state.to_file(&state_file_path);

		let mut process = Process::start(
			"vndf-game-service",
			[
				"--port"         , port.to_string().as_slice(),
				"--frame-time"   , "10",
				"--initial-state", state_file_path.to_c_str().as_str().unwrap(),
			]
		);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}

use collections::HashSet;
use rand;
use std::intrinsics::TypeId;

use common::physics::Radians;
use common::protocol::{
	Command,
	Message,
	SelfInfo,
	Update
};

use util::Process;


pub struct GameService {
	pub port   : uint,
	pub process: Process
}

impl GameService {
	pub fn start() -> GameService {
		let port = rand::random::<uint>() % 10000 + 40000;

		let mut process = Process::start(
			"output/bin/vndf-game-service",
			[
				~"--port", port.to_str(),
				~"--frame-time", ~"10"]);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}


pub struct ClientCore {
	process: Process,
	ignored: HashSet<TypeId>
}

impl ClientCore {
	pub fn start(port: uint) -> ClientCore {
		ClientCore {
			process: Process::start(
				"output/bin/vndf-client-core", [~"localhost", port.to_str()]),
			ignored: HashSet::new()
		}
	}

	pub fn stop(&mut self) {
		self.process.kill();
	}

	pub fn ignore(&mut self, type_id: TypeId) {
		self.ignored.insert(type_id);
	}

	pub fn expect_self_info(&mut self) -> SelfInfo {
		match self.next_message() {
			SelfInfo(self_info) => self_info,
			message @ _         => fail!("Unexpected message: {}", message)
		}
	}

	pub fn expect_update(&mut self) -> Update {
		match self.next_message() {
			Update(update) => update,
			message @ _    => fail!("unexpected message ({})", message)
		}
	}

	pub fn send_attitude(&mut self, attitude: Radians) {
		let message = Command(Command {
			attitude: attitude
		});
		self.process.write_stdin_line(message.to_str());
	}

	fn next_message(&mut self) -> Message {
		loop {
			let line    = self.process.read_stdout_line();
			let message = Message::from_str(line);

			if !self.ignored.contains(&message.type_id()) {
				return message
			}
		};
	}
}

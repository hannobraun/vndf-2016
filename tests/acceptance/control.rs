use rand;

use common::protocol::{Message, SelfInfo, Update};

use util::Process;


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

	pub fn ignore_message(&mut self) {
		self.process.read_stdout_line();
	}

	pub fn expect_self_id(&mut self) -> uint {
		let message = self.process.read_stdout_line();

		match Message::from_str(message) {
			SelfInfo(self_info) => self_info.id,
			_                   => fail!("unexpected message ({})", message)
		}
	}

	pub fn expect_update(&mut self) -> Update {
		let message = self.process.read_stdout_line();

		match Message::from_str(message) {
			Update(update) => update,
			_              => fail!("unexpected message ({})", message)
		}
	}
}

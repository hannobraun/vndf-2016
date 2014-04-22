use collections::HashSet;
use rand;
use std::intrinsics::TypeId;

use common::physics::Radians;
use common::protocol::{Command, Create, Message, SelfInfo, Update};

use util::Process;


pub struct GameService {
	pub port   : uint,
	pub process: Process
}

impl GameService {
	pub fn start() -> GameService {
		let port = rand::random::<uint>() % 10000 + 40000;

		let mut process = Process::start(
			"output/bin/vndf-game-service", [port.to_str()]);
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

	pub fn ignore(&mut self, type_id: TypeId) {
		self.ignored.insert(type_id);
	}

	pub fn expect_self_id(&mut self) -> uint {
		match self.next_message() {
			SelfInfo(self_info) => self_info.id,
			message @ _         => fail!("unexpected message ({})", message)
		}
	}

	pub fn expect_create(&mut self) -> Create {
		match self.next_message() {
			Create(create) => create,
			message @ _    => fail!("unexpected message ({})", message)
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

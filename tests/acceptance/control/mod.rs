use collections::HashSet;
use std::intrinsics::TypeId;

use common::physics::Radians;
use common::protocol::{
	Command,
	Message,
	Update
};

use util::Process;


pub use self::client::Client;
pub use self::gameservice::GameService;


mod client;
mod gameservice;


pub struct ClientCore {
	process: Process,
	ignored: HashSet<TypeId>
}

impl ClientCore {
	pub fn start(port: u16) -> ClientCore {
		ClientCore {
			process: Process::start(
				"output/bin/vndf-client-core", [~"localhost", port.to_str()]),
			ignored: HashSet::new()
		}
	}

	pub fn ignore(&mut self, type_id: TypeId) {
		self.ignored.insert(type_id);
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

use game::ecs::SharedWorldEntity;
use platform::Input;
use net::Connection;
use protocol::{
	Action,
	Perception
};
use rustecs::EntityId;

use super::error::exit;



pub struct Network {
	conn: Connection
}

impl Network {
	pub fn connect(address: &str, port: &str) -> Network {
		let connection = match Connection::connect(address, port) {
			Ok(connection) => connection,
			Err(error)     =>
				exit(format!("Error connecting to server: {}", error).as_slice())
		};

		Network {
			conn: connection
		}
	}

	pub fn send(&mut self, input: Input) {
		let action = Action {
			attitude: input.attitude,
			thrust  : input.thrust,
			missile : input.missile
		};
		match self.conn.send_message(action.to_string().as_slice()) {
			Ok(())     => (),
			Err(error) => exit(format!("Error sending message: {}", error).as_slice())
		}
	}

	pub fn receive(
		&mut self,
		handler: |Perception<EntityId, SharedWorldEntity>|
	) {
		let result = self.conn.receive_messages(|message| {
			let perception = match Perception::from_string(message.as_slice()) {
				Ok(perception) => perception,

				Err(error) =>
					exit(format!("Error decoding message: {}", error).as_slice())
			};

			handler(perception);
		});

		match result {
			Ok(())     => (),
			Err(error) => exit(format!("Failed to receive message: {}", error).as_slice())
		}
	}
}

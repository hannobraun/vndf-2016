use common::net::Connection;
use common::protocol::{
	Action,
	Perception
};
use common::physics::Radians;

use error::exit;


pub struct Network {
	conn: Connection
}

impl Network {
	pub fn connect(address: &str, port: &str) -> Network {
		let connection = match Connection::connect(address, port) {
			Ok(connection) => connection,
			Err(error)     =>
				exit(format!("Error connecting to server: {}", error))
		};

		Network {
			conn: connection
		}
	}

	pub fn receive(&mut self, handler: |Perception|) {
		let result = self.conn.receive_messages(|message| {
			let perception = match Perception::from_str(message) {
				Ok(perception) => perception,

				Err(error) =>
					exit(format!("Error decoding message: {}", error))
			};

			handler(perception);
		});

		match result {
			Ok(())     => (),
			Err(error) => exit(format!("Failed to receive message: {}", error))
		}
	}

	pub fn send_command(&mut self, attitude: Radians) {
		let action = Action { attitude: attitude };
		match self.conn.send_message(action.to_str()) {
			Ok(())     => (),
			Err(error) => exit(format!("Error sending message: {}", error))
		}
	}
}

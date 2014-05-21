use common::io::Input;
use common::net::Connection;
use common::protocol::{
	Action,
	Perception
};

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

	pub fn send(&mut self, input: Input) {
		let action = Action {
			attitude: input.attitude,
			missile : input.missile
		};
		match self.conn.send_message(action.to_str()) {
			Ok(())     => (),
			Err(error) => exit(format!("Error sending message: {}", error))
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
}

use common::net::Connection;
use common::protocol::{
	Command,
	Create,
	Message,
	Perception,
	Remove
};
use common::physics::Radians;

use entities::Entities;
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

	pub fn receive(&mut self, entities: &mut Entities, handler: |Perception|) {
		let result = self.conn.receive_messages(|raw_message| {
			let message = match Message::from_str(raw_message) {
				Ok(message) => message,
				Err(error)  =>
					exit(format!("Error decoding message: {}", error))
			};

			match message {
				Create(create) =>
					entities.create_ship(
						create.id),

				Perception(update) => {
					handler(update.clone());

					entities.self_id = Some(update.self_id);

					for ship in update.ships.iter() {
						entities.update_ship(
							ship.id,
							ship.body)
					}
				},

				Remove(remove) =>
					entities.remove_ship(
						remove.id),

				_ =>
					exit(format!("Unexpected message: {}", message))
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => exit(format!("Failed to receive message: {}", error))
		}
	}

	pub fn send_command(&mut self, attitude: Radians) {
		let command = Command(Command { attitude: attitude });
		match self.conn.send_message(command.to_str()) {
			Ok(())     => (),
			Err(error) => exit(format!("Error sending message: {}", error))
		}
	}
}

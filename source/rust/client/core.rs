use common::net::Connection;
use common::protocol::{Command, Create, Message, Remove, SelfInfo, Update};
use common::physics::{Radians};

use entities::Entities;


pub struct Core {
	conn: Connection,
	id  : Option<uint>
}

impl Core {
	pub fn start(address: &str, port: &str) -> Core {
		let connection = match Connection::connect(address, port) {
			Ok(connection) => connection,
			Err(error)     => fail!("Error connecting to server: {}", error)
		};

		Core {
			conn: connection,
			id  : None
		}
	}

	pub fn update_ships(&mut self, entities: &mut Entities) {
		let result = self.conn.receive_messages(|message| {
			match Message::from_str(message) {
				SelfInfo(self_info) =>
					entities.self_id = Some(self_info.id),

				Create(create) =>
					entities.create_ship(
						create.id),

				Update(update) =>
					for ship in update.ships.iter() {
						entities.update_ship(
							ship.id,
							ship.body)
					},

				Remove(remove) =>
					entities.remove_ship(
						remove.id),

				_ =>
					fail!("Unexpected message: {}", message)
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => fail!("Failed to receive message: {}", error)
		}
	}

	pub fn send_command(&mut self, attitude: Radians) {
		let command = Command(Command { attitude: attitude });
		match self.conn.send_message(command.to_str()) {
			Ok(())     => (),
			Err(error) => fail!("Error sending message: {}", error)
		}
	}
}

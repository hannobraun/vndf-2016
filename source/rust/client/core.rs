use ccore::args;
use ccore::net;
use ccore::protocol;
use ccore::protocol::Connection;

use entities::Entities;


pub struct Core {
	connection: protocol::Connection
}

impl Core {
	pub fn start() -> ~Core {
		let server_address = args::get_server_address();
		let socket_fd      = net::connect(server_address, ~"34481");

		~Core {
			connection: protocol::init(socket_fd) }
	}

	pub fn update_positions(&mut self, entities: &mut Entities) {
		let mut handler = ProtocolHandler {
			entities: entities };

		protocol::receive_positions(
			&mut self.connection,
			&mut handler);
	}
}


struct ProtocolHandler<'a> {
	entities: &'a mut Entities
}

impl<'a> protocol::Handler for ProtocolHandler<'a> {
	fn update_ship(&mut self, id: int, x: f64, y: f64) {
		self.entities.update_ship(id, x, y);
	}

	fn remove_ship(&mut self, id: int) {
		self.entities.remove_ship(id);
	}
}

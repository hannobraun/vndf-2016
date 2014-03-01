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
		protocol::receive_positions(
			&mut self.connection,
			entities);
	}
}

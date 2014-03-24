extern crate common;

use common::protocol::{Remove, SelfInfo, Update};

pub mod args;
pub mod net;
pub mod protocol;


struct ProtocolHandler;

impl protocol::Handler for ProtocolHandler {
	fn set_self_id(&self, message: SelfInfo) {
		print!("{}\n", message.to_str());
	}

	fn update_ship(&self, message: Update) {
		print!("{}\n", message.to_str());
	}

	fn remove_ship(&self, message: Remove) {
		print!("{}\n", message.to_str());
	}
}


fn main() {
	let (address, port) = args::address_and_port();

	let     socket_fd  = net::connect(address, port);
	let mut connection = protocol::init(socket_fd);

	let mut handler = ProtocolHandler;

	loop {
		protocol::receive_positions(
			&mut connection,
			&mut handler);
	}
}

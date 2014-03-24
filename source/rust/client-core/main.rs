extern crate common;

use common::protocol::{Remove, Update};

pub mod args;
pub mod net;
pub mod protocol;


struct ProtocolHandler;

impl protocol::Handler for ProtocolHandler {
	fn update_ship(&mut self, message: Update) {
		print!("{}\n", message.to_str());
	}

	fn remove_ship(&mut self, message: Remove) {
		print!("{}\n", message.to_str());
	}
}


fn main() {
	let (address, port) = args::address_and_port();

	let     socket_fd  = net::connect(address, port);
	let mut connection = protocol::init(socket_fd);

	let mut handler = ProtocolHandler;

	print!("SELF_ID {}\n", 1);

	loop {
		protocol::receive_positions(
			&mut connection,
			&mut handler);
	}
}

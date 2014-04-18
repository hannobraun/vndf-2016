extern crate common;
extern crate libc;

use common::net::Connection;
use common::protocol::{Create, Remove, SelfInfo, Update};

pub mod args;
pub mod net;
pub mod protocol;


struct ProtocolHandler;

impl protocol::Handler for ProtocolHandler {
	fn set_self_id(&self, message: SelfInfo) {
		print!("{}\n", SelfInfo(message).to_str());
	}

	fn create_ship(&self, message: Create) {
		print!("{}\n", Create(message).to_str());
	}

	fn update_ship(&self, message: Update) {
		print!("{}\n", Update(message).to_str());
	}

	fn remove_ship(&self, message: Remove) {
		print!("{}\n", Remove(message).to_str());
	}
}


fn main() {
	let (address, port) = args::address_and_port();

	let     connection = Connection::connect(address, port);
	let mut protocol   = protocol::init(connection.fd);

	let mut handler = ProtocolHandler;

	loop {
		protocol::receive_positions(
			&mut protocol,
			&mut handler);
	}
}

use std::io::net::ip::ToSocketAddr;
use std::vec::Drain;

use common::network::Connection;
use common::protocol::{
	ClientEvent,
	ServerEvent,
};


pub struct Network {
	connection: Connection<ServerEvent>,
}

impl Network {
	pub fn new<T: ToSocketAddr>(server_address: T) -> Network {
		Network {
			connection: Connection::new(server_address),
		}
	}

	pub fn send(&mut self, event: ClientEvent) {
		if let Err(error) = self.connection.send(Some(event).iter()) {
			panic!("Error sending event to server: {}", error);
		}
	}

	pub fn receive(&mut self) -> Drain<ServerEvent> {
		match self.connection.receive() {
			Ok(events) => events,
			Err(())    => panic!("Error receiving from connection"),
		}
	}

	pub fn update(&mut self) {}
}

use std::net::ToSocketAddrs;
use std::vec::Drain;

use common::network::Connection;
use common::protocol::{
	client,
	server,
};


pub struct Network {
	connection: Connection<server::Event>,
}

impl Network {
	pub fn new<T: ToSocketAddrs>(server_address: T) -> Network {
		Network {
			connection: Connection::new(server_address),
		}
	}

	pub fn send(&mut self, event: client::Event) {
		if let Err(error) = self.connection.send(Some(event).iter()) {
			panic!("Error sending event to server: {}", error);
		}
	}

	pub fn receive(&mut self) -> Drain<server::Event> {
		match self.connection.receive() {
			Ok(events) => events,
			Err(())    => panic!("Error receiving from connection"),
		}
	}
}

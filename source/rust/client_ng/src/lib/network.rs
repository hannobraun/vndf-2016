use std::io::net::ip::{
	SocketAddr,
	ToSocketAddr,
};

use acpe::network::{
	self,
	Message,
};
use acpe::protocol;

use common::protocol::Perception;


pub struct Socket {
	address : SocketAddr,
	inner   : network::Socket,
	messages: Vec<Message>,
}

impl Socket {
	pub fn new<T: ToSocketAddr>(address: T) -> Socket {
		let address = address
			.to_socket_addr()
			.unwrap_or_else(|error|
				panic!("Error converting socket address: {}", error)
			);

		let socket = network::Socket::new(0);

		Socket {
			address : address,
			inner   : socket,
			messages: Vec::new(),
		}
	}

	pub fn receive(&mut self, perceptions: &mut Vec<Perception>) {
		self.inner.receive(&mut self.messages);

		for (message, _) in self.messages.drain() {
			perceptions.push(
				protocol::Message::decode(message.as_slice())
					.unwrap_or_else(|error|
						panic!(
							"Error decoding message from server. \
							Message: {}; Error: {}",
							message, error
						)
					)
			);
		}
	}

	pub fn send(&mut self, message: &[u8]) {
		self.inner.send(message, self.address);
	}
}

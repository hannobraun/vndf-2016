use std::io::net::ip::{
	Port,
	SocketAddr,
};

use acpe::network::{
	mod,
	Message,
};
use acpe::protocol::Action;

use common::protocol::Step;


pub type ReceiveResult =
	Result<(Action<Step>, SocketAddr), (String, SocketAddr)>;


pub struct Socket {
	pub inner: network::Socket,

	messages: Vec<Message>,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		Socket {
			inner   : network::Socket::new(port),
			messages: Vec::new(),
		}
	}

	pub fn send(&mut self, message: &[u8], address: SocketAddr) {
		self.inner.send(message, address)
	}

	pub fn receive(&mut self) -> Vec<ReceiveResult> {
		self.messages.clear();
		self.inner.receive(&mut self.messages);

		self.messages
			.iter()
			.map(|&(ref message, address)| {
				match decode_message(message.as_slice()) {
					Ok(message) => Ok((message, address)),
					Err(error)  => Err((error, address)),
				}
			})
			.collect()
	}
}


fn decode_message(message: &[u8]) -> Result<Action<Step>, String> {
	let message = match Action::decode(message) {
		Ok(message) =>
			message,
		Err(error) =>
			return Err((
				format!(
					"Error decoding message. Error: {}; Message: {}",
					error, message
				)
			)),
	};

	Ok(message)
}

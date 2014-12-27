use std::io::net::ip::{
	Port,
	SocketAddr,
};

use acpe::network;
use acpe::protocol::Action;

use common::protocol::Step;


pub type ReceiveResult =
	Result<(Action<Step>, SocketAddr), (String, SocketAddr)>;


pub struct Socket {
	pub inner: network::Socket,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		Socket {
			inner: network::Socket::new(port),
		}
	}

	pub fn send_to(&mut self, message: &[u8], address: SocketAddr) {
		self.inner.send(message, address)
	}

	pub fn recv_from(&self) -> Vec<ReceiveResult> {
		self.inner.recv_from()
			.into_iter()
			.map(|(message, address)| {
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

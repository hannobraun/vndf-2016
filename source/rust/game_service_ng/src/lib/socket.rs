use std::io::net::ip::{
	Port,
	SocketAddr,
};

use common::network::{
	mod,
	ReceiveResult,
};


pub struct Socket {
	inner: network::Socket,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		Socket {
			inner: network::Socket::new(port),
		}
	}

	pub fn send_to(&mut self, message: &[u8], address: SocketAddr) {
		self.inner.send_to(message, address)
	}

	pub fn recv_from(&self) -> Vec<ReceiveResult> {
		self.inner.recv_from()
	}
}
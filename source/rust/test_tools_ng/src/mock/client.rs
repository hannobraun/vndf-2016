use std::io::net::ip::{
	Port,
	ToSocketAddr,
};
use std::io::timer::sleep;
use std::time::Duration;

use client_ng::Server;
use protocol_ng::{
	Action,
	Perception,
};


pub struct Client {
	server: Server,
}

impl Client {
	pub fn start(port: Port) -> Client {
		Client {
			server: Server::new(("localhost", port).to_socket_addr().unwrap()),
		}
	}

	pub fn send_action(&mut self, action: Action) {
		self.server.send_to(action)
	}

	pub fn expect_perception(&self) -> Option<Perception> {
		let mut perception = None;

		while perception.is_none() {
			perception = self.server.recv_from();
			sleep(Duration::milliseconds(20));
		}

		perception
	}
}

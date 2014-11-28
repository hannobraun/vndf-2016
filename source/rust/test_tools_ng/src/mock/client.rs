use std::io::net::ip::Port;
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
			server: Server::new(("localhost", port)),
		}
	}

	pub fn send_data(&mut self, data: &[u8]) {
		self.server.send_to(data);
	}

	pub fn send_action(&mut self, action: Action) {
		self.send_data(action.to_json().as_bytes());
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

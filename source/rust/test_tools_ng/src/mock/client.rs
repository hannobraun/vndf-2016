use std::io::net::ip::Port;

use protocol_ng::{
	Action,
	Perception,
};


pub struct Client;

impl Client {
	pub fn start(_port: Port) -> Client {
		Client
	}

	pub fn send_action(&self, _action: Action) {
		// TODO: Send action to server
	}

	pub fn expect_perception(&self) -> Option<Perception> {
		// TODO: Receive perception
		Some(Perception {
			last_action: 512,
			broadcasts : Vec::new(),
		})
	}
}

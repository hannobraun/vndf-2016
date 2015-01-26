use std::io::net::ip::Port;

use time::precise_time_s;

use client::network::Network;
use common::protocol::{
	ClientEvent,
	ServerEvent,
};


pub struct Client {
	network : Network,
	incoming: Vec<ServerEvent>,
}

impl Client {
	pub fn start(port: Port) -> Client {
		Client {
			network : Network::new(("localhost", port)),
			incoming: Vec::new(),
		}
	}

	pub fn send(&mut self, event: ClientEvent) {
		self.network.send(event);
		self.network.update();
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_event(&mut self) -> Option<ServerEvent> {
		let start_s = precise_time_s();

		while self.incoming.len() == 0 && precise_time_s() - start_s < 0.1 {
			self.incoming.extend(self.network.receive());
		}
		self.network.update();

		if self.incoming.len() > 0 {
			Some(self.incoming.remove(0))
		}
		else {
			None
		}
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn wait_until<F>(&mut self, mut condition: F) -> Option<ServerEvent>
		where F: FnMut(&Option<ServerEvent>) -> bool
	{
		let start_s = precise_time_s();

		let mut event = self.expect_event();

		while !condition(&event) {
			if precise_time_s() - start_s > 0.5 {
				panic!("Condition not satisfied after waiting");
			}

			event = self.expect_event();
		}

		event
	}
}

use time::precise_time_s;

use client::network::Network;
use shared::protocol::{
	client,
	server,
};


pub struct Client {
	network : Network,
	incoming: Vec<server::Event>,
}

impl Client {
	pub fn start(port: u16) -> Client {
		Client {
			network : Network::new(("localhost", port)),
			incoming: Vec::new(),
		}
	}

	pub fn send(&mut self, event: client::Event) {
		self.network.send(event);
	}

	// TODO(5rKZ3HPd): Make generic and move into a trait called Mock.
	pub fn expect_event(&mut self) -> Option<server::Event> {
		let start_s = precise_time_s();

		while self.incoming.len() == 0 && precise_time_s() - start_s < 0.1 {
			self.incoming.extend(self.network.receive());
		}

		if self.incoming.len() > 0 {
			Some(self.incoming.remove(0))
		}
		else {
			None
		}
	}

	// TODO(5rKZ3HPd): Make generic and move into a trait called Mock.
	// TODO(5rKZ3HPd): Don't return Option<server::Event>. Instead return
	//                 Option<whatever the closure returns>. The return value
	//                 of the closure should be something like (bool, T), or
	//                 maybe Result<T, ()>.
	// TODO(5rKZ3HPd): Passing an Option to the closure doesn't seem to make any
	//                 sense. Why call it at all, if expect_event returns None?
	pub fn wait_until<F>(&mut self, mut condition: F) -> Option<server::Event>
		where F: FnMut(&Option<server::Event>) -> bool
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

use std::net::SocketAddr;

use common::protocol::client;


pub struct EventHandler {
	pub incoming: Vec<(SocketAddr, client::Event)>,
}

impl EventHandler {
	pub fn new() -> EventHandler {
		EventHandler {
			incoming: Vec::new(),
		}
	}

	pub fn receive<E>(&mut self, events: E)
		where E: Iterator<Item = (SocketAddr, client::Event)>
	{
		for (address, event) in events {
			self.incoming.push((address, event));
		}
	}
}

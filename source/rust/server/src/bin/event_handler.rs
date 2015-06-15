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
}

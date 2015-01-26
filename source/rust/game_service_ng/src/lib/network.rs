use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::vec::Drain;

use common::network::{
	Acceptor,
	Connection,
};
use common::protocol::{
	ClientEvent,
	ServerEvent,
};


pub struct Network {
	acceptor   : Acceptor<ClientEvent>,
	connections: HashMap<SocketAddr, Connection<ClientEvent>>,
	incoming   : Vec<(SocketAddr, ClientEvent)>,
	to_remove  : Vec<SocketAddr>,
}

impl Network {
	pub fn new(port: Port) -> Network {
		Network {
			acceptor   : Acceptor::new(port),
			connections: HashMap::new(),
			incoming   : Vec::new(),
			to_remove  : Vec::new(),
		}
	}

	pub fn send<'a, R, E>(&mut self, mut recipients: R, events: E)
		where
			R: Iterator<Item = SocketAddr>,
			E: Iterator<Item = ServerEvent>,
	{
		let events: Vec<ServerEvent> = events.collect();

		for address in recipients {
			let mut recipient = match self.connections.get_mut(&address) {
				Some(connection) => connection,
				None             => continue,
			};

			if let Err(error) = recipient.send(events.iter()) {
				self.to_remove.push(address);
				print!("Error sending event to {}: {}\n", address, error)
			}
		}
	}

	pub fn receive(&mut self) -> Drain<(SocketAddr, ClientEvent)> {
		self.connections.extend(self.acceptor.accept());

		for address in self.to_remove.drain() {
			self.connections.remove(&address);
		}

		for (address, connection) in self.connections.iter_mut() {
			let events = match connection.receive() {
				Ok(events) =>
					events,
				Err(()) => {
					self.to_remove.push(*address);
					continue;
				},
			};

			self.incoming.extend(events.map(|event| (*address, event)));
		}

		self.incoming.drain()
	}

	pub fn update(&mut self) {}
}

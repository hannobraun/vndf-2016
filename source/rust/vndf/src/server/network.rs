use std::collections::HashMap;
use std::net::SocketAddr;
use std::vec::Drain;

use shared::network::{
	Acceptor,
	Connection,
};
use shared::protocol::{
	client,
	server,
};


pub struct Network {
	acceptor   : Acceptor<client::Event>,
	connections: HashMap<SocketAddr, Connection<client::Event>>,
	incoming   : Vec<(SocketAddr, client::Event)>,
	to_remove  : Vec<SocketAddr>,
}

impl Network {
	pub fn new(port: u16) -> Network {
		Network {
			acceptor   : Acceptor::new(port),
			connections: HashMap::new(),
			incoming   : Vec::new(),
			to_remove  : Vec::new(),
		}
	}

	pub fn send<R, E>(&mut self, recipients: R, events: E)
		where
			R: Iterator<Item = SocketAddr>,
			E: Iterator<Item = server::Event>,
	{
		// I don't like all the heap allocation that goes on in this method, but
		// I believe it's okay for now. It's only a crutch that's required until
		// we switch to a non-blocking network API. If we had only one thread,
		// we wouldn't need to move all this stuff around.
		let events: Vec<_> = events.collect();

		for address in recipients {
			let mut recipient = match self.connections.get_mut(&address) {
				Some(connection) => connection,
				None             => continue,
			};

			if let Err(error) = recipient.send(events.clone().into_iter()) {
				self.to_remove.push(address);
				debug!("Error sending event to {}: {}", address, error)
			}
		}
	}

	pub fn receive(&mut self) -> Drain<(SocketAddr, client::Event)> {
		self.connections.extend(self.acceptor.accept());

		for address in self.to_remove.drain(..) {
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

		self.incoming.drain(..)
	}
}

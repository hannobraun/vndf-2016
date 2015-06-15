use std::collections::HashMap;
use std::net::SocketAddr;
use std::vec::Drain;

use common::network::{
	Acceptor,
	Connection,
};
use common::protocol::{
	client,
	ServerEvent,
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

	pub fn send<R>(&mut self, recipients: R, events: &[ServerEvent])
		where
			R: Iterator<Item = SocketAddr>,
	{
		for address in recipients {
			let mut recipient = match self.connections.get_mut(&address) {
				Some(connection) => connection,
				None             => continue,
			};

			if let Err(error) = recipient.send(events.iter()) {
				self.to_remove.push(address);
				debug!("Error sending event to {}: {}\n", address, error)
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

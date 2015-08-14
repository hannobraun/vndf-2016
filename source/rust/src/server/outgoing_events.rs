use std::net::SocketAddr;

use server::clients::Clients;
use server::network::Network;
use shared::protocol::server::Event;


pub struct OutgoingEvents {
	events: Vec<(Recipients, Event)>,
}

impl OutgoingEvents {
	pub fn new() -> OutgoingEvents {
		OutgoingEvents {
			events: Vec::new(),
		}
	}

	pub fn push(&mut self, event: Event, recipients: Recipients) {
		self.events.push((recipients, event));
	}

	pub fn send(&mut self, clients: &mut Clients, network: &mut Network) {
		for (recipients, event) in self.events.drain(..) {
			match recipients {
				Recipients::All => {
					let recipients = clients.clients
						.iter()
						.map(|(&address, _)|
							address
						);

					network.send(
						recipients,
						Some(event).into_iter(),
					);
				},

				Recipients::One(address) => {
					network.send(
						Some(address).into_iter(),
						Some(event).into_iter(),
					);
				},
			};
		}
	}
}


pub enum Recipients {
	All,
	One(SocketAddr),
}

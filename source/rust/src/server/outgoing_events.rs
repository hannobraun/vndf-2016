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
		let recipients = clients
			.iter()
			.map(|(&address, _)|
				address
			);

		let events = self.events
			.drain(..)
			.map(|(_, event)| event);

		network.send(recipients, events);
	}
}


pub enum Recipients {
	All,
}

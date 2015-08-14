use server::clients::Clients;
use server::network::Network;
use shared::protocol::server::Event;


pub struct OutgoingEvents {
	// TODO: Make private
	pub events: Vec<Event>,
}

impl OutgoingEvents {
	pub fn new() -> OutgoingEvents {
		OutgoingEvents {
			events: Vec::new(),
		}
	}

	pub fn push(&mut self, event: Event) {
		self.events.push(event);
	}

	pub fn send(&mut self, clients: &mut Clients, network: &mut Network) {
		let recipients = clients
			.iter()
			.map(|(&address, _)|
				address
			);

		network.send(recipients, self.events.as_ref());
		self.events.clear();
	}
}

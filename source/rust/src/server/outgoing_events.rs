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
}

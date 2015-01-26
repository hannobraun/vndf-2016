use std::collections::HashSet;
use std::io::net::ip::ToSocketAddr;
use std::vec::Drain;

use acpe::protocol::Encoder;

use action_assembler::ActionAssembler;
use common::protocol::{
	ClientEvent,
	Percept,
	Perception,
	ServerEvent,
	Step,
};
use socket::Socket;


pub struct Network {
	pub action_assembler: ActionAssembler,
	pub broadcasters    : HashSet<String>,
	pub encoder         : Encoder,
	pub events          : Vec<ServerEvent>,
	pub perceptions     : Vec<Perception>,
	pub server          : Socket,
}

impl Network {
	pub fn new<T: ToSocketAddr>(server_address: T) -> Network {
		Network {
			action_assembler: ActionAssembler::new(),
			broadcasters    : HashSet::new(),
			encoder         : Encoder::new(),
			events          : Vec::new(),
			perceptions     : Vec::new(),
			server          : Socket::new(server_address),
		}
	}

	pub fn send(&mut self, event: ClientEvent) {
		let step = match event {
			ClientEvent::Login                   => Step::Login,
			ClientEvent::Heartbeat               => return,
			ClientEvent::StartBroadcast(message) => Step::Broadcast(message),
			ClientEvent::StopBroadcast           => Step::StopBroadcast,
		};

		self.action_assembler.add_step(step);
	}

	/// This method is intended for testing and should not be used in production
	/// code.
	pub fn send_raw(&mut self, message: &[u8]) {
		self.server.send(message);
	}

	pub fn receive(&mut self) -> Drain<ServerEvent> {
		self.server.receive(&mut self.perceptions);

		for mut perception in self.perceptions.drain() {
			let mut current_broadcasters = HashSet::new();

			for (_, percept) in perception.drain_update_items() {
				match percept {
					Percept::Broadcast(broadcast) => {
						current_broadcasters.insert(broadcast.sender.clone());
						self.events.push(
							ServerEvent::StartBroadcast(broadcast)
						);
					},
				}
			}

			match perception.header.self_id {
				Some(self_id) =>
					self.events.push(ServerEvent::SelfId(self_id)),
				None =>
					(),
			}

			for sender in self.broadcasters.drain() {
				if !current_broadcasters.contains(&sender) {
					self.events.push(ServerEvent::StopBroadcast(sender))
				}
			}
			self.broadcasters = current_broadcasters;

			self.action_assembler.process_receipt(
				perception.header.confirm_action
			);
		}

		self.events.drain()
	}

	pub fn update(&mut self) {
		let message = self.action_assembler.assemble(&mut self.encoder);
		self.server.send(message.as_slice());
	}
}

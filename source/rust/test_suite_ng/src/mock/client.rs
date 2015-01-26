use std::collections::HashMap;
use std::io::net::ip::Port;

use acpe::protocol::{
	Message,
	PerceptionHeader,
	Seq,
};
use time::precise_time_s;

use client::network::Network;
use common::game::Broadcast;
use common::protocol::{
	ClientEvent,
	Percept,
	Perception,
	ServerEvent,
	Step,
};


pub struct Client {
	network   : Network,
	incoming  : Vec<ServerEvent>,
	broadcasts: HashMap<String, Broadcast>,
	self_id   : Option<String>,
}

impl Client {
	pub fn start(port: Port) -> Client {
		Client {
			network   : Network::new(("localhost", port)),
			incoming  : Vec::new(),
			broadcasts: HashMap::new(),
			self_id   : None,
		}
	}

	pub fn send_raw(&mut self, data: &[u8]) {
		self.network.send_raw(data);
	}

	pub fn send_action(&mut self, steps: Vec<Step>) {
		for step in steps.into_iter() {
			let event = match step {
				Step::Login =>
					ClientEvent::Login,
				Step::Broadcast(broadcast) =>
					ClientEvent::StartBroadcast(broadcast),
				Step::StopBroadcast =>
					ClientEvent::StopBroadcast,
			};
			self.network.send(event);
		}
		self.network.update();
	}

	pub fn login(&mut self, _: Seq) {
		self.send_action(vec![Step::Login]);
	}

	pub fn broadcast(&mut self, _: Seq, text: String) {
		self.send_action(vec![Step::Broadcast(text)]);
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_perception(&mut self) -> Option<Perception> {
		let start_s = precise_time_s();

		while self.incoming.len() == 0 && precise_time_s() - start_s < 0.1 {
			self.incoming.extend(self.network.receive());
		}
		self.network.update();

		if self.incoming.len() > 0 {
			for event in self.incoming.drain() {
				match event {
					ServerEvent::SelfId(self_id) => {
						self.self_id = Some(self_id);
					},
					ServerEvent::StartBroadcast(broadcast) => {
						self.broadcasts.insert(
							broadcast.sender.clone(),
							broadcast
						);
					},
					ServerEvent::StopBroadcast(sender) => {
						self.broadcasts.remove(&sender);
					},
				}
			}

			let mut perception = Message::new(PerceptionHeader {
				confirm_action: 0,
				self_id       : self.self_id.clone(),
			});

			for (_, broadcast) in self.broadcasts.iter() {
				perception.add_update(
					broadcast.sender.clone(),
					Percept::Broadcast(broadcast.clone()),
				);
			}

			Some(perception)
		}
		else {
			None
		}
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn wait_until<F>(&mut self, mut condition: F) -> Option<Perception>
		where F: FnMut(&Option<Perception>) -> bool
	{
		let start_s = precise_time_s();

		let mut perception = self.expect_perception();

		while !condition(&perception) {
			if precise_time_s() - start_s > 0.5 {
				panic!("Condition not satisfied after waiting");
			}

			perception = self.expect_perception();
		}

		perception
	}
}

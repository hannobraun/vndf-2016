use std::io::net::ip::Port;

use acpe::protocol::{
	ActionHeader,
	Message,
	Seq,
};
use time::precise_time_s;

use client::socket::Socket;
use common::protocol::{
	Perception,
	Step,
};


// TODO: Rewrite on top of Network
pub struct Client {
	socket     : Socket,
	perceptions: Vec<Perception>,
}

impl Client {
	pub fn start(port: Port) -> Client {
		Client {
			socket     : Socket::new(("localhost", port)),
			perceptions: Vec::new(),
		}
	}

	pub fn send_raw(&mut self, data: &[u8]) {
		self.socket.send(data);
	}

	pub fn send_action(&mut self, seq: Seq, steps: Vec<Step>) {
		let mut action = Message::new(ActionHeader { id: seq });
		for step in steps.into_iter() {
			action.add_update(0, step);
		}

		self.send_raw(action.encode().as_slice());
	}

	pub fn login(&mut self, seq: Seq) {
		self.send_action(seq, vec![Step::Login]);
	}

	pub fn broadcast(&mut self, seq: Seq, text: String) {
		self.send_action(seq, vec![Step::Broadcast(text)]);
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_perception(&mut self) -> Option<Perception> {
		let start_s = precise_time_s();

		while self.perceptions.len() == 0 && precise_time_s() - start_s < 0.1 {
			self.socket.receive(&mut self.perceptions);
		}

		if self.perceptions.len() > 0 {
			Some(self.perceptions.remove(0))
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

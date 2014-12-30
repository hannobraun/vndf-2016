use std::io::net::ip::Port;

use acpe::protocol::{
	ActionHeader,
	Message,
	PerceptionHeader,
	Seq,
};
use time::precise_time_s;

use client::network::Socket;
use common::protocol::{
	Action,
	Percept,
	Step,
};


pub struct Client {
	socket     : Socket,
	perceptions: Vec<Message<PerceptionHeader, String, Percept>>,
}

impl Client {
	pub fn start(port: Port) -> Client {
		Client {
			socket     : Socket::new(("localhost", port)),
			perceptions: Vec::new(),
		}
	}

	pub fn send_data(&mut self, data: &[u8]) {
		self.socket.send(data);
	}

	pub fn send_action(&mut self, seq: Seq, steps: Vec<Step>) {
		let action = Action {
			header : ActionHeader { id: seq },
			update : steps,
			destroy: Vec::new(),
		};
		self.send_data(action.encode().as_slice());
	}

	pub fn login(&mut self, seq: Seq) {
		self.send_action(seq, vec![Step::Login]);
	}

	pub fn broadcast(&mut self, seq: Seq, text: String) {
		self.send_action(seq, vec![Step::Broadcast(text)]);
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_perception(&mut self) -> Option<Message<PerceptionHeader, String, Percept>> {
		let start_s = precise_time_s();

		while self.perceptions.len() == 0 && precise_time_s() - start_s < 0.1 {
			self.socket.receive(&mut self.perceptions);
		}

		self.perceptions.remove(0)
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn wait_until(
		&mut self,
		condition: |&Option<Message<PerceptionHeader, String, Percept>>| -> bool
	) -> Option<Message<PerceptionHeader, String, Percept>> {
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

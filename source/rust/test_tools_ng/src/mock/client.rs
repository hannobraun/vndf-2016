use std::io::net::ip::Port;

use acpe::protocol::{
	Action,
	ActionHeader,
	Perception,
	Seq,
};
use time::precise_time_s;

use client::network::Socket;
use common::protocol::{
	Percept,
	Step,
};


pub struct Client {
	socket     : Socket,
	perceptions: Vec<Perception<String, Percept>>,
}

impl Client {
	pub fn start(port: Port) -> Client {
		Client {
			socket     : Socket::new(("localhost", port)),
			perceptions: Vec::new(),
		}
	}

	pub fn send_data(&mut self, data: &[u8]) {
		self.socket.send_to(data);
	}

	pub fn send_action(&mut self, action: Action<Step>) {
		self.send_data(action.encode().as_slice());
	}

	pub fn login(&mut self, seq: Seq) {
		self.send_action(Action {
			header: ActionHeader { id: seq },
			steps : vec![Step::Login],
		});
	}

	pub fn broadcast(&mut self, seq: Seq, text: String) {
		self.send_action(Action {
			header: ActionHeader { id: seq },
			steps : vec![Step::Broadcast(text)],
		})
	}

	pub fn expect_perception(&mut self) -> Option<Perception<String, Percept>> {
		let start_s = precise_time_s();

		while self.perceptions.len() == 0 && precise_time_s() - start_s < 0.1 {
			self.perceptions.push_all(self.socket.recv_from().as_slice());
		}

		self.perceptions.remove(0)
	}

	pub fn wait_until(
		&mut self,
		condition: |&Option<Perception<String, Percept>>| -> bool
	) -> Option<Perception<String, Percept>> {
		let mut perception = self.expect_perception();

		while !condition(&perception) {
			perception = self.expect_perception();
		}

		perception
	}
}

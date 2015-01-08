use std::io::net::ip::Port;

use acpe::protocol::{
	ActionHeader,
	Seq,
};
use time::precise_time_s;

use client::network::Socket;
use common::protocol::{
	Action,
	Perception,
	Step,
};


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

	pub fn send_data(&mut self, data: &[u8]) {
		self.socket.send(data);
	}

	pub fn send_action(&mut self, seq: Seq, steps: Vec<Step>) {
		let action = Action {
			header : ActionHeader { id: seq },
			update : steps.into_iter().map(|step| (0u64, step)).collect(),
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
	pub fn wait_until<F>(&mut self, condition: F) -> Option<Perception>
		where F: Fn(&Option<Perception>) -> bool
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

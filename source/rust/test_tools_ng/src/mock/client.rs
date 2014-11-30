use std::io::net::ip::Port;
use std::io::timer::sleep;
use std::time::Duration;
use time::precise_time_s;

use client_ng::Server;
use protocol_ng::{
	Action,
	Perception,
	Seq,
	Step,
};


pub struct Client {
	server: Server,
}

impl Client {
	pub fn start(port: Port) -> Client {
		Client {
			server: Server::new(("localhost", port)),
		}
	}

	pub fn send_data(&mut self, data: &[u8]) {
		self.server.send_to(data);
	}

	pub fn send_action(&mut self, action: Action) {
		self.send_data(action.to_json().as_bytes());
	}

	pub fn login(&mut self, seq: Seq) {
		self.send_action(Action {
			seq  : seq,
			steps: vec![Step::Login],
		});
	}

	pub fn broadcast(&mut self, seq: Seq, text: String) {
		self.send_action(Action {
			seq  : seq,
			steps: vec![Step::Broadcast(text)],
		})
	}

	pub fn expect_perception(&self) -> Option<Perception> {
		let start_s = precise_time_s();

		let mut perception = None;

		while perception.is_none() && precise_time_s() - start_s < 0.1 {
			perception = self.server.recv_from();
			sleep(Duration::milliseconds(20));
		}

		perception
	}

	pub fn wait_until(
		&self,
		condition: |&Option<Perception>| -> bool
	) -> Option<Perception> {
		let mut perception = self.expect_perception();

		while !condition(&perception) {
			perception = self.expect_perception();
		}

		perception
	}
}

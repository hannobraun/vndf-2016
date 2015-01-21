use std::io::net::ip::SocketAddr;
use std::vec::Drain;

use acpe::protocol::Encoder;

use action_assembler::ActionAssembler;
use client::network::Socket;
use common::protocol::{
	Perception,
	Step,
};


pub struct Network {
	pub action_assembler: ActionAssembler,
	pub encoder         : Encoder,
	pub perceptions     : Vec<Perception>,
	pub server          : Socket,
}

impl Network {
	pub fn new(server_address: SocketAddr) -> Network {
		let mut action_assembler = ActionAssembler::new();
		let     encoder          = Encoder::new();
		let     perceptions      = Vec::new();
		let     server           = Socket::new(server_address);

		action_assembler.add_step(Step::Login);

		Network {
			action_assembler: action_assembler,
			encoder         : encoder,
			perceptions     : perceptions,
			server          : server,
		}
	}

	pub fn send(&mut self, event: Step) {
		self.action_assembler.add_step(event);
	}

	pub fn receive(&mut self) -> Drain<Perception> {
		self.server.receive(&mut self.perceptions);

		for perception in self.perceptions.iter() {
			self.action_assembler.process_receipt(
				perception.header.confirm_action
			);
		}

		self.perceptions.drain()
	}
}

use std::io::net::ip::{
	Port,
	SocketAddr,
};
use time::precise_time_s;

use acceptance::random_port;
use acpe::network::SocketSender;
use acpe::protocol::{
	Encoder,
	MessageEncoder,
	PerceptionHeader,
	Seq,
};

use common::protocol::{
	Action,
	Percept,
	Perception,
};
use game_service::{
	ReceiveResult,
	Socket,
};


pub struct GameService {
	port    : Port,
	socket  : Socket,
	received: Vec<ReceiveResult>,
}

impl GameService {
	pub fn start() -> GameService {
		let port   = random_port(40000, 50000);
		let socket = Socket::new(port);

		GameService {
			port    : port,
			socket  : socket,
			received: Vec::new(),
		}
	}

	pub fn port(&self) -> Port {
		self.port
	}

	pub fn send_perception(
		&mut self,
		address : SocketAddr,
		confirm : Seq,
		percepts: Vec<Percept>,
	) {
		let perception = Perception {
			header: PerceptionHeader {
				confirm_action: confirm,
				self_id       : None,
			},
			update : percepts,
			destroy: Vec::new(),
		};

		self.socket.send(perception.encode().as_slice(), address);
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_action(&mut self) -> Option<ActionHandle> {
		let start_s = precise_time_s();

		while self.received.len() == 0 && precise_time_s() - start_s < 0.5 {
			self.socket.receive(&mut self.received);
		}

		match self.received.remove(0) {
			Some(result) => match result {
				Ok((action, address)) =>
					Some(ActionHandle {
						inner  : action,
						address: address,
						sender : self.socket.inner.sender.clone(),
					}),
				Err((error, address)) =>
					panic!(
						"Error receiving message from {}: {}",
						address, error
					),
			},
			None =>
				None,
		}
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn wait_until(
		&mut self,
		condition: |&mut Option<ActionHandle>| -> bool
	) -> Option<ActionHandle> {
		let start_s = precise_time_s();

		let mut action = self.expect_action();

		while !condition(&mut action) {
			if precise_time_s() - start_s > 0.5 {
				panic!("Condition not satisfied after waiting");
			}

			action = self.expect_action();
		}

		action
	}
}


pub struct ActionHandle {
	pub inner  : Action,
	pub address: SocketAddr,

	sender : SocketSender,
}

impl ActionHandle {
	pub fn ignore(&self) {}

	pub fn confirm(&mut self) {
		let mut encoder = Encoder::new();

		let perception: MessageEncoder<PerceptionHeader, Percept> =
			encoder.message(&PerceptionHeader {
				confirm_action: self.inner.header.id,
				self_id       : None,
			});

		let message = perception.encode();
		self.sender.send(message, self.address);
	}
}

use std::io::net::ip::{
	Port,
	SocketAddr,
};
use time::precise_time_s;

use acceptance::random_port;
use game_service_ng::{
	ReceiveResult,
	Socket,
	SocketSender,
};
use protocol_ng::{
	MAX_PACKET_SIZE,
	Action,
	Encoder,
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

	pub fn expect_action(&mut self) -> Option<ActionHandle> {
		let start_s = precise_time_s();

		while self.received.len() == 0 && precise_time_s() - start_s < 0.5 {
			self.received.push_all(self.socket.recv_from().as_slice());
		}

		match self.received.remove(0) {
			Some(result) => match result {
				ReceiveResult::Message(action, address) =>
					Some(ActionHandle {
						inner  : action,
						address: address,
						sender : self.socket.sender.clone(),
					}),
				ReceiveResult::Error(error, address) =>
					panic!(
						"Error receiving message from {}: {}",
						address, error
					),
			},
			None =>
				None,
		}
	}

	pub fn wait_until(
		&mut self,
		condition: |&Option<ActionHandle>| -> bool
	) -> Option<ActionHandle> {
		let mut action = self.expect_action();

		while !condition(&action) {
			action = self.expect_action();
		}

		action
	}
}


pub struct ActionHandle {
	pub inner: Action,

	address: SocketAddr,
	sender : SocketSender,
}

impl ActionHandle {
	pub fn ignore(&self) {}

	pub fn confirm(&mut self) {
		let mut encoder       = Encoder::new();
		let mut encode_buffer = [0, ..MAX_PACKET_SIZE];
		let     perception    = encoder.perception(self.inner.seq);

		let message = perception.encode(&mut encode_buffer).unwrap();
		self.sender.send(message, self.address);
	}
}

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
	Action,
	Perception,
};


pub struct GameService {
	port  : Port,
	socket: Socket,
}

impl GameService {
	pub fn start() -> GameService {
		let port   = random_port(40000, 50000);
		let socket = Socket::new(port);

		GameService {
			port  : port,
			socket: socket,
		}
	}

	pub fn port(&self) -> Port {
		self.port
	}

	pub fn expect_action(&mut self) -> Option<ActionHandle> {
		let start_s = precise_time_s();

		let mut message = None;

		while message.is_none() && precise_time_s() - start_s < 0.5 {
			message = match self.socket.recv_from() {
				ReceiveResult::Message(action, address) =>
					Some((action, address)),
				ReceiveResult::None =>
					None,
				ReceiveResult::ClientError(error, address) =>
					panic!(
						"Error receiving message from {}: {}",
						address, error
					),
			}
		}

		match message {
			Some((action, address)) =>
				Some(ActionHandle {
					inner  : action,
					address: address,
					sender : self.socket.sender.clone(),
				}),

			None => None,
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
		let perception = Perception {
			last_action: self.inner.seq,
			broadcasts : Vec::new(),
		};

		self.sender.send(perception.to_json().as_bytes(), self.address);
	}
}

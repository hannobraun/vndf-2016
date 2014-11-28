use std::io::net::ip::{
	Port,
	SocketAddr,
};
use time::precise_time_s;

use acceptance::random_port;
use game_service_ng::Socket;
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
			message = self.socket.recv_from();
		}

		print!("message: {}\n", message);

		match message {
			Some((action, address)) =>
				Some(ActionHandle {
					inner  : action,
					address: address,
					socket : &mut self.socket,
				}),

			None => None,
		}
	}
}


pub struct ActionHandle<'r> {
	pub inner: Action,

	address: SocketAddr,
	socket : &'r mut Socket,
}

impl<'r> ActionHandle<'r> {
	pub fn ignore(&self) {}

	pub fn confirm(&mut self) {
		let perception = Perception {
			last_action: self.inner.seq,
			broadcasts : Vec::new(),
		};

		self.socket.send_to(perception.to_json().as_bytes(), self.address);
	}
}

use std::io::net::ip::{
	Port,
	SocketAddr,
};
use time::precise_time_s;

use acceptance::random_port;
use acpe::network::SocketSender;
use acpe::protocol::{
	Encoder,
	Message,
	MessageEncoder,
	PerceptionHeader,
	Seq,
};

use common::protocol::{
	Action,
	Percept,
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
		update  : Vec<(String, Percept)>,
	) {
		let mut perception: Message<PerceptionHeader<String>, _, _> =
			Message::new(PerceptionHeader {
				confirm_action: confirm,
				self_id       : None,
			}
		);
		for (id, entity) in update.into_iter() {
			perception.add_update(id, entity);
		}

		self.socket.send(perception.encode().as_slice(), address);
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_action(&mut self) -> Option<ActionHandle> {
		let start_s = precise_time_s();

		while self.received.len() == 0 && precise_time_s() - start_s < 0.5 {
			self.socket.receive(&mut self.received);
		}

		if self.received.len() > 0 {
			match self.received.remove(0) {
				Ok((action, address)) => {
					let mut action_handle = ActionHandle {
						inner  : action,
						address: address,
						sender : self.socket.inner.sender.clone(),
					};

					let mut encoder = Encoder::new();

					let perception: MessageEncoder<PerceptionHeader<String>, String, Percept> =
						encoder.message(&PerceptionHeader {
							confirm_action: action_handle.inner.header.id,
							self_id       : None,
						});

					let message = perception.encode();
					action_handle.sender.send(message, address);

					Some(action_handle)
				},
				Err((error, address)) =>
					panic!(
						"Error receiving message from {}: {}",
						address, error
					),
			}
		}
		else {
			None
		}
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn wait_until<F>(&mut self, condition: F) -> Option<ActionHandle>
		where F: Fn(&mut Option<ActionHandle>) -> bool
	{
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
}

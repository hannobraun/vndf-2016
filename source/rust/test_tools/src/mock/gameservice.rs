use std::rand::random;

use game::ecs::Entity;
use net::{
	Acceptor,
	Connection
};
use protocol::Perception;
use rustecs::EntityId;


pub struct MockGameService {
	pub port: u16,

	acceptor: Acceptor,
	clients : Vec<Connection>
}

impl MockGameService {
	pub fn start() -> MockGameService {
		let port = random::<u16>() % 10000 + 40000;

		let acceptor = match Acceptor::new(port.to_string().as_slice()) {
			Ok(acceptor) => acceptor,
			Err(error)   => panic!("Error creating acceptor: {}", error)
		};

		MockGameService {
			port    : port,
			acceptor: acceptor,
			clients : Vec::new()
		}
	}

	pub fn accept_client(&mut self) {
		match self.acceptor.accept() {
			Ok(connection) => self.clients.push(connection),
			Err(error)     => panic!("Error accepting client: {}", error)
		}
	}

	pub fn send_perception(
		&self,
		perception: &Perception<EntityId, (EntityId, Entity)>
	) {
		for connection in self.clients.iter() {
			match connection.send_message(perception.to_string().as_slice()) {
				Ok(())     => (),
				Err(error) => panic!("Error sending perception: {}", error)
			}
		}
	}
}

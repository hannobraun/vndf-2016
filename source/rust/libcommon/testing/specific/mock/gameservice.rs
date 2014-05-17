use rand::random;

use net::{
	Acceptor,
	Connection
};
use protocol::Perception;


pub struct MockGameService {
	pub port: u16,

	acceptor: Acceptor,
	clients : Vec<Connection>
}

impl MockGameService {
	pub fn start() -> MockGameService {
		let port = random::<u16>() % 10000 + 40000;

		let acceptor = match Acceptor::new(port.to_str()) {
			Ok(acceptor) => acceptor,
			Err(error)   => fail!("Error creating acceptor: {}", error)
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
			Err(error)     => fail!("Error accepting client: {}", error)
		}
	}

	pub fn send_perception(&self, perception: &Perception) {
		for connection in self.clients.iter() {
			match connection.send_message(perception.to_str()) {
				Ok(())     => (),
				Err(error) => fail!("Error sending perception: {}", error)
			}
		}
	}
}

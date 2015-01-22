use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};

use acpe::protocol::Seq;

use common::protocol::Broadcast;
use game_service::Socket;

use self::receiver::Receiver;
use self::sender::Sender;


mod receiver;
mod sender;


pub type Clients = HashMap<SocketAddr, Client>;


pub struct Client {
	pub id           : String,
	pub last_active_s: f64,
	pub broadcast    : Option<String>,
}


pub struct Network {
	pub clients: Clients,

	last_actions: HashMap<SocketAddr, Seq>,
	socket      : Socket,
	receiver    : Receiver,
	sender      : Sender,
}

impl Network {
	pub fn new(port: Port) -> Network {
		Network {
			clients     : HashMap::new(),
			last_actions: HashMap::new(),
			socket      : Socket::new(port),
			receiver    : Receiver::new(),
			sender      : Sender::new(),
		}
	}

	pub fn send(&mut self, broadcasts: &Vec<Broadcast>) {
		self.sender.send(&mut self.socket, &mut self.clients, broadcasts, &mut self.last_actions);
	}

	pub fn receive(&mut self) {
		self.receiver.receive(&mut self.socket, &mut self.clients, &mut self.last_actions);
	}
}

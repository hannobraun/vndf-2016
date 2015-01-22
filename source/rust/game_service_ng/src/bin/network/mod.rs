use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::vec::Drain;

use acpe::protocol::Seq;

use common::protocol::{
	Broadcast,
	ClientEvent,
	Step,
};
use game_service::{
	ReceiveResult,
	Socket,
};

use self::sender::Sender;


mod sender;


pub type Clients = HashMap<SocketAddr, Client>;


pub struct Client {
	pub id           : String,
	pub last_active_s: f64,
	pub broadcast    : Option<String>,
}


pub struct Network {
	last_actions: HashMap<SocketAddr, Seq>,
	socket      : Socket,
	sender      : Sender,

	received: Vec<ReceiveResult>,
	events  : Vec<(SocketAddr, ClientEvent)>,
}

impl Network {
	pub fn new(port: Port) -> Network {
		Network {
			last_actions: HashMap::new(),
			socket      : Socket::new(port),
			sender      : Sender::new(),

			received: Vec::new(),
			events  : Vec::new(),
		}
	}

	pub fn send(&mut self, clients: &mut Clients, broadcasts: &Vec<Broadcast>) {
		self.sender.send(&mut self.socket, clients, broadcasts, &mut self.last_actions);
	}

	pub fn receive(&mut self) -> Drain<(SocketAddr, ClientEvent)> {
		self.socket.receive(&mut self.received);

		for result in self.received.drain() {
			match result {
				Ok((mut action, address)) => {
					self.last_actions.insert(address, action.header.id);

					for (_, step) in action.drain_update_items() {
						let event = match step {
							Step::Login =>
								ClientEvent::Login,
							Step::Broadcast(broadcast) =>
								ClientEvent::Broadcast(broadcast),
							Step::StopBroadcast =>
								ClientEvent::StopBroadcast,
						};

						self.events.push((address, event));
					}

					self.events.push((address, ClientEvent::Heartbeat));
				},
				Err((error, address)) => {
					print!(
						"Error receiving message from {}: {}\n",
						address, error
					);
				},
			}
		}

		self.events.drain()
	}
}

use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::rand::random;

use acpe::protocol::Seq;
use time::precise_time_s;

use common::protocol::{
	Broadcast,
	Step,
};
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
		for (address, step) in self.receiver.receive(&mut self.socket, &mut self.clients, &mut self.last_actions) {
			match step {
				Step::Login => {
					self.clients.insert(address, Client {
						id           : generate_id(),
						last_active_s: precise_time_s(),
						broadcast    : None,
					});
				},
				Step::Broadcast(broadcast) => {
					match self.clients.get_mut(&address) {
						Some(client) =>
							client.broadcast = Some(broadcast),
						None =>
							continue, // invalid, ignore
					}
				},
				Step::StopBroadcast => {
					match self.clients.get_mut(&address) {
						Some(client) =>
							client.broadcast = None,
						None =>
							continue, // invalid, ignore
					}
				},
			}
		}
	}
}


// TODO(85374284): The generated id should be guaranteed to be unique.
fn generate_id() -> String {
	fn random_char(min: char, max: char) -> char {
		let min = min as u8;
		let max = max as u8;

		((random::<u8>() % (max + 1 - min)) + min) as char
	}
	fn random_letter() -> char {
		random_char('A', 'Z')
	}
	fn random_letter_or_number() -> char {
		if random() {
			random_letter()
		}
		else {
			random_char('0', '9')
		}
	}

	let mut id = String::new();

	for _ in range(0u8, 3) {
		id.push(random_letter());
	}
	id.push('-');
	for _ in range(0u8, 5) {
		id.push(random_letter_or_number());
	}

	id
}

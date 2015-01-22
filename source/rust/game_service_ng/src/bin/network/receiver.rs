use std::collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::vec::Drain;

use acpe::protocol::Seq;
use time::precise_time_s;

use common::protocol::Step;
use game_service::{
	ReceiveResult,
	Socket,
};

use super::Clients;


pub struct Receiver {
	received: Vec<ReceiveResult>,
	steps   : Vec<(SocketAddr, Step)>,
}

impl Receiver {
	pub fn new() -> Receiver {
		Receiver {
			received: Vec::new(),
			steps   : Vec::new(),
		}
	}

	pub fn receive(&mut self, socket: &mut Socket, clients: &mut Clients, last_actions: &mut HashMap<SocketAddr, Seq>)
		-> Drain<(SocketAddr, Step)>
	{
		socket.receive(&mut self.received);

		for result in self.received.drain() {
			match result {
				Ok((mut action, address)) => {
					last_actions.insert(address, action.header.id);

					for (_, step) in action.drain_update_items() {
						self.steps.push((address, step));
					}

					match clients.get_mut(&address) {
						Some(client) => {
							client.last_active_s = precise_time_s();
						},
						None =>
							continue, // invalid, ignore
					}
				},
				Err((error, address)) => {
					print!(
						"Error receiving message from {}: {}\n",
						address, error
					);
				},
			}
		}

		self.steps.drain()
	}
}

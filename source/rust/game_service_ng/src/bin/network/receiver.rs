use std::collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::vec::Drain;

use acpe::protocol::Seq;

use common::protocol::{
	ClientEvent,
	Step,
};
use game_service::{
	ReceiveResult,
	Socket,
};


pub struct Receiver {
	received: Vec<ReceiveResult>,
	events  : Vec<(SocketAddr, ClientEvent)>,
}

impl Receiver {
	pub fn new() -> Receiver {
		Receiver {
			received: Vec::new(),
			events  : Vec::new(),
		}
	}

	pub fn receive(&mut self, socket: &mut Socket, last_actions: &mut HashMap<SocketAddr, Seq>)
		-> Drain<(SocketAddr, ClientEvent)>
	{
		socket.receive(&mut self.received);

		for result in self.received.drain() {
			match result {
				Ok((mut action, address)) => {
					last_actions.insert(address, action.header.id);

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

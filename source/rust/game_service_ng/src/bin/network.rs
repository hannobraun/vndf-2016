use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::vec::Drain;

use acpe::protocol::{
	Encoder,
	PerceptionHeader,
	Seq,
};

use common::protocol::{
	Broadcast,
	ClientEvent,
	Percept,
	Step,
};
use game_service::{
	ReceiveResult,
	Socket,
};


pub struct Network {
	last_actions: HashMap<SocketAddr, Seq>,
	socket      : Socket,

	encoder: Encoder,

	received: Vec<ReceiveResult>,
	events  : Vec<(SocketAddr, ClientEvent)>,
}

impl Network {
	pub fn new(port: Port) -> Network {
		Network {
			last_actions: HashMap::new(),
			socket      : Socket::new(port),

			encoder: Encoder::new(),

			received: Vec::new(),
			events  : Vec::new(),
		}
	}

	pub fn send(&mut self, recipients: &[(SocketAddr, String)], broadcasts: &Vec<Broadcast>)
	{
		for &(address, ref id) in recipients.iter() {
			let header = PerceptionHeader {
				confirm_action: self.last_actions[address],
				self_id       : Some(id.clone()),
			};
			// TODO(85373160): It's not necessary to keep resending all the
			//                 broadcasts every frame. The client should confirm
			//                 the last sent perception and the server should
			//                 only send what has changed. This requires a list
			//                 of destroyed entities in Perception.
			let mut broadcasts = broadcasts.clone();

			// TODO: This just keeps sending perceptions over and over, until
			//       all data is gone. This potentially means that there are
			//       always several perceptions "in-flight". This makes it
			//       complicated (i.e. impossible with the way things currently
			//       work) to figure out which perceptions has been received by
			//       the client, which means it can't be determined what data
			//       needs to be resent. The solution: Only keep one perception
			//       in-flight at any given time.
			let mut needs_to_send_perception = true;
			while needs_to_send_perception {
				send_perception(
					&mut self.encoder,
					&header,
					&mut broadcasts,
					&mut self.socket,
					address,
				);

				needs_to_send_perception = broadcasts.len() > 0;
			}
		}
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


fn send_perception(
	encoder    : &mut Encoder,
	header     : &PerceptionHeader<String>,
	broadcasts : &mut Vec<Broadcast>,
	socket     : &mut Socket,
	address    : SocketAddr,
) {
	let mut perception = encoder.message(header);
	loop {
		let broadcast = match broadcasts.pop() {
			Some(broadcast) => broadcast,
			None            => break,
		};

		let could_add = perception.update(
			&broadcast.sender,
			&Percept::Broadcast(broadcast.clone())
		);
		if !could_add {
			broadcasts.push(broadcast);
			break;
		}
	}

	let message = perception.encode();
	socket.send(message, address);
}

use std::collections::HashMap;
use std::io::net::ip::SocketAddr;

use acpe::protocol::{
	Encoder,
	PerceptionHeader,
	Seq,
};

use common::protocol::{
	Broadcast,
	Percept,
};
use game_service::Socket;

use super::Clients;


pub struct Sender {
	encoder: Encoder,
}

impl Sender {
	pub fn new() -> Sender {
		Sender {
			encoder: Encoder::new(),
		}
	}

	pub fn send(
		&mut self,
		socket      : &mut Socket,
		clients     : &mut Clients,
		broadcasts  : &Vec<Broadcast>,
		last_actions: &mut HashMap<SocketAddr, Seq>,
	) {
		for (&address, client) in clients.iter() {
			let header = PerceptionHeader {
				confirm_action: last_actions[address],
				self_id       : Some(client.id.clone()),
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
					socket,
					address,
				);

				needs_to_send_perception = broadcasts.len() > 0;
			}
		}
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

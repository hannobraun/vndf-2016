use std::io::net::ip::SocketAddr;

use acpe::protocol::{
	Encoder,
	PerceptionHeader,
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
		socket    : &mut Socket,
		clients   : &mut Clients,
		broadcasts: &Vec<Broadcast>,
	) {
		for (&address, client) in clients.iter() {
			let header = PerceptionHeader {
				confirm_action: client.last_action,
				self_id       : Some(client.id.clone()),
			};
			// TODO(85373160): It's not necessary to keep resending all the
			//                 broadcasts every frame. The client should confirm
			//                 the last sent perception and the server should
			//                 only send what has changed. This requires a list
			//                 of destroyed entities in Perception.
			let mut broadcasts = broadcasts.clone();

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

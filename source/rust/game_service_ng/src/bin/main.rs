#![feature(slicing_syntax)]


extern crate acpe;

extern crate game_service_ng;
extern crate protocol_ng;


use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::timer::sleep;
use std::time::Duration;

use acpe::MAX_PACKET_SIZE;

use protocol_ng::{
	Encoder,
	Percept,
	Seq,
	Step,
};

use game_service_ng::Socket;


struct Client {
	last_action: Seq,
	broadcast  : Option<String>,
}


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	let mut clients = HashMap::new();
	let mut socket  = Socket::new(port);
	let mut encoder = Encoder::new();

	loop {
		let received = socket.recv_from();
		for result in received.into_iter() {
			match result {
				Ok((action, address)) => {
					for step in action.steps.into_iter() {
						match step {
							Step::Login => {
								clients.insert(address, Client {
									last_action: action.seq,
									broadcast  : None,
								});
							},
							Step::Broadcast(broadcast) => {
								clients[address].broadcast = Some(broadcast);
							},
						}
					}

					clients[address].last_action = action.seq;
				},
				Err((error, address)) => {
					print!("Error receiving message from {}: {}", address, error);
					clients.remove(&address);
				},
			}
		}

		let broadcasts: Vec<String> = clients
			.iter()
			.filter_map(
				|(_, client)|
					client.broadcast.clone()
			)
			.collect();

		for (&address, client) in clients.iter() {
			let mut broadcasts: Vec<&str> = broadcasts
				.iter()
				.map(|broadcast| broadcast.as_slice())
				.collect();

			let mut needs_to_send_perception = true;
			while needs_to_send_perception {
				send_perception(
					&mut encoder,
					&mut broadcasts,
					&mut socket,
					client.last_action,
					address,
				);

				needs_to_send_perception = broadcasts.len() > 0;
			}
		}

		sleep(Duration::milliseconds(20));
	}
}


fn send_perception(
	encoder    : &mut Encoder,
	broadcasts : &mut Vec<&str>,
	socket     : &mut Socket,
	last_action: Seq,
	address    : SocketAddr,
) {
	let mut perception = encoder.message(last_action);
	loop {
		let broadcast = match broadcasts.pop() {
			Some(broadcast) => broadcast,
			None            => break,
		};

		if !perception.add(&Percept::Broadcast(broadcast.to_string())) {
			broadcasts.push(broadcast);
			break;
		}
	}

	let mut encode_buffer = [0, ..MAX_PACKET_SIZE];

	let message = perception
		.encode(&mut encode_buffer)
		.unwrap_or_else(|error|
			panic!("Error encoding perception: {}", error)
		);
	socket.send_to(message, address);
}

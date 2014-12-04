#![feature(slicing_syntax)]


extern crate getopts;

extern crate acpe;

extern crate common;
extern crate game_service;


use std::collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::io::timer::sleep;
use std::os;
use std::rand::random;
use std::time::Duration;

use acpe::MAX_PACKET_SIZE;
use acpe::protocol::{
	Encoder,
	PerceptionHeader,
	Seq,
};

use args::Args;
use common::protocol::{
	Broadcast,
	Percept,
	Step,
};
use game_service::Socket;


mod args;


struct Client {
	id         : String,
	last_action: Seq,
	broadcast  : Option<String>,
}


fn main() {
	let args = Args::parse(os::args().as_slice());

	let mut clients = HashMap::new();
	let mut socket  = Socket::new(args.port);
	let mut encoder = Encoder::new();

	print!("Listening on port {}\n", args.port);

	loop {
		let received = socket.recv_from();
		for result in received.into_iter() {
			match result {
				Ok((action, address)) => {
					for step in action.steps.into_iter() {
						match step {
							Step::Login => {
								clients.insert(address, Client {
									// TODO(83867146): Guarantee uniqueness
									id         : generate_id(),
									last_action: action.header.id,
									broadcast  : None,
								});
							},
							Step::Broadcast(broadcast) => {
								clients[address].broadcast = Some(broadcast);
							},
						}
					}

					clients[address].last_action = action.header.id;
				},
				Err((error, address)) => {
					print!("Error receiving message from {}: {}", address, error);
					clients.remove(&address);
				},
			}
		}

		let broadcasts: Vec<Broadcast> = clients
			.iter()
			.filter_map(
				|(_, client)|
					client.broadcast.clone().map(|broadcast|
						Broadcast {
							sender : client.id.clone(),
							message: broadcast,
						}
					)
			)
			.collect();

		for (&address, client) in clients.iter() {
			let header = PerceptionHeader {
				confirm_action: client.last_action,
				self_id       : Some(client.id.clone()),
			};
			let mut broadcasts = broadcasts.clone();

			let mut needs_to_send_perception = true;
			while needs_to_send_perception {
				send_perception(
					&mut encoder,
					&header,
					&mut broadcasts,
					&mut socket,
					address,
				);

				needs_to_send_perception = broadcasts.len() > 0;
			}
		}

		sleep(Duration::milliseconds(20));
	}
}


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

fn send_perception(
	encoder    : &mut Encoder,
	header     : &PerceptionHeader,
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

		if !perception.add(&Percept::Broadcast(broadcast.clone())) {
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

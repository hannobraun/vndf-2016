#![feature(slicing_syntax)]


extern crate game_service_ng;
extern crate protocol_ng;


use std::collections::HashMap;
use std::io::net::ip::Port;
use std::io::timer::sleep;
use std::time::Duration;

use protocol_ng::{
	Encoder,
	Step,
};

use game_service_ng::{
	ReceiveResult,
	Socket,
};


struct Client {
	last_action: u64,
	broadcast  : Option<String>,
}


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	let mut clients = HashMap::new();
	let mut socket  = Socket::new(port);
	let mut encoder = Encoder::new();

	loop {
		match socket.recv_from() {
			ReceiveResult::Message(action, address) => {
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
			ReceiveResult::None =>
				(),
			ReceiveResult::ClientError(error, address) => {
				print!("Error receiving message from {}: {}", address, error);
				clients.remove(&address);
			},
		}

		let broadcasts: Vec<String> = clients
			.iter()
			.filter_map(
				|(_, client)|
					client.broadcast.clone()
			)
			.collect();

		for (&address, client) in clients.iter() {
			// TODO(83504690): We need to make sure that the encoded perception
			//                 fits into a UDP packet. Research suggests that,
			//                 given typical MTU sizes, 512 bytes are a safe bet
			//                 for the maximum size.
			let mut encode_buffer = [0, ..512];
			let mut perception    = encoder.perception(client.last_action);

			for broadcast in broadcasts.iter() {
				perception.update(broadcast.as_slice());
			}

			let message = perception
				.encode(&mut encode_buffer)
				.unwrap_or_else(|error|
					panic!("Error encoding perception: {}", error)
				);
			socket.send_to(message, address);
		}

		sleep(Duration::milliseconds(20));
	}
}

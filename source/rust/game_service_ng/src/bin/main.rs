#![feature(slicing_syntax)]


extern crate protocol_ng;


use std::collections::HashMap;
use std::io::net::ip::Port;

use protocol_ng::{
	Action,
	Perception,
	Step,
};

use socket::Socket;


mod socket;


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	let mut clients = HashMap::new();
	let mut buffer  = [0u8, ..512];
	let mut socket  = Socket::new(port);

	print!("Listening on port {}\n", port);

	loop {
		let (action, address) = match socket.socket.recv_from(&mut buffer) {
			// TODO(83503278): Handle decoding errors.
			Ok((len, address)) => {
				let action =
					Action::from_json(
						String::from_utf8(
							buffer[.. len].to_vec()
						)
						.unwrap()
						.as_slice()
					)
					.unwrap();

				(action, address)
			},

			Err(error) => {
				print!("Error receiving data: {}\n", error);
				continue;
			},
		};

		for step in action.steps.into_iter() {
			match step {
				Step::Login => {
					clients.insert(address, None);
				},
				Step::Broadcast(broadcast) => {
					clients.insert(address, Some(broadcast));
				},
			}
		}

		let broadcasts: Vec<String> = clients
			.iter()
			.filter_map(
				|(_, broadcast)|
					broadcast.clone()
			)
			.collect();
		let perception = Perception {
			// TODO: The sequence number must be per-client.
			last_action: action.seq,
			broadcasts : broadcasts,
		};
		// TODO(83504690): We need to make sure that the encoded perception fits
		//                 into a UDP packet. Research suggests that, given
		//                 typical MTU sizes, 512 bytes are a safe bet for the
		//                 maximum size.
		let perception = perception.to_json();

		for (&address, _) in clients.iter() {
			socket.send_to(perception.as_bytes(), address);
		}
	}
}

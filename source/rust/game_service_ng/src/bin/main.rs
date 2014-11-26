#![feature(slicing_syntax)]


extern crate protocol_ng;


use std::collections::HashMap;
use std::io::net::ip::Port;
use std::io::net::udp::UdpSocket;

use protocol_ng::{
	Action,
	Perception,
};


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	let mut clients = HashMap::new();
	let mut buffer  = [0u8, ..512];
	let mut socket  = UdpSocket::bind(("0.0.0.0", port)).unwrap();

	print!("Listening on port {}\n", port);

	loop {
		let (message, address) = match socket.recv_from(&mut buffer) {
			// TODO: Handle decoding errors.
			Ok((len, address)) => {
				let message =
					Action::from_json(
						String::from_utf8(
							buffer[.. len].to_vec()
						)
						.unwrap()
						.as_slice()
					)
					.unwrap();

				(message, address)
			},

			Err(error) => {
				print!("Error receiving data: {}\n", error);
				continue;
			},
		};

		match message {
			Action::Login => {
				clients.insert(address, None);
			},
			Action::Broadcast(broadcast) => {
				clients.insert(address, Some(broadcast));
			},
		}

		let broadcasts: Vec<String> = clients
			.iter()
			.filter_map(
				|(_, broadcast)|
					broadcast.clone()
			)
			.collect();
		let perception = Perception {
			broadcasts: broadcasts,
		};
		// TODO: We need to make sure that the encoded perception fits into a
		//       UDP packet. Research suggests that, given typical MTU sizes,
		//       512 bytes are a safe bet for the maximum size.
		let perception = perception.to_json();

		for (&address, _) in clients.iter() {
			match socket.send_to(perception.as_bytes(), address) {
				Ok(())     => (),
				Err(error) =>
					print!(
						"Error sending data to {}: {}",
						address, error
					),
			}
		}
	}
}

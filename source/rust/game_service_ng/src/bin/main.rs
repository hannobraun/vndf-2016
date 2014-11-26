#![feature(slicing_syntax)]


extern crate protocol_ng;


use std::collections::HashSet;
use std::io::net::ip::Port;
use std::io::net::udp::UdpSocket;

use protocol_ng::{
	Action,
	Perception,
};


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	let mut clients = HashSet::new();
	let mut buffer  = [0u8, ..512];
	let mut socket  = UdpSocket::bind(("0.0.0.0", port)).unwrap();

	print!("Listening on port {}\n", port);

	// TODO: The game service sends at most a single broadcast that can be
	//       overwritten by every client. Instead we want one broadcast per
	//       client.
	let mut broadcast = None;

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
				clients.insert(address);
			},
			Action::Broadcast(string) => {
				broadcast = Some(string);
			},
		}

		let perception = Perception {
			broadcasts: broadcast.as_slice().to_vec(),
		};
		// TODO: We need to make sure that the encoded perception fits into a
		//       UDP packet. Research suggests that, given typical MTU sizes,
		//       512 bytes are a safe bet for the maximum size.
		let perception = perception.to_json();

		for &address in clients.iter() {
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

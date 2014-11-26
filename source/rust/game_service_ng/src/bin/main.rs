#![feature(slicing_syntax)]


extern crate protocol_ng;


use std::collections::HashSet;
use std::io::net::ip::Port;
use std::io::net::udp::UdpSocket;

use protocol_ng::Message;


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	let mut clients = HashSet::new();
	let mut buffer  = [0u8, ..512];
	let mut socket  = UdpSocket::bind(("0.0.0.0", port)).unwrap();

	print!("Listening on port {}\n", port);

	// TODO: This is just the broadcast the test case expects. This should be
	//       read from broadcasting clients instead.
	let broadcast = "This is a broadcast.";

	loop {
		let (message, address) = match socket.recv_from(&mut buffer) {
			// TODO: Handle decoding errors.
			Ok((len, address)) => {
				let message =
					Message::from_json(
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
			Message::Login => {
				clients.insert(address);
			},

			_ => print!("Unexpected message: {}\n", message),
		}

		for &address in clients.iter() {
			match socket.send_to(broadcast.as_bytes(), address) {
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

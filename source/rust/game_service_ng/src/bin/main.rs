#![feature(slicing_syntax)]


extern crate protocol_ng;


use std::collections::HashMap;
use std::io::net::ip::Port;
use std::io::timer::sleep;
use std::time::Duration;

use protocol_ng::{
	Perception,
	Step,
};

use socket::Socket;


mod socket;


fn main() {
	let port: Port = from_str(std::os::args()[1].as_slice()).unwrap();

	// TODO: Those two parallel HashMaps are now very nice.
	let mut clients = HashMap::new();
	let mut seqs    = HashMap::new();

	let mut socket = Socket::new(port);

	loop {
		match socket.recv_from() {
			Some((action, address)) => {
				seqs.insert(address, action.seq);

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
			},

			None => (),
		}

		let broadcasts: Vec<String> = clients
			.iter()
			.filter_map(
				|(_, broadcast)|
					broadcast.clone()
			)
			.collect();

		for (&address, _) in clients.iter() {
			let perception = Perception {
				last_action: seqs[address],
				broadcasts : broadcasts.clone(),
			};
			// TODO(83504690): We need to make sure that the encoded perception fits
			//                 into a UDP packet. Research suggests that, given
			//                 typical MTU sizes, 512 bytes are a safe bet for the
			//                 maximum size.
			let perception = perception.to_json();

			socket.send_to(perception.as_bytes(), address);
		}

		sleep(Duration::milliseconds(20));
	}
}

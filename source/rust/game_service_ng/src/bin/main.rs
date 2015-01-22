#![allow(unstable)]


extern crate getopts;

extern crate acpe;
extern crate time;

extern crate common;
extern crate game_service;


use std::collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::io::timer::sleep;
use std::os;
use std::rand::random;
use std::time::Duration;

use time::precise_time_s;

use args::Args;
use common::protocol::{
	Broadcast,
	ClientEvent,
};
use network::{
	Client,
	Network,
};


mod args;
mod network;


fn main() {
	let args = Args::parse(os::args().as_slice());

	let mut clients = HashMap::new();
	let mut network = Network::new(args.port);

	print!("Listening on port {}\n", args.port);

	loop {
		for (address, step) in network.receive() {
			match step {
				ClientEvent::Login => {
					clients.insert(address, Client {
						id           : generate_id(),
						last_active_s: precise_time_s(),
						broadcast    : None,
					});
				},
				ClientEvent::Heartbeat => {
					// Nothing to do here, really, as the the last active time
					// is updated below, no matter which event was received.
					()
				},
				ClientEvent::Broadcast(broadcast) => {
					match clients.get_mut(&address) {
						Some(client) =>
							client.broadcast = Some(broadcast),
						None =>
							continue, // invalid, ignore
					}
				},
				ClientEvent::StopBroadcast => {
					match clients.get_mut(&address) {
						Some(client) =>
							client.broadcast = None,
						None =>
							continue, // invalid, ignore
					}
				},
			}

			match clients.get_mut(&address) {
				Some(client) => {
					client.last_active_s = precise_time_s();
				},
				None =>
					continue, // invalid, ignore
			}
		}

		let now_s = precise_time_s();
		clients = clients
			.into_iter()
			.filter(|&(_, ref client)|
				// TODO(84970652): The timeout value should be configurable to
				//                 satisfy both real-world and testing
				//                 requirements.
				// TODO(84970652): Fine-tune timeout value. This is probably too
				//                 low for non-local connections.
				client.last_active_s + 0.05 > now_s
			)
			.collect();

		let recipients: Vec<(SocketAddr, String)> = clients
			.iter()
			.map(|(address, client)|
				(address.clone(), client.id.clone())
			)
			.collect();

		let broadcasts = clients
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

		network.send(recipients.as_slice(), &broadcasts);

		sleep(Duration::milliseconds(20));
	}
}


// TODO(85374284): The generated id should be guaranteed to be unique.
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

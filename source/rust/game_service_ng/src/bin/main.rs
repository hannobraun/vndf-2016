#![allow(unstable)]


extern crate getopts;

extern crate acpe;
extern crate time;

extern crate common;
extern crate game_service;


use std::collections::HashMap;
use std::io::timer::sleep;
use std::os;
use std::rand::random;
use std::time::Duration;

use time::precise_time_s;

use args::Args;

use common::game::Broadcast;
use common::protocol::{
	ClientEvent,
	ServerEvent,
};
use network::Network;


mod args;
mod network;


struct Client {
	pub id           : String,
	pub last_active_s: f64,
}


fn main() {
	let args = Args::parse(os::args().as_slice());

	let mut broadcasts = HashMap::new();
	let mut clients    = HashMap::new();
	let mut network    = Network::new(args.port);

	let mut incoming_events = Vec::new();
	let mut outgoing_events = Vec::new();

	print!("Listening on port {}\n", args.port);

	loop {
		outgoing_events.clear();

		for (address, event) in network.receive() {
			incoming_events.push((address, event));
		}

		for (address, event) in incoming_events.drain() {
			match event {
				ClientEvent::Login => {
					let client = Client {
						id           : generate_id(),
						last_active_s: precise_time_s(),
					};

					network.send(
						vec![(address, client.id.clone())].into_iter(),
						vec![ServerEvent::SelfId(client.id.clone())].iter()
					);
					clients.insert(address, client);
				},
				ClientEvent::Heartbeat => {
					// Nothing to do here, really, as the the last active time
					// is updated below, no matter which event was received.
					()
				},
				ClientEvent::StartBroadcast(message) => {
					match clients.get_mut(&address) {
						Some(client) => {
							broadcasts.insert(address, Broadcast {
								sender : client.id.clone(),
								message: message,
							});
						},
						None =>
							continue, // invalid, ignore
					}
				},
				ClientEvent::StopBroadcast => {
					match clients.get_mut(&address) {
						Some(client) => {
							broadcasts.remove(&address);
							outgoing_events.push(
								ServerEvent::StopBroadcast(client.id.clone())
							);
						},
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

		let     now_s     = precise_time_s();
		let mut to_remove = Vec::new();
		for (&address, client) in clients.iter() {
			// TODO(84970652): The timeout value should be configurable to
			//                 satisfy both real-world and testing
			//                 requirements.
			// TODO(84970652): Fine-tune timeout value. This is probably too low
			//                 for non-local connections.
			if client.last_active_s + 0.05 < now_s {
				to_remove.push(address);
			}
		}
		for address in to_remove.drain() {
			broadcasts.remove(&address);
			if let Some(client) = clients.remove(&address) {
				outgoing_events.push(ServerEvent::StopBroadcast(client.id));
			}
		}

		let recipients = clients
			.iter()
			.map(|(address, client)|
				(address.clone(), client.id.clone())
			);

		for (_, broadcast) in broadcasts.iter() {
			outgoing_events.push(
				ServerEvent::StartBroadcast(broadcast.clone())
			);
		}

		network.send(recipients, outgoing_events.iter());
		network.update();

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

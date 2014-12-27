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

use acpe::protocol::{
	Encoder,
	PerceptionHeader,
	Seq,
};
use time::precise_time_s;

use args::Args;
use common::protocol::{
	Broadcast,
	Percept,
	Step,
};
use game_service::Socket;


mod args;


struct Client {
	id           : String,
	last_action  : Seq,
	last_active_s: f64,
	broadcast    : Option<String>,
}


fn main() {
	let args = Args::parse(os::args().as_slice());

	let mut clients = HashMap::new();
	let mut socket  = Socket::new(args.port);
	let mut encoder = Encoder::new();

	print!("Listening on port {}\n", args.port);

	loop {
		let received = socket.receive();
		for result in received.into_iter() {
			match result {
				Ok((action, address)) => {
					for step in action.steps.into_iter() {
						match step {
							Step::Login => {
								clients.insert(address, Client {
									id           : generate_id(),
									last_action  : action.header.id,
									last_active_s: precise_time_s(),
									broadcast    : None,
								});
							},
							Step::Broadcast(broadcast) => {
								match clients.get_mut(&address) {
									Some(client) =>
										client.broadcast = Some(broadcast),
									None =>
										continue, // invalid, ignore
								}
							},
							Step::StopBroadcast => {
								match clients.get_mut(&address) {
									Some(client) =>
										client.broadcast = None,
									None =>
										continue, // invalid, ignore
								}
							},
						}
					}

					match clients.get_mut(&address) {
						Some(client) => {
							client.last_action   = action.header.id;
							client.last_active_s = precise_time_s();
						},
						None =>
							continue, // invalid, ignore
					}
				},
				Err((error, address)) => {
					print!("Error receiving message from {}: {}", address, error);
					clients.remove(&address);
				},
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
			// TODO(84970652): It's not necessary to keep resending all the
			//                 broadcasts every frame. The client should confirm
			//                 the last sent perception and the server should
			//                 only send what has changed. This requires a list
			//                 of destroyed entities in Perception.
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

	let message = perception.encode();
	socket.send(message, address);
}

#![allow(unstable)]


extern crate getopts;

extern crate acpe;
extern crate time;

extern crate common;
extern crate game_service;


use std::collections::HashMap;
use std::io::timer::sleep;
use std::os;
use std::time::Duration;

use time::precise_time_s;

use args::Args;
use common::protocol::Broadcast;
use game_service::Socket;
use network::{
	Receiver,
	Sender,
};


mod args;
mod network;


fn main() {
	let args = Args::parse(os::args().as_slice());

	let mut clients  = HashMap::new();
	let mut socket   = Socket::new(args.port);
	let mut receiver = Receiver::new();
	let mut sender   = Sender::new();

	print!("Listening on port {}\n", args.port);

	loop {
		receiver.receive(&mut socket, &mut clients);

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

		sender.send(&mut socket, &mut clients, &broadcasts);

		sleep(Duration::milliseconds(20));
	}
}

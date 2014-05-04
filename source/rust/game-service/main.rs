extern crate collections;
extern crate common;
extern crate getopts;
extern crate libc;
extern crate time;


use std::os;

use clients::Clients;
use eventhandler::EventHandler;
use events::{
	Init,
	Update
};
use network::Network;


mod args;
mod clients;
mod events;
mod eventbuffer;
mod eventhandler;
mod network;


fn main() {
	let args = match args::parse() {
		Some(args) => args,

		None => {
			os::set_exit_status(1);
			return;
		}
	};

	let mut network       = Network::new(args.port);
	let mut event_handler = EventHandler::new();
	let mut clients       = Clients::new();

	let frame_time_in_ms = args.frame_time;

	event_handler.incoming.push(Init);

	loop {
		network.update(frame_time_in_ms, &mut event_handler.incoming, &mut clients);

		event_handler.incoming.push(Update(frame_time_in_ms as f64 / 1000.0));
		event_handler.handle(&mut clients, &mut network.incoming);
	}
}

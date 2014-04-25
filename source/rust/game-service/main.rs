extern crate collections;
extern crate common;
extern crate libc;
extern crate time;


use clients::Clients;
use eventhandler::{
	EventHandler,
	Update
};
use network::Network;


mod args;
mod clients;
mod eventbuffer;
mod eventhandler;
mod network;


fn main() {
	print!("Game Service started.\n");

	let network           = Network::new(args::port());
	let mut event_handler = EventHandler::new();
	let mut clients       = Clients::new();

	loop {
		let frame_time_in_ms = 1000;

		network.update(frame_time_in_ms, &mut event_handler.incoming);

		event_handler.incoming.push(Update(frame_time_in_ms as f64 / 1000.0));
		event_handler.handle(&mut clients);
	}
}

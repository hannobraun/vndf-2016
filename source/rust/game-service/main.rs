extern crate collections;
extern crate common;
extern crate libc;
extern crate time;

use common::net::Acceptor;

use clients::Clients;
use events::Events;

mod args;
mod clients;
mod events;


fn main() {
	print!("Game Service started.\n");

	let acceptor      = Acceptor::create(args::port());
	let mut events    = Events::new();
	let mut clientMap = Clients::new(4);

	loop {
		let frameTimeInMs = 50;

		acceptor.accept(frameTimeInMs, |connection| {
			events.push(events::Connect(connection))
		});

		events.push(events::Update);
		events::handle_events(events, clientMap, frameTimeInMs as uint);
	}
}

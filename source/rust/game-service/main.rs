extern crate collections;
extern crate common;
extern crate extra;
extern crate time;

use std::libc;

use clients::Clients;
use events::Events;

mod args;
mod clients;
mod events;
mod net;
mod protocol;
mod util;


fn main() {
	util::log("Game Service started.");

	let net = net::init(args::port());

	let mut events    = Events::new();
	let mut clientMap = Clients::new(4);

	loop {
		let frameTimeInMs: uint = 50;
		let numberOfEvents = net::number_of_events(&net, frameTimeInMs as i32) as int;
		handle_connects(numberOfEvents, net.serverFD, events);
		events.push(events::Update);
		events::handle_events(events, clientMap, frameTimeInMs);
	}
}

fn handle_connects(numberOfEvents: int, serverFD: libc::c_int, events: &mut events::Events) {
	let mut i = 0;
	while i < numberOfEvents {
		let clientFD = net::accept_client(serverFD);

		events.push(events::Connect(clientFD));

		i += 1;
	}
}

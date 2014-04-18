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
mod net;


fn main() {
	print!("Game Service started.\n");

	let acceptor = Acceptor::create(args::port());

	let mut events    = Events::new();
	let mut clientMap = Clients::new(4);

	loop {
		let frameTimeInMs = 50;
		let numberOfEvents = net::number_of_events(&acceptor, frameTimeInMs) as int;
		handle_connects(numberOfEvents, acceptor.fd, events);
		events.push(events::Update);
		events::handle_events(events, clientMap, frameTimeInMs as uint);
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

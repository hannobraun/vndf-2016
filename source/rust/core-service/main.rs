extern mod collections;
extern mod common;
extern mod extra;

use std::libc;

use clients::Clients;
use events::Events;

pub mod clients;
pub mod events;
pub mod net;
pub mod protocol;
pub mod util;


fn main() {
	util::log("Core Service started.");

	let net = net::init("34481");

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

extern crate collections;
extern crate common;
extern crate libc;
extern crate time;

use common::net::Acceptor;
use common::net::epoll;
use common::net::epoll::EPoll;

use clients::Clients;
use events::Events;

mod args;
mod clients;
mod events;


fn main() {
	print!("Game Service started.\n");

	let epoll = match EPoll::create() {
		Ok(epoll)  => epoll,
		Err(error) => fail!("Error initializing epoll: {}", error)
	};

	let acceptor      = Acceptor::create(args::port(), epoll);
	let mut events    = Events::new();
	let mut clientMap = Clients::new(4);

	match epoll.add(acceptor.fd, epoll::ffi::EPOLLIN) {
		Err(error) =>
			fail!("Error registering server socket with epoll: {}", error),

		_ => ()
	}

	loop {
		let frameTimeInMs = 50;

		let number_of_events = match epoll.wait(frameTimeInMs) {
			Ok(number_of_events) => number_of_events,

			Err(error) => fail!("Error while waiting for events: {}", error)
		};

		for _ in range(0, number_of_events) {
			match acceptor.accept() {
				Ok(connection) =>
					events.push(events::Connect(connection)),

				Err(error) =>
					fail!("Error accepting connection: {}", error)
			}
		}

		events.push(events::Update);
		events::handle_events(events, clientMap, frameTimeInMs as uint);
	}
}

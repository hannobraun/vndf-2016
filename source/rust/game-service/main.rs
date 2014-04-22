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

	let acceptor      = Acceptor::create(args::port());
	let mut events    = Events::new();
	let mut clientMap = Clients::new();

	match epoll.add(acceptor.fd, epoll::ffi::EPOLLIN) {
		Err(error) =>
			fail!("Error registering server socket with epoll: {}", error),

		_ => ()
	}

	loop {
		let frameTimeInMs = 50;

		let result = epoll.wait(frameTimeInMs, |fd| {
			if fd == acceptor.fd {
				match acceptor.accept() {
					Ok(connection) => {
						match epoll.add(connection.fd, epoll::ffi::EPOLLIN) {
							Ok(()) => (),

							Err(error) =>
								fail!("Error adding to epoll: {}", error)
						}
						events.push(events::Connect(connection));
					},

					Err(error) =>
						fail!("Error accepting connection: {}", error)
				}
			}
			else {
				events.push(events::DataReceived(fd))
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => fail!("Error while waiting for events: {}", error)
		};

		events.push(events::Update);
		events::handle_events(events, clientMap, frameTimeInMs as uint);
	}
}

extern crate collections;
extern crate common;
extern crate libc;
extern crate time;

use common::net::Acceptor;
use common::net::epoll;
use common::net::epoll::EPoll;

use clients::Clients;
use events::EventHandler;

mod args;
mod clients;
mod eventbuffer;
mod events;


fn main() {
	print!("Game Service started.\n");

	let epoll = match EPoll::create() {
		Ok(epoll)  => epoll,
		Err(error) => fail!("Error initializing epoll: {}", error)
	};

	let acceptor          = Acceptor::create(args::port());
	let mut event_handler = EventHandler::new();
	let mut clients       = Clients::new();

	match epoll.add(acceptor.fd, epoll::ffi::EPOLLIN) {
		Err(error) =>
			fail!("Error registering server socket with epoll: {}", error),

		_ => ()
	}

	loop {
		let frame_time_in_ms = 50;

		let result = epoll.wait(frame_time_in_ms, |fd| {
			if fd == acceptor.fd {
				match acceptor.accept() {
					Ok(connection) => {
						match epoll.add(connection.fd, epoll::ffi::EPOLLIN) {
							Ok(()) => (),

							Err(error) =>
								fail!("Error adding to epoll: {}", error)
						}
						event_handler.incoming.push(events::Connect(connection));
					},

					Err(error) =>
						fail!("Error accepting connection: {}", error)
				}
			}
			else {
				event_handler.incoming.push(events::DataReceived(fd))
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => fail!("Error while waiting for events: {}", error)
		};

		event_handler.incoming.push(events::Update(frame_time_in_ms as f64 / 1000.0));
		event_handler.handle(&mut clients);
	}
}

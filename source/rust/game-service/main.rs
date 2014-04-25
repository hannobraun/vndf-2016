extern crate collections;
extern crate common;
extern crate libc;
extern crate time;


use common::net::epoll;

use clients::Clients;
use eventhandler::{
	Connect,
	DataReceived,
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

		let result = network.epoll.wait(frame_time_in_ms, |fd| {
			if fd == network.acceptor.fd {
				match network.acceptor.accept() {
					Ok(connection) => {
						match network.epoll.add(connection.fd, epoll::ffi::EPOLLIN) {
							Ok(()) => (),

							Err(error) =>
								fail!("Error adding to epoll: {}", error)
						}
						event_handler.incoming.push(Connect(connection));
					},

					Err(error) =>
						fail!("Error accepting connection: {}", error)
				}
			}
			else {
				event_handler.incoming.push(DataReceived(fd))
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => fail!("Error while waiting for events: {}", error)
		};

		event_handler.incoming.push(Update(frame_time_in_ms as f64 / 1000.0));
		event_handler.handle(&mut clients);
	}
}

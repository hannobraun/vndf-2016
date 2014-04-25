use common::net::Acceptor;
use common::net::epoll;
use common::net::epoll::EPoll;

use eventbuffer::EventBuffer;
use eventhandler::{
	Connect,
	DataReceived,
	Event,
};


pub struct Network {
	pub epoll   : EPoll,
	pub acceptor: Acceptor
}

impl Network {
	pub fn new(port: ~str) -> Network {
		let epoll = match EPoll::create() {
			Ok(epoll)  => epoll,
			Err(error) => fail!("Error initializing epoll: {}", error)
		};

		let acceptor = Acceptor::create(port);

		match epoll.add(acceptor.fd, epoll::ffi::EPOLLIN) {
			Ok(()) => (),

			Err(error) =>
				fail!("Error registering server socket with epoll: {}", error)
		}

		Network {
			epoll   : epoll,
			acceptor: acceptor
		}
	}

	pub fn update(&self, timeout_in_ms: u32, events: &mut EventBuffer<Event>) {
		let result = self.epoll.wait(timeout_in_ms, |fd| {
			if fd == self.acceptor.fd {
				match self.acceptor.accept() {
					Ok(connection) => {
						match self.epoll.add(connection.fd, epoll::ffi::EPOLLIN) {
							Ok(()) => (),

							Err(error) =>
								fail!("Error adding to epoll: {}", error)
						}
						events.push(Connect(connection));
					},

					Err(error) =>
						fail!("Error accepting connection: {}", error)
				}
			}
			else {
				events.push(DataReceived(fd))
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => fail!("Error while waiting for events: {}", error)
		};
	}
}

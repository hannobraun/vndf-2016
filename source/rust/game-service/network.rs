use std::comm::{
	Disconnected,
	Empty
};

use common::net::Acceptor;
use common::net::epoll;
use common::net::epoll::EPoll;

use clients::Clients;
use eventbuffer::EventBuffer;
use events::{
	Close,
	DataReceived,
	Enter,
	GameEvent,
	Leave,
	NetworkEvent
};


pub struct Network {
	epoll   : EPoll,
	acceptor: Acceptor,
	events  : Receiver<NetworkEvent>,

	pub event_sender: Sender<NetworkEvent>
}

impl Network {
	pub fn new(port: &str) -> Network {
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

		let (sender, receiver) = channel();

		Network {
			epoll   : epoll,
			acceptor: acceptor,
			events  : receiver,

			event_sender: sender
		}
	}

	pub fn update(&mut self, timeout_in_ms: u32, events: &mut EventBuffer<GameEvent>, clients: &mut Clients) {
		loop {
			match self.events.try_recv() {
				Ok(event) => match event {
					Close(fd) => match clients.remove(fd) {
						Some(client) => {
							client.conn.close();
							events.push(Leave(fd));
						},

						None => ()
					}
				},

				Err(error) => match error {
					Empty        => break,
					Disconnected => fail!("Unexpected error: {}", error)
				}
			}
		}

		let result = self.epoll.wait(timeout_in_ms, |fd| {
			if fd == self.acceptor.fd {
				let connection = match self.acceptor.accept() {
					Ok(connection) => connection,

					Err(error) =>
						fail!("Error accepting connection: {}", error)
				};

				match self.epoll.add(connection.fd, epoll::ffi::EPOLLIN) {
					Ok(()) => (),

					Err(error) =>
						fail!("Error adding to epoll: {}", error)
				}

				events.push(Enter(connection));
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

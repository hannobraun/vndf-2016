use std::comm::{
	Disconnected,
	Empty
};

use common::net::Acceptor;
use common::net::epoll;
use common::net::epoll::EPoll;

use clients::Clients;
use events::{
	Close,
	DataReceived,
	Enter,
	GameEvent,
	Leave,
	NetworkEvent
};


pub struct Network {
	pub events: Sender<NetworkEvent>,

	incoming: Receiver<NetworkEvent>,
	epoll   : EPoll,
	acceptor: Acceptor
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
			events  : sender,
			incoming: receiver,
			epoll   : epoll,
			acceptor: acceptor

		}
	}

	pub fn update(&mut self, timeout_in_ms: u32, events: &mut Sender<GameEvent>, clients: &mut Clients) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => match event {
					Close(fd) => match clients.remove(fd) {
						Some(client) => {
							client.conn.close();
							events.send(Leave(fd));
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

				events.send(Enter(connection));
			}
			else {
				events.send(DataReceived(fd))
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => fail!("Error while waiting for events: {}", error)
		};
	}
}

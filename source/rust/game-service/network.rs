use libc::c_int;
use std::comm::{
	Disconnected,
	Empty
};

use common::net::Acceptor;
use common::net::epoll;
use common::net::epoll::EPoll;
use common::protocol::Action;

use clients::Clients;
use events::{
	Action,
	Close,
	Enter,
	GameEvent,
	Leave,
	Message,
	NetworkEvent
};


pub type ClientId = uint;


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

		let acceptor = match Acceptor::new(port) {
			Ok(acceptor) => acceptor,
			Err(error)   => fail!("Error creating acceptor: {}", error)
		};

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

	pub fn update(&mut self, timeout_in_ms: u32, game: &mut Sender<GameEvent>, clients: &mut Clients) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => match event {
					Message(recipients, message) => {
						for &id in recipients.iter() {
							let (_, connection) = match clients.client_by_fd(id as c_int) {
								Some(connection) => connection,
								None             => return
							};

							match connection.send_message(message.to_str()) {
								Ok(())     => (),
								Err(error) => self.events.send(Close(id))
							}
						}
					},

					Close(fd) => match clients.remove(fd) {
						Some(conn) => {
							conn.close();
							game.send(Leave(fd));
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

				let (id, _) = clients.add(connection);

				game.send(Enter(id));
			}
			else {
				let (client_id, conn) = match clients.client_by_fd(fd) {
					Some(result) => result,
					None         => return
				};

				let result = conn.receive_messages(|raw_message| {
					let action = match Action::from_str(raw_message) {
						Ok(message) => message,

						Err(error) =>
							fail!("Error decoding message: {}", error)
					};

					game.send(Action(fd as ClientId, action));
				});

				match result {
					Ok(()) => (),
					Err(_) => self.events.send(Close(client_id))
				}
			}
		});

		match result {
			Ok(())     => (),
			Err(error) => fail!("Error while waiting for events: {}", error)
		};
	}
}

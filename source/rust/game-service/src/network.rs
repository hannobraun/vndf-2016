use std::collections::HashMap;
use std::comm::{
	Disconnected,
	Empty
};

use common::net::{
	Acceptor,
	Connection,
	ConnId,
};
use common::net::epoll;
use common::net::epoll::EPoll;
use common::protocol::Action;

use events::{
	Action,
	Close,
	Enter,
	GameEvent,
	Leave,
	Message,
	NetworkEvent
};


pub struct Network {
	pub events: Sender<NetworkEvent>,

	incoming   : Receiver<NetworkEvent>,
	epoll      : EPoll,
	acceptor   : Acceptor,
	connections: HashMap<ConnId, Connection>
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
			events     : sender,
			incoming   : receiver,
			epoll      : epoll,
			acceptor   : acceptor,
			connections: HashMap::new()

		}
	}

	pub fn update(&mut self, timeout_in_ms: u32, game: &mut Sender<GameEvent>) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => match event {
					Message(recipients, message) => {
						for &id in recipients.iter() {
							let connection = match self.connections.find(&id) {
								Some(connection) => connection,
								None             => return
							};

							match connection.send_message(message.to_string().as_slice()) {
								Ok(())     => (),
								Err(error) => self.events.send(Close(id, error))
							}
						}
					},

					Close(id, _) => match self.connections.pop(&id) {
						Some(conn) => {
							conn.close();
							game.send(Leave(id));
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

		let mut to_accept = Vec::new();

		match self.epoll.wait(timeout_in_ms) {
			Ok(fds) => for &fd in fds.iter() {
				if fd == self.acceptor.fd {
					to_accept.push(fd);
				}
				else {
					let client_id = fd as ConnId;

					let conn = match self.connections.find_mut(&client_id) {
						Some(result) => result,
						None         => return
					};

					let result = conn.receive_messages(|raw_message| {
						let action = match Action::from_string(raw_message.as_slice()) {
							Ok(message) => message,

							Err(error) =>
								fail!("Error decoding message: {}", error)
						};

						game.send(Action(fd as ConnId, action));
					});

					match result {
						Ok(())     => (),
						Err(error) => self.events.send(Close(client_id, error))
					}
				}
			},

			Err(error) => fail!("Error while waiting for events: {}", error)
		}

		for _ in to_accept.iter() {
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

			let client_id = connection.fd as ConnId;
			self.connections.insert(client_id, connection);
			game.send(Enter(client_id));
		}
	}
}

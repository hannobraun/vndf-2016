use libc::c_int;

use common::physics::{Body, Radians, Vec2};
use common::net::Connection;
use common::protocol::{
	Command,
	Perception,
	Ship
};

use clients::{Client, Clients};
use eventbuffer::EventBuffer;
use events::{
	CommandEvent,
	Connect,
	CreateEvent,
	DataReceived,
	Disconnect,
	GameEvent,
	Init,
	Update
};


pub struct EventHandler {
	pub incoming: EventBuffer<GameEvent>
}


impl EventHandler {
	pub fn new() -> EventHandler {
		EventHandler {
			incoming: EventBuffer::new()
		}
	}

	pub fn handle(&mut self, clients: &mut Clients) {
		loop {
			match self.incoming.pop() {
				Some(event) => {
					print!("Incoming event: {}\n", event);

					match event {
						Init =>
							(), // nothing do do, it just exists for the logging
						Connect(connection) =>
							self.on_connect(connection, clients),
						Disconnect(clientId) =>
							self.on_disconnect(clientId, clients),
						DataReceived(fd) =>
							self.on_data_received(fd, clients),
						CreateEvent(client_id) =>
							self.on_create(client_id, clients),
						Update(frame_time_in_s) =>
							self.on_update(clients, frame_time_in_s),
						CommandEvent(client_id, attitude) =>
							self.on_command(client_id, attitude, clients)
					}
				},

				None => break
			}
		}
	}

	fn on_connect(&mut self, connection: Connection, clients: &mut Clients) {
		let velocity = Vec2(30.0, 10.0);

		let ship = Body {
			position: Vec2::zero(),
			velocity: velocity,
			attitude: Radians::from_vec(velocity)
		};

		let new_client = Client::new(connection, ship);
		let (client_id, _) = clients.add(new_client);

		self.incoming.push(CreateEvent(client_id))
	}

	fn on_disconnect(&mut self, removed_id: uint, clients: &mut Clients) {
		clients.remove(removed_id);
	}

	fn on_data_received(&mut self, fd: c_int, clients: &mut Clients) {
		let (client_id, client) = match clients.client_by_fd(fd) {
			Some(result) => result,
			None         => return
		};

		let result = client.conn.receive_messages(|raw_message| {
			let command = match Command::from_str(raw_message) {
				Ok(message) => message,
				Err(error)  => fail!("Error decoding message: {}", error)
			};

			self.incoming.push(CommandEvent(fd, command.attitude));
		});

		match result {
			Ok(()) => (),
			Err(_) => self.incoming.push(Disconnect(client_id))
		}
	}

	fn on_create(&mut self, created_id: uint, clients: &mut Clients) {
		clients.mut_each(|client_id, client| {
			if client_id == created_id {
				client.created = true;
			}
		});
	}

	fn on_update(&mut self, clients: &mut Clients, dTimeInS: f64) {
		clients.mut_each(|_, client| {
			if client.created {
				client.ship.velocity = client.ship.attitude.to_vec() * 30.0;
				client.ship.position =
					client.ship.position + client.ship.velocity * dTimeInS;
			}
		});

		let mut ships = Vec::new();
		clients.each(|client_id, client| {
			if client.created {
				ships.push(Ship {
					id  : client_id,
					body: client.ship
				});
			}
		});

		clients.each(|client_id, client| {
			let update = Perception {
				self_id: client_id,
				ships  : ships.as_slice().to_owned()
			};
			let message = update.to_str();

			match client.conn.send_message(message) {
				Err(_) => self.incoming.push(Disconnect(client_id)),
				_      => ()
			};
		});
	}

	fn on_command(&self, fd: c_int, attitude: Radians, clients: &mut Clients) {
		match clients.client_by_fd(fd) {
			Some((_, client)) => client.ship.attitude = attitude,
			None              => ()
		}
	}
}

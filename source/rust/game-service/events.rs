use libc::c_int;

use common::physics::{Body, Radians, Vec2};
use common::net::Connection;
use common::protocol;
use common::protocol::{Command, Create, Message, Remove, SelfInfo, Update};

use clients::{Client, Clients};
use eventbuffer::Events;


#[deriving(Eq, Show)]
pub enum Event {
	Connect(Connection),
	Disconnect(uint),
	DataReceived(c_int),
	CreateEvent(uint),
	CommandEvent(c_int, Radians),
	Update
}


pub fn handle_events(events: &mut Events, clients: &mut Clients, frameTimeInMs: uint) {
	loop {
		match events.pull() {
			Some(event) =>
				match event {
					Connect(connection)    => on_connect(connection, clients, events),
					Disconnect(clientId)   => on_disconnect(clientId, clients, events),
					DataReceived(fd)       => on_data_received(fd, clients, events),
					CreateEvent(client_id) => on_create(client_id, clients, events),
					Update                 => on_update(clients, events, frameTimeInMs as f64 / 1000.0),

					CommandEvent(client_id, attitude) =>
						on_command(client_id, attitude, clients)
				},

			None => break
		}
	}
}

fn on_connect(connection: Connection, clients: &mut Clients, events: &mut Events) {
	let velocity = Vec2 {
		x: 30.0,
		y: 10.0
	};

	let ship = Body {
		position: Vec2 {
			x: 0.0,
			y: 0.0
		},
		velocity: velocity,
		attitude: Radians::from_vec(velocity)
	};

	let new_client = Client::new(connection, ship);

	match clients.add(new_client) {
		Ok((client_id, client)) => {
			let message = SelfInfo(SelfInfo {
				id: client_id
			});

			match client.conn.send_message(message.to_str()) {
				Err(_) => events.push(Disconnect(client_id)),
				_      => ()
			}

			events.push(CreateEvent(client_id))
		},

		Err(client) => client.conn.close()
	}
}

fn on_disconnect(removed_id: uint, clients: &mut Clients, events: &mut Events) {
	clients.remove(removed_id);

	clients.each(|client_id, client| {
		let message = Remove(Remove {
			id: removed_id
		});

		match client.conn.send_message(message.to_str()) {
			Err(_) => events.push(Disconnect(client_id)),
			_      => ()
		}
	})
}

fn on_data_received(fd: c_int, clients: &mut Clients, events: &mut Events) {
	let (client_id, client) = match clients.client_by_fd(fd) {
		Some(result) => result,
		None         => return
	};

	let result = client.conn.receive_messages(|message| {
		match Message::from_str(message) {
			Command(command) =>
				events.push(CommandEvent(fd, command.attitude)),

			_ => fail!("Received unexpected message from client: {}", message)
		}
	});

	match result {
		Ok(()) => (),
		Err(_) => events.push(Disconnect(client_id))
	}
}

fn on_create(created_id: uint, clients: &mut Clients, events: &mut Events) {
	clients.mut_each(|client_id, client| {
		if client_id == created_id {
			client.created = true;
		}

		let message = Create(Create {
			id  : created_id,
			kind: ~"ship"
		});

		match client.conn.send_message(message.to_str()) {
			Err(_) => events.push(Disconnect(client_id)),
			_      => ()
		}
	});
}

fn on_update(clients: &mut Clients, events: &mut Events, dTimeInS: f64) {
	clients.mut_each(|_, client| {
		if client.created {
			client.ship.velocity = client.ship.attitude.to_vec() * 30.0;
			client.ship.position =
				client.ship.position + client.ship.velocity * dTimeInS;
		}
	});

	clients.each(|client_a_id, clientA| {
		clients.each(|client_b_id, clientB| {
			if clientB.created {
				let message = protocol::Update(Update {
					id  : client_b_id,
					body: clientB.ship
				});

				match clientA.conn.send_message(message.to_str()) {
					Err(_) => events.push(Disconnect(client_a_id)),
					_      => ()
				}
			}
		})
	});
}

fn on_command(fd: c_int, attitude: Radians, clients: &mut Clients) {
	match clients.client_by_fd(fd) {
		Some((_, client)) => client.ship.attitude = attitude,
		None              => ()
	}
}

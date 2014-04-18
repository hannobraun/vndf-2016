use collections::Deque;
use collections::RingBuf;
use libc;

use common::physics::{Body, Radians, Vec2};
use common::net::Connection;
use common::protocol;
use common::protocol::{Create, Remove, SelfInfo, Update};

use clients::Clients;

extern {
	fn close(fd: libc::c_int) -> libc::c_int;
}


pub struct Events {
	buffer: ~Deque<Event>
}

#[deriving(Eq, Show)]
pub enum Event {
	Connect(Connection),
	Disconnect(uint),
	Update
}

impl Events {
	pub fn new() -> ~Events {
		~Events {
			buffer: ~RingBuf::<Event>::new() }
	}

	pub fn push(&mut self, event: Event) {
		self.buffer.push_back(event)
	}

	pub fn pull(&mut self) -> Option<Event> {
		self.buffer.pop_front()
	}
}


pub fn handle_events(events: &mut Events, clients: &mut Clients, frameTimeInMs: uint) {
	loop {
		match events.pull() {
			Some(event) =>
				match event {
					Connect(connection)  => on_connect(connection, clients, events),
					Disconnect(clientId) => on_disconnect(clientId, clients, events),
					Update               => on_update(clients, events, frameTimeInMs as f64 / 1000.0)
				},

			None => break
		}
	}
}

fn on_connect(connection: Connection, clients: &mut Clients, events: &mut Events) {
	let ship = Body {
		position: Vec2 {
			x: 0.0,
			y: 0.0
		},
		velocity: Vec2 {
			x: 30.0,
			y: 10.0
		},
		attitude: Radians(0.0)
	};

	match clients.add(connection, ship) {
		Some(client) => {
			let message = SelfInfo(SelfInfo {
				id: client.id
			});

			match client.conn.send_message(message.to_str()) {
				Err(_) => events.push(Disconnect(client.id)),
				_      => ()
			}

			clients.each(|clientB| {
				let message = Create(Create {
					id  : client.id,
					kind: ~"ship"
				});

				match clientB.conn.send_message(message.to_str()) {
					Err(_) => events.push(Disconnect(clientB.id)),
					_      => ()
				}
			});
		},

		None =>
			unsafe {
				close(connection.fd);
			}
	}
}

fn on_disconnect(clientId: uint, clients: &mut Clients, events: &mut Events) {
	clients.remove(clientId);

	clients.each(|client| {
		let message = Remove(Remove {
			id: clientId
		});

		match client.conn.send_message(message.to_str()) {
			Err(_) => events.push(Disconnect(client.id)),
			_      => ()
		}
	})
}

fn on_update(clients: &mut Clients, events: &mut Events, dTimeInS: f64) {
	clients.mut_each(|client| {
		client.ship.position =
			client.ship.position + client.ship.velocity * dTimeInS;
	});

	clients.each(|clientA| {
		clients.each(|clientB| {
			let message = protocol::Update(Update {
				id  : clientB.id,
				body: clientB.ship
			});

			match clientA.conn.send_message(message.to_str()) {
				Err(_) => events.push(Disconnect(clientA.id)),
				_      => ()
			}
		})
	});
}

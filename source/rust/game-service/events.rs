use collections::Deque;
use collections::RingBuf;

use common::physics::{Body, Radians, Vec2};
use common::net::Connection;
use common::protocol;
use common::protocol::{Create, Remove, SelfInfo, Update};

use clients::Clients;


pub struct Events {
	buffer: ~Deque<Event>
}

#[deriving(Eq, Show)]
pub enum Event {
	Connect(Connection),
	Disconnect(uint),
	CreateEvent(uint),
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
					Connect(connection)  =>   on_connect(connection, clients, events),
					Disconnect(clientId) =>   on_disconnect(clientId, clients, events),
					CreateEvent(client_id) => on_create(client_id, clients, events),
					Update               =>   on_update(clients, events, frameTimeInMs as f64 / 1000.0)
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

			events.push(CreateEvent(client.id))
		},

		None => connection.close()
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

fn on_create(client_id: uint, clients: &mut Clients, events: &mut Events) {
	clients.each(|client| {
		let message = Create(Create {
			id  : client_id,
			kind: ~"ship"
		});

		match client.conn.send_message(message.to_str()) {
			Err(_) => events.push(Disconnect(client_id)),
			_      => ()
		}
	});
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

use collections::Deque;
use collections::RingBuf;

use common::physics::{Body, Radians, Vec2};
use common::net::Connection;
use common::protocol;
use common::protocol::{Create, Remove, SelfInfo, Update};

use clients::{Client, Clients};


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

	let new_client = Client::new(connection, ship);

	match clients.add(new_client) {
		Some((client_id, client)) => {
			let message = SelfInfo(SelfInfo {
				id: client_id
			});

			match client.conn.send_message(message.to_str()) {
				Err(_) => events.push(Disconnect(client_id)),
				_      => ()
			}

			events.push(CreateEvent(client_id))
		},

		None => connection.close()
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

fn on_create(created_id: uint, clients: &mut Clients, events: &mut Events) {
	clients.each(|client_id, client| {
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
	clients.mut_each(|client| {
		client.ship.position =
			client.ship.position + client.ship.velocity * dTimeInS;
	});

	clients.each(|client_a_id, clientA| {
		clients.each(|client_b_id, clientB| {
			let message = protocol::Update(Update {
				id  : client_b_id,
				body: clientB.ship
			});

			match clientA.conn.send_message(message.to_str()) {
				Err(_) => events.push(Disconnect(client_a_id)),
				_      => ()
			}
		})
	});
}

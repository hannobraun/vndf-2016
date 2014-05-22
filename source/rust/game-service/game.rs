use collections::HashMap;
use std::comm::{
	Disconnected,
	Empty
};

use common::physics::{
	Body,
	Radians,
	Vec2
};
use common::protocol::{
	Action,
	Perception,
	Ship
};

use events::{
	Action,
	Enter,
	GameEvent,
	Init,
	Leave,
	Message,
	NetworkEvent,
	Update
};
use network::ClientId;


pub struct Game {
	pub events: Sender<GameEvent>,

	incoming: Receiver<GameEvent>,
	network : Sender<NetworkEvent>,

	missiles: HashMap<ClientId, Body>,
	ships   : HashMap<ClientId, Body>,
	controls: HashMap<ClientId, Control>
}


impl Game {
	pub fn new(network: Sender<NetworkEvent>) -> Game {
		let (sender, receiver) = channel();

		Game {
			events  : sender,

			incoming: receiver,
			network : network,

			missiles: HashMap::new(),
			ships   : HashMap::new(),
			controls: HashMap::new()
		}
	}

	pub fn update(&mut self) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => {
					print!("Incoming event: {}\n", event);

					match event {
						Init =>
							(), // nothing do do, it just exists for the logging
						Enter(client_id) =>
							self.on_enter(client_id),
						Leave(client_id) =>
							self.on_leave(client_id),
						Update(frame_time_in_s) =>
							self.on_update(frame_time_in_s),
						Action(client_id, action) =>
							self.on_action(client_id, action)
					}
				},

				Err(error) => match error {
					Empty        => break,
					Disconnected => fail!("Unexpected error: {}", error)
				}
			}
		}
	}

	fn on_enter(&mut self, id: ClientId) {
		let velocity = Vec2(30.0, 10.0);
		self.ships.insert(id, Body {
			position: Vec2::zero(),
			velocity: velocity,
			attitude: Radians::from_vec(velocity)
		});

		self.controls.insert(id, Control {
			missile_index: 0
		});
	}

	fn on_leave(&mut self, id: ClientId) {
		self.ships.remove(&id);
		self.controls.remove(&id);
	}

	fn on_update(&mut self, dTimeInS: f64) {
		for (_, ship) in self.ships.mut_iter() {
			ship.velocity = ship.attitude.to_vec() * 30.0;
			ship.position = ship.position + ship.velocity * dTimeInS;
		}

		for (_, missile) in self.missiles.mut_iter() {
			missile.velocity = missile.attitude.to_vec() * 30.0;
			missile.position =
				missile.position + missile.velocity * dTimeInS;
		}

		let mut ships = Vec::new();
		for (&id, &ship) in self.ships.mut_iter() {
			ships.push(Ship {
				id  : id,
				body: ship
			});
		}

		let missiles: Vec<_> = self.missiles
			.iter()
			.map(|(&id, &body)|
				Ship {
					id  : id,
					body: body})
			.collect();

		for &id in self.controls.keys() {
			let update = Perception {
				self_id : id,
				ships   : ships.clone(),
				missiles: missiles.clone()
			};

			self.network.send(Message(vec!(id), update));
		}
	}

	fn on_action(&mut self, id: ClientId, action: Action) {
		match self.ships.find_mut(&id) {
			Some(ship) => {
				ship.attitude = action.attitude;

				let control = self.controls
					.find_mut(&id)
					.expect("execpted control");

				if action.missile > control.missile_index {
					let mut body = Body::default();
					body.position = ship.position;
					body.attitude = ship.attitude;

					self.missiles.insert(
						(id * 1000) as ClientId + action.missile as ClientId,
						body);
				}
				control.missile_index = action.missile;
			},

			None => ()
		}
	}
}


struct Control {
	missile_index: u64
}

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

use ecs::{
	Components,
	Player
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

	missiles: Components<Body>,
	ships   : Components<Body>,
	players : Components<Player>
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
			players : HashMap::new()
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

		self.players.insert(id, Player {
			missile_index: 0
		});
	}

	fn on_leave(&mut self, id: ClientId) {
		self.ships.remove(&id);
		self.players.remove(&id);
	}

	fn on_update(&mut self, delta_time_in_s: f64) {
		for (_, ship) in self.ships.mut_iter() {
			integrate(ship, delta_time_in_s);
		}

		for (_, missile) in self.missiles.mut_iter() {
			integrate(missile, delta_time_in_s);
		}

		for &id in self.players.keys() {
			let update = Perception {
				self_id : id,
				ships   : to_vec(&self.ships),
				missiles: to_vec(&self.missiles)
			};

			self.network.send(Message(vec!(id), update));
		}
	}

	fn on_action(&mut self, id: ClientId, action: Action) {
		match self.ships.find_mut(&id) {
			Some(ship) => {
				ship.attitude = action.attitude;

				let player = self.players
					.find_mut(&id)
					.expect("expected control");

				if action.missile > player.missile_index {
					let mut body = Body::default();
					body.position = ship.position;
					body.attitude = ship.attitude;

					self.missiles.insert(
						(id * 1000) as ClientId + action.missile as ClientId,
						body);
				}
				player.missile_index = action.missile;
			},

			None => ()
		}
	}
}

fn integrate(body: &mut Body, delta_time_in_s: f64) {
	body.velocity = body.attitude.to_vec() * 30.0;
	body.position = body.position + body.velocity * delta_time_in_s;
}

fn to_vec(bodies: &Components<Body>) -> Vec<Ship> {
	bodies
		.iter()
		.map(|(&id, &body)|
			Ship {
				id  : id,
				body: body
			})
		.collect()
}

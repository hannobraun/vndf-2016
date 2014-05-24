use collections::HashMap;
use std::comm::{
	Disconnected,
	Empty
};

use common::ecs::{
	Components,
	EntityId
};
use common::physics::Body;
use common::protocol::{
	Action,
	Perception
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
use game::entities::Entities;
use network::ClientId;


pub struct GameState {
	pub events: Sender<GameEvent>,

	incoming: Receiver<GameEvent>,
	network : Sender<NetworkEvent>,

	entities: Entities,
	missiles: Components<Body>
}

impl GameState {
	pub fn new(network: Sender<NetworkEvent>) -> GameState {
		let (sender, receiver) = channel();

		GameState {
			events  : sender,

			incoming: receiver,
			network : network,

			entities: Entities::new(),
			missiles: HashMap::new(),
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
		self.entities.create_ship(id);
	}

	fn on_leave(&mut self, id: ClientId) {
		self.entities.destroy_ship(id);
	}

	fn on_update(&mut self, delta_time_in_s: f64) {
		for (_, body) in self.entities.bodies.mut_iter() {
			integrate(body, delta_time_in_s);
		}

		for (_, missile) in self.missiles.mut_iter() {
			integrate(missile, delta_time_in_s);
		}

		for &id in self.entities.ships.keys() {
			let update = Perception {
				self_id: id,

				ships: self.entities.bodies
					.iter()
					.filter(|&(id, _)| self.entities.ships.contains_key(id))
					.map(|(&id, &body)| (id, body))
					.collect(),

				missiles: self.missiles.clone()
			};

			self.network.send(Message(vec!(id), update));
		}
	}

	fn on_action(&mut self, id: ClientId, action: Action) {
		let ship_body = match self.entities.bodies.find_mut(&id) {
			Some(body) => body,
			None       => return
		};

		ship_body.attitude = action.attitude;

		let ship = self.entities.ships
			.find_mut(&id)
			.expect("expected ship");

		if action.missile > ship.missile_index {
			let mut body = Body::default();
			body.position = ship_body.position;
			body.attitude = ship_body.attitude;

			self.missiles.insert(
				(id * 1000) as EntityId + action.missile as EntityId,
				body);
		}
		ship.missile_index = action.missile;
	}
}


fn integrate(body: &mut Body, delta_time_in_s: f64) {
	body.velocity = body.attitude.to_vec() * 30.0;
	body.position = body.position + body.velocity * delta_time_in_s;
}

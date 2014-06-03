use std::comm::{
	Disconnected,
	Empty
};

use common::physics::{
	Body,
	Radians,
	Vec2
};
use common::protocol;
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
	MissileLaunch,
	NetworkEvent,
	Update
};
use game::entities::Entities;
use network::ClientId;


pub struct State {
	pub events: Sender<GameEvent>,

	incoming: Receiver<GameEvent>,
	network : Sender<NetworkEvent>,

	entities: Entities
}

impl State {
	pub fn new(network: Sender<NetworkEvent>) -> State {
		let (sender, receiver) = channel();

		State {
			events  : sender,

			incoming: receiver,
			network : network,

			entities: Entities::new()
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
							self.on_action(client_id, action),
						MissileLaunch(position, attitude) =>
							self.on_missile_launch(position, attitude)
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

		for (&id, ship_control) in self.entities.ship_controls.iter() {
			let perception = Perception {
				self_id: id,
				updates: protocol::Entities {
					bodies  : self.entities.bodies.clone(),
					ships   : self.entities.ships.clone(),
					missiles: self.entities.missiles.clone()
				}
			};

			self.network.send(
				Message(vec!(ship_control.client_id), perception));
		}
	}

	fn on_action(&mut self, client_id: ClientId, action: Action) {
		let id = match self.entities.entity_id_from_client_id(client_id) {
			Some(id) => id,
			None     => return
		};

		let body = self.entities.bodies
			.find_mut(&id)
			.expect("expected body");
		let ship_control = self.entities.ship_controls
			.find_mut(&id)
			.expect("expected ship");

		body.attitude = action.attitude;

		if action.missile > ship_control.missile_index {
			self.events.send(
				MissileLaunch(
					body.position,
					body.attitude))
		}
		ship_control.missile_index = action.missile;
	}

	fn on_missile_launch(&mut self, position: Vec2, attitude: Radians) {
		self.entities.create_missile(position, attitude);
	}
}


fn integrate(body: &mut Body, delta_time_in_s: f64) {
	body.velocity = body.attitude.to_vec() * 30.0;
	body.position = body.position + body.velocity * delta_time_in_s;
}

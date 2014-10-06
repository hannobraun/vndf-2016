use std::comm::{
	Disconnected,
	Empty
};

use cgmath::{
	EuclideanVector,
	Quaternion,
	Rotation,
	Vector,
	Vector3,
};

use game::ecs::World as SharedWorld;
use game::ecs::Entity;
use initialstate::InitialState;
use net::ConnId;
use protocol::{
	Action,
	Perception
};

use super::ecs::{
	mod,
	integrate,
	World,
};
use super::events::{
	mod,
	GameEvent,
	NetworkEvent,
};


pub struct GameState {
	pub events: Sender<GameEvent>,

	incoming: Receiver<GameEvent>,
	network : Sender<NetworkEvent>,

	world: World
}

impl GameState {
	pub fn new(
		network      : Sender<NetworkEvent>,
		initial_state: &str,
	) -> GameState {
		let (sender, receiver) = channel();

		let mut world = World::new();
		let initial_state = InitialState::from_file(initial_state);
		initial_state.apply_to_world(&mut world);

		GameState {
			events  : sender,

			incoming: receiver,
			network : network,

			world: world,
		}
	}

	pub fn update(&mut self) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => {
					print!("Incoming event: {}\n", event);

					match event {
						events::Init =>
							(), // nothing do do, it just exists for the logging
						events::Enter(client_id) =>
							self.on_enter(client_id),
						events::Leave(client_id) =>
							self.on_leave(client_id),
						events::Update(frame_time_in_s) =>
							self.on_update(frame_time_in_s),
						events::Action(client_id, action) =>
							self.on_action(client_id, action),
						events::MissileLaunch(position, attitude) =>
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

	fn on_enter(&mut self, conn_id: ConnId) {
		let ship_id = self.world.create_ship();
		self.world.create_player(conn_id, ship_id);
	}

	fn on_leave(&mut self, conn_id: ConnId) {
		match ecs::entity_id_from_conn_id(&self.world, conn_id) {
			Some(player_id) => {
				match self.world.players[player_id].ship_id {
					Some(ship_id) =>
						self.world.destroy_entity(ship_id),
					None => (),
				}

				self.world.destroy_entity(player_id);
			},
			None => (),
		}
	}

	fn on_update(&mut self, delta_time_in_s: f64) {
		for (_, body) in self.world.bodies.iter_mut() {
			integrate(delta_time_in_s, body);
		}

		let mut entities_to_destroy = vec![];
		for (&body_id, body) in self.world.bodies.iter() {
			for (_, planet) in self.world.planets.iter() {
				if (body.position - planet.position).length() <= planet.radius {
					entities_to_destroy.push(body_id);
				}
			}
		}
		for &id in entities_to_destroy.iter() {
			self.world.destroy_entity(id);
		}

		// If you think the exponent should be -11, please consider that we're
		// using km instead of m, so the constant has to be adjusted for that.
		let gravitational_constant = 6.673e-17;
		for (_, body) in self.world.bodies.iter_mut() {
			for (_, planet) in self.world.planets.iter() {
				let body_to_planet = planet.position - body.position;
				let force =
					gravitational_constant
					* planet.mass
					/ body_to_planet.length2();

				body.force =
					body.force + body_to_planet.normalize().mul_s(force);
			}
		}

		for (_, body) in self.world.bodies.iter() {
			print!("{}\n", body);
			break;
		}

		let entities = {
			let world = SharedWorld::from_entities(
				self.world
					.export_entities()
					.iter()
					.map(|entity|
						Entity {
							id    : entity.id,
							body  : entity.body,
							visual: entity.visual,
							planet: entity.planet,
						}
					)
					.collect()
			);

			world.export_entities()
		};

		for (_, player) in self.world.players.iter_mut() {
			match player.ship_id {
				Some(ship_id) =>
					if !self.world.bodies.contains_key(&ship_id) {
						player.ship_id = None;
					},

				None => (),
			}

			let perception = Perception::new(
				|entity| entity.id,
				player.ship_id,
				player.last_snapshot.clone(),
				entities.clone()
			);

			self.network.send(
				events::Message(vec!(player.client_id), perception)
			);

			player.last_snapshot = entities.clone();
		}
	}

	fn on_action(&mut self, client_id: ConnId, action: Action) {
		let id = match ecs::entity_id_from_conn_id(&self.world, client_id) {
			Some(id) => id,
			None     => return
		};

		let player = self.world.players
			.find_mut(&id)
			.expect("expected player");
		let body = match player.ship_id {
			Some(id) =>
				self.world.bodies
					.find_mut(&id)
					.expect("expected body"),
			None => return,
		};

		body.attitude = action.attitude;

		let attitude_vec =
			body.attitude.rotate_vector(&Vector3::new(1.0, 0.0, 0.0));
		if action.thrust {
			body.force = body.force + attitude_vec.mul_s(10.0)
		}

		if action.missile > player.missile_index {
			self.events.send(
				events::MissileLaunch(
					body.position,
					body.attitude,
				)
			)
		}
		player.missile_index = action.missile;
	}

	fn on_missile_launch(&mut self, position: Vector3<f64>, attitude: Quaternion<f64>) {
		self.world.create_missile(position, attitude);
	}
}

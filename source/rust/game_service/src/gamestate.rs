use std::comm::{
	Disconnected,
	Empty
};

use cgmath::{
	Quaternion,
	Rotation,
	Vector,
	Vector3,
};

use game::ecs::World as SharedWorld;
use game::ecs::Entity;
use game::physics::Body;
use net::ConnId;
use protocol::{
	Action,
	Perception
};

use super::ecs::{
	mod,
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
	pub fn new(network: Sender<NetworkEvent>) -> GameState {
		let (sender, receiver) = channel();

		GameState {
			events  : sender,

			incoming: receiver,
			network : network,

			world: World::new()
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

	fn on_enter(&mut self, id: ConnId) {
		self.world.create_ship(id);
	}

	fn on_leave(&mut self, id: ConnId) {
		ecs::destroy_ship(&mut self.world, id);
	}

	fn on_update(&mut self, delta_time_in_s: f64) {
		for (_, body) in self.world.bodies.iter_mut() {
			integrate(body, delta_time_in_s);
		}

		let entities =
			SharedWorld::from_entities(
				self.world
					.export_entities()
					.iter()
					.map(|entity|
						Entity {
							id    : entity.id,
							body  : entity.body,
							visual: entity.visual,
							planet: None,
						})
					.collect())
			.export_entities();

		for (&id, player) in self.world.players.iter_mut() {
			let perception = Perception::new(
				|entity| entity.id,
				id,
				player.last_snapshot.clone(),
				entities.clone());

			self.network.send(
				events::Message(vec!(player.client_id), perception));

			player.last_snapshot = entities.clone();
		}
	}

	fn on_action(&mut self, client_id: ConnId, action: Action) {
		let id = match ecs::entity_id_from_client_id(&self.world, client_id) {
			Some(id) => id,
			None     => return
		};

		let body = self.world.bodies
			.find_mut(&id)
			.expect("expected body");
		let player = self.world.players
			.find_mut(&id)
			.expect("expected ship");

		body.attitude = action.attitude;

		let attitude_vec =
			body.attitude.rotate_vector(&Vector3::new(1.0, 0.0, 0.0));
		body.force = if action.thrust {
			attitude_vec.mul_s(10.0)
		}
		else {
			Vector3::zero()
		};

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


fn integrate(body: &mut Body, delta_time_in_s: f64) {
	body.velocity = body.velocity + body.force.mul_s(delta_time_in_s);
	body.position = body.position + body.velocity.mul_s(delta_time_in_s);
	body.force    = Vector3::zero();
}

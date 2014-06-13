use std::comm::{
	Disconnected,
	Empty
};

use common::ecs::{
	SharedWorld,
	SharedWorldEntity,
	World
};
use common::net::ConnId;
use common::physics::{
	Body,
	Radians,
	Vec2
};
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
use game::ecs;


pub struct State {
	pub events: Sender<GameEvent>,

	incoming: Receiver<GameEvent>,
	network : Sender<NetworkEvent>,

	world: World
}

impl State {
	pub fn new(network: Sender<NetworkEvent>) -> State {
		let (sender, receiver) = channel();

		State {
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

	fn on_enter(&mut self, id: ConnId) {
		self.world.create_ship(id);
	}

	fn on_leave(&mut self, id: ConnId) {
		ecs::destroy_ship(&mut self.world, id);
	}

	fn on_update(&mut self, delta_time_in_s: f64) {
		for (_, body) in self.world.bodies.mut_iter() {
			integrate(body, delta_time_in_s);
		}

		let entities =
			SharedWorld::from_entities(
				self.world
					.to_entities()
					.iter()
					.map(|entity|
						SharedWorldEntity {
							id    : entity.id,
							body  : entity.body,
							visual: entity.visual,
						})
					.collect())
			.to_entities();

		for (&id, player) in self.world.players.iter() {
			let perception = Perception::new(
				id,
				entities.clone(),
				entities.clone());

			self.network.send(
				Message(vec!(player.client_id), perception));
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

		if action.missile > player.missile_index {
			self.events.send(
				MissileLaunch(
					body.position,
					body.attitude))
		}
		player.missile_index = action.missile;
	}

	fn on_missile_launch(&mut self, position: Vec2, attitude: Radians) {
		self.world.create_missile(position, attitude);
	}
}


fn integrate(body: &mut Body, delta_time_in_s: f64) {
	body.velocity = body.attitude.to_vec() * 30.0;
	body.position = body.position + body.velocity * delta_time_in_s;
}

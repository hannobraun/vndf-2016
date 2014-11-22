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
use rustecs::{
	Control,
	EntityContainer,
	EntityId,
};

use game::ecs::Entity as SharedEntity;
use game::ecs::{
	ShowAsMissile,
	ShowAsShip,
};
use game::physics::Body;
use initialstate::InitialState;
use net::ConnId;
use protocol::{
	Action,
	Perception
};

use super::ecs::{
	mod,
	apply_gravity,
	integrate,
	kill_colliding_ships,
	Entities,
	Entity,
	Player,
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

	world  : Entities,
	control: Control<Entity>,
}

impl GameState {
	pub fn new(
		network      : Sender<NetworkEvent>,
		initial_state: &str,
	) -> GameState {
		let (sender, receiver) = channel();

		let mut world = Entities::new();
		let initial_state = InitialState::from_file(initial_state);
		initial_state.apply_to_world(&mut world);

		GameState {
			events  : sender,

			incoming: receiver,
			network : network,

			world  : world,
			control: Control::new(),
		}
	}

	pub fn update(&mut self) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => {
					print!("Incoming event: {}\n", event);

					match event {
						GameEvent::Init =>
							(), // nothing do do, it just exists for the logging
						GameEvent::Enter(client_id) =>
							self.on_enter(client_id),
						GameEvent::Leave(client_id) =>
							self.on_leave(client_id),
						GameEvent::Update(frame_time_in_s) =>
							self.on_update(frame_time_in_s),
						GameEvent::Action(client_id, action) =>
							self.on_action(client_id, action),
						GameEvent::MissileLaunch(position, attitude) =>
							self.on_missile_launch(position, attitude)
					}
				},

				Err(error) => match error {
					Empty        => break,
					Disconnected => panic!("Unexpected error: {}", error)
				}
			}
		}
	}

	fn on_enter(&mut self, conn_id: ConnId) {
		let ship_id = self.world.add(
			Entity::new()
				.with_body(Body::new()
					.with_position(Vector3::new(3000.0, 0.0, 0.0))
					.with_velocity(Vector3::new(-50.0, 0.0, 0.0))
				)
				.with_visual(ShowAsShip)
		);
		self.world.add(
			Entity::new()
				.with_player(Player::new(conn_id, ship_id))
		);
	}

	fn on_leave(&mut self, conn_id: ConnId) {
		match ecs::entity_id_from_conn_id(&self.world, conn_id) {
			Some(player_id) => {
				match self.world.players[player_id].ship_id {
					Some(ship_id) =>
						self.world.remove(ship_id),
					None => (),
				}

				self.world.remove(player_id);
			},
			None => (),
		}
	}

	fn on_update(&mut self, delta_time_in_s: f64) {
		integrate(delta_time_in_s, &mut self.world.bodies);
		kill_colliding_ships(
			&self.world.bodies,
			&self.world.planets,
			&mut self.control
		);
		apply_gravity(&mut self.world.bodies, &self.world.planets);

		self.control.apply(&mut self.world);

		let entities: Vec<(EntityId, SharedEntity)> = self.world
			.clone()
			.export()
			.iter()
			.map(|&(id, ref entity)|
				(
					id,
					SharedEntity {
						body  : entity.body,
						visual: entity.visual,
						planet: entity.planet,
					}
				)
			)
			.collect();

		for (_, player) in self.world.players.iter_mut() {
			match player.ship_id {
				Some(ship_id) =>
					if !self.world.bodies.contains_key(&ship_id) {
						player.ship_id = None;
					},

				None => (),
			}

			let perception = Perception::new(
				|&(id, _)| id,
				player.ship_id,
				player.last_snapshot.clone(),
				entities.clone()
			);

			self.network.send(
				NetworkEvent::Message(vec!(player.client_id), perception)
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
			.get_mut(&id)
			.expect("expected player");
		let body = match player.ship_id {
			Some(id) =>
				self.world.bodies
					.get_mut(&id)
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
				GameEvent::MissileLaunch(
					body.position,
					body.attitude,
				)
			)
		}
		player.missile_index = action.missile;
	}

	fn on_missile_launch(&mut self, position: Vector3<f64>, attitude: Quaternion<f64>) {
		self.world.add(
			Entity::new()
				.with_body(
					Body::new()
						.with_position(position)
						.with_attitude(attitude)
				)
				.with_visual(ShowAsMissile)
		);
	}
}

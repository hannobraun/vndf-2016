use collections::HashMap;

use common::ecs::components::{
	MissileVisual,
	ShipVisual,
	Visual
};
use common::ecs::infra::{
	Components,
	EntityId,
};
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data::Player;
use network::ClientId;


entity!(Missile<Body, Visual>, |args: (Vec2, Radians)| {
	let (position, attitude) = args;

	let body = Body {
		position: position,
		velocity: Vec2::zero(),
		attitude: attitude
	};

	(body, MissileVisual)
})

entity!(Ship<Body, Player, Visual>, |client_id: ClientId| {
	let body = Body {
		position: Vec2::zero(),
		velocity: Vec2::zero(),
		attitude: Radians(0.0)
	};

	let player = Player {
		client_id    : client_id,
		missile_index: 0
	};

	(body, player, ShipVisual)
})


pub struct Entities {
	next_id: EntityId,

	pub bodies : Components<Body>,
	pub players: Components<Player>,
	pub visuals: Components<Visual>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			next_id: 0,

			bodies : HashMap::new(),
			players: HashMap::new(),
			visuals: HashMap::new(),
		}
	}

	pub fn entity_id_from_client_id(&self, client_id: ClientId) -> Option<EntityId> {
		for (&id, player) in self.players.iter() {
			if player.client_id == client_id {
				return Some(id);
			}
		}

		None
	}

	pub fn create_ship(&mut self, client_id: ClientId) {
		let id = self.next_id();

		Ship::create(
			id,
			client_id,
			&mut self.bodies,
			&mut self.players,
			&mut self.visuals);
	}

	pub fn destroy_ship(&mut self, client_id: ClientId) {
		let id = match self.entity_id_from_client_id(client_id) {
			Some(id) => id,
			None     => return
		};

		self.destroy_entity(id);
	}

	pub fn create_missile(&mut self, position: Vec2, attitude: Radians) {
		let id = self.next_id();

		Missile::create(
			id,
			(position, attitude),
			&mut self.bodies,
			&mut self.visuals);
	}

	pub fn destroy_entity(&mut self, id: EntityId) {
		self.bodies.remove(&id);
		self.players.remove(&id);
		self.visuals.remove(&id);
	}

	fn next_id(&mut self) -> EntityId {
		let id = self.next_id;
		self.next_id += 1;
		id
	}
}

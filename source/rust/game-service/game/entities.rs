use collections::HashMap;

use common::ecs::infra::{
	Components,
	EntityId,
	EntityTemplate2
};
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data::{
	Missile,
	ShipControl
};
use network::ClientId;


pub struct Entities {
	missile_template: MissileTemplate,
	ship_template   : ShipTemplate,

	next_id: EntityId,

	pub bodies  : Components<Body>,
	pub missiles: Components<Missile>,
	pub ships   : Components<ShipControl>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			missile_template: MissileTemplate,
			ship_template   : ShipTemplate,

			next_id: 0,

			bodies  : HashMap::new(),
			missiles: HashMap::new(),
			ships   : HashMap::new()
		}
	}

	pub fn entity_id_from_client_id(&self, client_id: ClientId) -> Option<EntityId> {
		for (&id, ship) in self.ships.iter() {
			if ship.client_id == client_id {
				return Some(id);
			}
		}

		None
	}

	pub fn create_ship(&mut self, client_id: ClientId) {
		let id = self.next_id();

		self.ship_template.create(
			id,
			client_id,
			&mut self.bodies,
			&mut self.ships);
	}

	pub fn destroy_ship(&mut self, client_id: ClientId) {
		let id = match self.entity_id_from_client_id(client_id) {
			Some(id) => id,
			None     => return
		};

		self.ship_template.destroy(
			id,
			&mut self.bodies,
			&mut self.ships);
	}

	pub fn create_missile(&mut self, position: Vec2, attitude: Radians) {
		let id = self.next_id();

		self.missile_template.create(
			id,
			(position, attitude),
			&mut self.bodies,
			&mut self.missiles);
	}

	fn next_id(&mut self) -> EntityId {
		let id = self.next_id;
		self.next_id += 1;
		id
	}
}


struct ShipTemplate;

impl EntityTemplate2<ClientId, Body, ShipControl> for ShipTemplate {
	fn create_components(&self, client_id: ClientId) -> (Body, ShipControl) {
		let body = Body {
			position: Vec2::zero(),
			velocity: Vec2::zero(),
			attitude: Radians(0.0)
		};

		let ship = ShipControl {
			client_id    : client_id,
			missile_index: 0
		};

		(body, ship)
	}
}


struct MissileTemplate;

impl EntityTemplate2<(Vec2, Radians), Body, Missile> for MissileTemplate {
	fn create_components(&self, (position, attitude): (Vec2, Radians)) -> (Body, Missile) {
		let body = Body {
			position: position,
			velocity: Vec2::zero(),
			attitude: attitude
		};

		(body, Missile)
	}
}

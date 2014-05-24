use collections::HashMap;

use common::ecs::{
	Components,
	EntityId,
	EntityTemplate2
};
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data::Ship;
use network::ClientId;


pub struct Entities {
	ship_template: ShipTemplate,

	pub bodies: Components<Body>,
	pub ships : Components<Ship>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			ship_template: ShipTemplate,

			bodies: HashMap::new(),
			ships : HashMap::new()
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
		let id = client_id;

		self.ship_template.create(
			id,
			client_id,
			&mut self.bodies,
			&mut self.ships);
	}

	pub fn destroy_ship(&mut self, client_id: ClientId) {
		let mut ids = Vec::new();

		for (&id, ship) in self.ships.iter() {
			if ship.client_id == client_id {
				ids.push(id);
			}
		}

		for &id in ids.iter() {
			self.ship_template.destroy(id, &mut self.bodies, &mut self.ships);
		}
	}
}


struct ShipTemplate;

impl EntityTemplate2<ClientId, Body, Ship> for ShipTemplate {
	fn create_components(&self, client_id: ClientId) -> (Body, Ship) {
		let velocity = Vec2(30.0, 10.0);
		let body = Body {
			position: Vec2::zero(),
			velocity: velocity,
			attitude: Radians::from_vec(velocity)
		};

		let ship = Ship {
			client_id    : client_id,
			missile_index: 0
		};

		(body, ship)
	}
}

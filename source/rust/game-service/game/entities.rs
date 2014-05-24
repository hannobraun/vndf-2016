use common::ecs::Components;
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data::Ship;
use network::ClientId;


pub struct Entities {
	pub bodies: Components<Body>,
	pub ships : Components<Ship>
}

impl Entities {
	pub fn create_ship(&mut self, id: ClientId) {
		ShipTemplate::create(id, &mut self.bodies, &mut self.ships);
	}

	pub fn destroy_ship(&mut self, id: ClientId) {
		ShipTemplate::destroy(id, &mut self.bodies, &mut self.ships);
	}
}


struct ShipTemplate;

impl ShipTemplate {
	fn create(id: ClientId, bodies: &mut Components<Body>, ships: &mut Components<Ship>) {
		let velocity = Vec2(30.0, 10.0);
		bodies.insert(id, Body {
			position: Vec2::zero(),
			velocity: velocity,
			attitude: Radians::from_vec(velocity)
		});

		ships.insert(id, Ship {
			missile_index: 0
		});
	}

	fn destroy(id: ClientId, bodies: &mut Components<Body>, ships: &mut Components<Ship>) {
		bodies.remove(&id);
		ships.remove(&id);
	}
}

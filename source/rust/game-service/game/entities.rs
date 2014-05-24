use common::ecs::Components;
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data;
use network::ClientId;


pub struct Entities {
	pub bodies: Components<Body>,
	pub ships : Components<data::Ship>
}

impl Entities {
	pub fn create_ship(&mut self, id: ClientId) {
		let velocity = Vec2(30.0, 10.0);
		self.bodies.insert(id, Body {
			position: Vec2::zero(),
			velocity: velocity,
			attitude: Radians::from_vec(velocity)
		});

		self.ships.insert(id, data::Ship {
			missile_index: 0
		});
	}

	pub fn destroy_ship(&mut self, id: ClientId) {
		self.bodies.remove(&id);
		self.ships.remove(&id);
	}
}

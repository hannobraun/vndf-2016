use collections::HashMap;

use common::physics::{Body, Radians, Vec2};


pub type Id            = uint;
pub type Components<T> = HashMap<Id, T>;


pub struct Entities {
	pub self_id: Option<uint>,

	pub bodies  : Components<Body>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			self_id: None,

			bodies  : HashMap::new()
		}
	}

	pub fn create_ship(&mut self, id: Id) {
		let body = Body {
			position: Vec2::zero(),
			velocity: Vec2::zero(),
			attitude: Radians(0.0)
		};
		self.bodies.insert(id, body);
	}

	pub fn update_ship(&mut self, id: Id, body: Body) {
		self.bodies.insert(id, body);
	}

	pub fn remove_ship(&mut self, id: Id) {
		self.bodies.remove(&id);
	}
}

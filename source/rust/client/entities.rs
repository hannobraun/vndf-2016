use collections::HashMap;

use common::physics::{Body, Radians, Vec2};

use components::{Control};


pub type Id            = uint;
pub type Components<T> = HashMap<Id, T>;


pub struct Entities {
	pub self_id: Option<uint>,

	pub bodies  : Components<Body>,
	pub controls: Components<Control>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			self_id: None,

			controls: HashMap::new(),
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

		match self.self_id {
			Some(self_id) => if id == self_id {
				let control = Control {
					attitude: Radians(0.0),
					send    : false
				};
				self.controls.insert(id, control);
			},
			None => ()
		}
	}

	pub fn update_ship(&mut self, id: Id, body: Body) {
		self.bodies.insert(id, body);
	}

	pub fn remove_ship(&mut self, id: Id) {
		self.bodies.remove(&id);
	}
}

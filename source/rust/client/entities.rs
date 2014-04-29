use collections::HashMap;

use common::physics::{Body, Radians, Vec2};

use components::{Control, Visual};


pub type Id            = uint;
pub type Components<T> = HashMap<Id, T>;


pub struct Entities {
	pub self_id: Option<uint>,

	pub bodies  : Components<Body>,
	pub controls: Components<Control>,
	pub visuals : Components<Visual>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			self_id: None,

			controls: HashMap::new(),
			bodies  : HashMap::new(),
			visuals : HashMap::new() }
	}

	pub fn create_ship(&mut self, id: Id) {
		let body = Body {
			position: Vec2 { x: 0.0, y: 0.0 },
			velocity: Vec2 { x: 0.0, y: 0.0 },
			attitude: Radians(0.0)
		};
		self.bodies.insert(id, body);
		self.visuals.insert(id, Visual { texture: ~"images/spaceship.png" });

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
		self.visuals.remove(&id);
	}

	pub fn update_asteroid(&mut self, id: Id, position: Vec2) {
		let body = Body {
			position: position,
			velocity: Vec2 { x: 0.0, y: 0.0 },
			attitude: Radians(0.0)
		};
		self.bodies.insert(id, body);
		self.visuals.insert(id, Visual { texture: ~"char:A" });
	}
}

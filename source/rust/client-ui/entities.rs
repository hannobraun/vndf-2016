use collections::HashMap;

use common::physics::{Body, Vec2};

use components::Visual;


pub type Id            = uint;
pub type Components<T> = HashMap<Id, T>;


pub struct Entities {
	positions: Components<Body>,
	visuals  : Components<Visual>
}

impl Entities {
	pub fn new() -> ~Entities {
		~Entities {
			positions: HashMap::new(),
			visuals  : HashMap::new() }
	}

	pub fn update_ship(&mut self, id: Id, position: Vec2) {
		let body = Body {
			position: position,
			velocity: Vec2 { x: 0.0, y: 0.0 },
			attitude: 0.0
		};
		self.positions.insert(id, body);
		self.visuals.insert(id, Visual { texture: ~"images/spaceship.png" });
	}

	pub fn remove_ship(&mut self, id: Id) {
		self.positions.remove(&id);
		self.visuals.remove(&id);
	}

	pub fn update_asteroid(&mut self, id: Id, position: Vec2) {
		let body = Body {
			position: position,
			velocity: Vec2 { x: 0.0, y: 0.0 },
			attitude: 0.0
		};
		self.positions.insert(id, body);
		self.visuals.insert(id, Visual { texture: ~"char:A" });
	}
}

use std::hashmap::HashMap;

use common::vec::Vec2;

use visual::Visual;


pub type Id = int;


pub struct Entities {
	positions: HashMap<Id, Vec2>,
	visuals  : HashMap<Id, Visual>
}

impl Entities {
	pub fn new() -> ~Entities {
		~Entities {
			positions: HashMap::new(),
			visuals  : HashMap::new() }
	}

	pub fn update(&mut self, id: Id, x: f64, y: f64) {
		self.positions.insert(id, Vec2 { x: x, y: y });
		self.visuals.insert(id, Visual { image: ~"images/spaceship.png" });
	}

	pub fn remove(&mut self, id: Id) {
		self.positions.remove(&id);
		self.visuals.remove(&id);
	}
}

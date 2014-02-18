use std::hashmap::HashMap;

use common::vec::Vec2;


pub type Id = int;


pub struct Entities {
	positions: HashMap<Id, Vec2>
}

impl Entities {
	pub fn new() -> ~Entities {
		~Entities {
			positions: HashMap::new() }
	}

	pub fn update(&mut self, id: Id, x: f64, y: f64) {
		self.positions.insert(id, Vec2 { x: x, y: y });
	}
}

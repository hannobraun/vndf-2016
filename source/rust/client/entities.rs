use std::hashmap::HashMap;

use common::vec::Vec2;

use visual::Visual;


pub type Id            = int;
pub type Components<T> = HashMap<Id, T>;


pub struct Entities {
	positions: Components<Vec2>,
	visuals  : Components<Visual>
}

impl Entities {
	pub fn new() -> ~Entities {
		~Entities {
			positions: HashMap::new(),
			visuals  : HashMap::new() }
	}

	pub fn update_ship(&mut self, id: Id, x: f64, y: f64) {
		self.positions.insert(id, Vec2 { x: x, y: y });
		self.visuals.insert(id, Visual { texture: ~"images/spaceship.png" });
	}

	pub fn remove_ship(&mut self, id: Id) {
		self.positions.remove(&id);
		self.visuals.remove(&id);
	}
}

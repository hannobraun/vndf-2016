use collections::HashMap;

use common::vec::Vec3;

use visual::Visual;


pub type Id            = int;
pub type Components<T> = HashMap<Id, T>;


pub struct Entities {
	positions: Components<Vec3>,
	visuals  : Components<Visual>
}

impl Entities {
	pub fn new() -> ~Entities {
		~Entities {
			positions: HashMap::new(),
			visuals  : HashMap::new() }
	}

	pub fn update_ship(&mut self, id: Id, x: f64, y: f64) {
		self.positions.insert(id, Vec3 { x: x, y: y, z: 0.0 });
		self.visuals.insert(id, Visual { texture: ~"images/spaceship.png" });
	}

	pub fn remove_ship(&mut self, id: Id) {
		self.positions.remove(&id);
		self.visuals.remove(&id);
	}

	pub fn update_asteroid(&mut self, id: Id, x: f64, y: f64) {
		self.positions.insert(id, Vec3 { x: x, y: y, z: 0.0 });
		self.visuals.insert(id, Visual { texture: ~"A" });
	}
}

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
}

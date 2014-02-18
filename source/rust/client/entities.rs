use std::hashmap::HashMap;

use common::vec::Vec2;


pub struct Entities {
	positions: HashMap<int, Vec2>
}

impl Entities {
	pub fn new() -> ~Entities {
		~Entities {
			positions: HashMap::new() }
	}
}

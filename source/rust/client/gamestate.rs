use collections::HashMap;
use time;

use common::physics::Body;


pub struct GameState {
	pub self_id: Option<uint>,

	pub previous_time : u64,
	pub current_time  : u64,
	pub previous_ships: HashMap<uint, Body>,
	pub current_ships : HashMap<uint, Body>,

	pub missiles: HashMap<uint, Body>
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			self_id: None,

			previous_time : time::precise_time_ns(),
			current_time  : time::precise_time_ns(),
			previous_ships: HashMap::new(),
			current_ships : HashMap::new(),

			missiles: HashMap::new()
		}
	}
}

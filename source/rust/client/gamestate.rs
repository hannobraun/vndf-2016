use collections::HashMap;

use common::physics::Body;


pub struct GameState {
	pub self_id: Option<uint>,

	pub previous_time : u64,
	pub current_time  : u64,
	pub previous_ships: HashMap<uint, Body>,
	pub current_ships : HashMap<uint, Body>,

	pub missiles: HashMap<uint, Body>
}

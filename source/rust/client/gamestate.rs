use collections::HashMap;
use time;

use common::physics::Body;

use network::Network;


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

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			self.previous_time = self.current_time;
			self.current_time  = time::precise_time_ns();

			self.previous_ships.clear();
			for (&id, &ship) in self.current_ships.iter() {
				self.previous_ships.insert(id, ship);
			}

			self.current_ships.clear();
			for ship in perception.ships.iter() {
				self.current_ships.insert(ship.id, ship.body);
			}

			for missile in perception.missiles.iter() {
				self.missiles.insert(missile.id, missile.body);
			}
		});
	}
}

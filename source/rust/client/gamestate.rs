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

	pub fn receive_updates(network: &mut Network, game_state: &mut GameState) {
		network.receive(|perception| {
			game_state.self_id = Some(perception.self_id);

			game_state.previous_time = game_state.current_time;
			game_state.current_time  = time::precise_time_ns();

			game_state.previous_ships.clear();
			for (&id, &ship) in game_state.current_ships.iter() {
				game_state.previous_ships.insert(id, ship);
			}

			game_state.current_ships.clear();
			for ship in perception.ships.iter() {
				game_state.current_ships.insert(ship.id, ship.body);
			}

			for missile in perception.missiles.iter() {
				game_state.missiles.insert(missile.id, missile.body);
			}
		});
	}
}

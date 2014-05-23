use collections::HashMap;
use time;

use common::physics::{
	Body,
	Vec2
};

use network::Network;


pub struct GameState {
	self_id: Option<uint>,
	ships  : InterpolatedBodies,

	pub missiles: HashMap<uint, Body>
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			self_id: None,

			ships: InterpolatedBodies {
				previous_time : time::precise_time_ns(),
				current_time  : time::precise_time_ns(),
				previous_ships: HashMap::new(),
				current_ships : HashMap::new()
			},

			missiles: HashMap::new()
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			self.ships.previous_time = self.ships.current_time;
			self.ships.current_time  = time::precise_time_ns();

			self.ships.previous_ships.clear();
			for (&id, &ship) in self.ships.current_ships.iter() {
				self.ships.previous_ships.insert(id, ship);
			}

			self.ships.current_ships.clear();
			for ship in perception.ships.iter() {
				self.ships.current_ships.insert(ship.id, ship.body);
			}

			for missile in perception.missiles.iter() {
				self.missiles.insert(missile.id, missile.body);
			}
		});
	}

	pub fn interpolate_ships_and_camera(&mut self, camera: &mut Vec2) -> Vec<Body> {
		let i = {
			let diff = (self.ships.current_time - self.ships.previous_time) as f64;
			if diff <= 0.0 {
				0.0
			}
			else {
				(time::precise_time_ns() - self.ships.current_time) as f64 / diff
			}
		};

		let mut ships = Vec::new();
		for (&ship_id, &current) in self.ships.current_ships.iter() {
			match self.ships.previous_ships.find(&ship_id) {
				Some(&previous) => {
					let mut body = current.clone();
					body.position = previous.position + (current.position - previous.position) * i;
					ships.push(body);

					match self.self_id {
						Some(id) => if id == ship_id {
							*camera = body.position;
						},

						None => ()
					}
				},

				None => ()
			}
		}

		ships
	}
}


struct InterpolatedBodies {
	previous_time : u64,
	current_time  : u64,
	previous_ships: HashMap<uint, Body>,
	current_ships : HashMap<uint, Body>
}

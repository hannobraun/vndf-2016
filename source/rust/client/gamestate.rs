use collections::HashMap;
use time;

use common::physics::{
	Body,
	Vec2
};

use network::Network;


pub struct GameState {
	self_id : Option<uint>,
	ships   : InterpolatedBodies,
	missiles: InterpolatedBodies
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			self_id : None,
			ships   : InterpolatedBodies::new(),
			missiles: InterpolatedBodies::new()
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			self.ships.receive(&perception.ships);
			self.missiles.receive(&perception.missiles);
		});
	}

	pub fn interpolate(&mut self) -> (Vec<Body>, Vec<Body>) {
		(
			self.ships.interpolate(),
			self.missiles.interpolate())
	}

	pub fn update_camera(&self, camera: &mut Vec2) {
		let self_id = match self.self_id {
			Some(id) => id,
			None     => return
		};

		for (&id, ship) in self.ships.bodies.iter() {
			if id == self_id && ship.current.is_some() {
				*camera = ship.current.unwrap().position;
			}
		}
	}
}


struct InterpolatedBodies {
	previous_time: u64,
	current_time : u64,

	bodies: HashMap<uint, Interpolated>
}

impl InterpolatedBodies {
	fn new() -> InterpolatedBodies {
		InterpolatedBodies {
			previous_time: time::precise_time_ns(),
			current_time : time::precise_time_ns(),

			bodies: HashMap::new()
		}
	}

	fn receive(&mut self, bodies: &HashMap<uint, Body>) {
		self.previous_time = self.current_time;
		self.current_time  = time::precise_time_ns();

		for (_, body) in self.bodies.mut_iter() {
			body.previous = body.current;
			body.current  = None;
		}

		for (&id, &body) in bodies.iter() {
			let interpolated = self.bodies.find_or_insert(id, Interpolated {
				previous: None,
				current : None
			});

			interpolated.current = Some(body);
		}
	}

	fn interpolate(&self) -> Vec<Body> {
		let i = {
			let diff = (self.current_time - self.previous_time) as f64;
			if diff <= 0.0 {
				0.0
			}
			else {
				(time::precise_time_ns() - self.current_time) as f64 / diff
			}
		};

		let mut bodies = Vec::new();
		for (_, &interpolated) in self.bodies.iter() {
			let previous = match interpolated.previous {
				Some(body) => body,
				None       => continue
			};
			let current = match interpolated.current {
				Some(body) => body,
				None       => continue
			};

			let mut body = current.clone();
			body.position =
				previous.position + (current.position - previous.position) * i;
			bodies.push(body);
		}

		bodies
	}
}


struct Interpolated {
	previous: Option<Body>,
	current : Option<Body>
}

use collections::HashMap;
use time;

use common::ecs::{
	Components,
	EntityId
};
use common::physics::{
	Body,
	Vec2
};

use network::Network;


pub struct GameState {
	self_id : Option<EntityId>,
	ships   : Components<Interpolated>,
	missiles: Components<Interpolated>
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			self_id : None,
			ships   : HashMap::new(),
			missiles: HashMap::new()
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			receive(&mut self.ships, &perception.ships);
			receive(&mut self.missiles, &perception.missiles);
		});
	}

	pub fn interpolate(&mut self) -> (Vec<Body>, Vec<Body>) {
		(
			interpolate(&mut self.ships),
			interpolate(&mut self.missiles))
	}

	pub fn update_camera(&self, camera: &mut Vec2) {
		let self_id = match self.self_id {
			Some(id) => id,
			None     => return
		};

		for (&id, ship) in self.ships.iter() {
			if id == self_id && ship.current.is_some() {
				*camera = ship.current.unwrap().position;
			}
		}
	}
}


struct InterpolatedBodies {
	interpolateds: HashMap<uint, Interpolated>
}

impl InterpolatedBodies {
	fn new() -> InterpolatedBodies {
		InterpolatedBodies {
			interpolateds: HashMap::new()
		}
	}
}


fn receive(interpolateds: &mut Components<Interpolated>, bodies: &HashMap<uint, Body>) {
	let current_time = time::precise_time_ns();

	for (_, body) in interpolateds.mut_iter() {
		body.previous_time = body.current_time;

		body.previous = body.current;
		body.current  = None;
	}

	for (&id, &body) in bodies.iter() {
		let interpolated = interpolateds.find_or_insert(
			id,
			Interpolated {
				previous_time: current_time,
				current_time : current_time,

				previous: None,
				current : None
			});

		interpolated.current      = Some(body);
		interpolated.current_time = current_time;
	}
}


fn interpolate(interpolateds: &mut Components<Interpolated>) -> Vec<Body> {
	let mut bodies = Vec::new();
	for (_, &interpolated) in interpolateds.iter() {
		let previous = match interpolated.previous {
			Some(body) => body,
			None       => continue
		};
		let current = match interpolated.current {
			Some(body) => body,
			None       => continue
		};

		let i = {
			let diff =
				(interpolated.current_time - interpolated.previous_time) as f64;
			if diff <= 0.0 {
				0.0
			}
			else {
				(time::precise_time_ns() - interpolated.current_time) as f64 / diff
			}
		};

		let mut body = current.clone();
		body.position =
			previous.position + (current.position - previous.position) * i;
		bodies.push(body);
	}

	bodies
}


struct Interpolated {
	previous_time: u64,
	current_time : u64,

	previous: Option<Body>,
	current : Option<Body>
}

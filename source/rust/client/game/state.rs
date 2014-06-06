use time;

use rustecs::{
	components,
	Components,
	EntityId
};

use common::ecs::components::{
	MissileVisual,
	ShipVisual
};
use common::physics::{
	Body,
	Vec2
};

use game::data::{
	Interpolated,
	Missile
};
use network::Network;


pub struct State {
	self_id: Option<EntityId>,

	interpolateds: Components<Interpolated>,
	missiles     : Components<Missile>,
	ships        : Components<Interpolated>
}

impl State {
	pub fn new() -> State {
		State {
			self_id: None,

			interpolateds: components(),
			missiles     : components(),
			ships        : components(),
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			prepare_receive(&mut self.ships);
			prepare_receive(&mut self.interpolateds);

			receive(
				&mut self.ships,
				&perception.updated.bodies
					.iter()
					.filter(|&(id, _)|
						perception.updated.visuals.get(id) == &ShipVisual)
					.map(|(&id, &body)|
						(id, body))
					.collect());
			receive(
				&mut self.interpolateds,
				&perception.updated.bodies
					.iter()
					.filter(|&(id, _)|
						perception.updated.visuals.get(id) == &MissileVisual)
					.map(|(&id, &body)|
						(id, body))
					.collect());
		});
	}

	pub fn interpolate(&mut self) -> (Vec<Body>, Vec<Body>) {
		(
			interpolate(&mut self.ships),
			interpolate(&mut self.interpolateds))
	}

	pub fn update_camera(&self, camera: &mut Vec2) {
		let self_id = match self.self_id {
			Some(id) => id,
			None     => return
		};

		for (&id, interpolated) in self.ships.iter() {
			if id == self_id && interpolated.current.is_some() {
				*camera = interpolated.current.unwrap().position;
			}
		}
	}
}


fn prepare_receive(interpolateds: &mut Components<Interpolated>) {
	for (_, body) in interpolateds.mut_iter() {
		body.previous_time = body.current_time;

		body.previous = body.current;
		body.current  = None;
	}
}

fn receive(interpolateds: &mut Components<Interpolated>, bodies: &Components<Body>) {
	let current_time = time::precise_time_ns();

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

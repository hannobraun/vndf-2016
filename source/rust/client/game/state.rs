use std::collections::HashMap;
use time;

use rustecs::{
	components,
	Components,
	EntityId
};

use common::ecs::{
	Interpolated,
	ShowAsMissile,
	ShowAsShip,
	Visual,
};
use common::physics::{
	Body,
	Vec2
};

use network::Network;


pub struct State {
	self_id: Option<EntityId>,

	bodies       : Components<Body>,
	interpolateds: Components<Interpolated>,
	visuals      : Components<Visual>,
}

impl State {
	pub fn new() -> State {
		State {
			self_id: None,

			bodies       : components(),
			interpolateds: components(),
			visuals      : components(),
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			for (_, interpolated) in self.interpolateds.mut_iter() {
				interpolated.previous_time = interpolated.current_time;

				interpolated.previous = interpolated.current;
				interpolated.current  = None;
			}

			let current_time = time::precise_time_ns();
			for entity in perception.updated.iter() {
				let interpolated = self.interpolateds.find_or_insert(
					entity.id,
					Interpolated {
						previous_time: current_time,
						current_time : current_time,

						previous: None,
						current : None
					}
				);

				interpolated.current      = entity.body;
				interpolated.current_time = current_time;

				self.visuals.insert(entity.id, entity.visual.unwrap());
			}
		});
	}

	pub fn interpolate(&mut self) -> (Vec<Body>, Vec<Body>) {
		let ships = self.interpolateds
			.iter()
			.filter(|&(id, _)|
				self.visuals.get(id) == &ShowAsShip)
			.collect();
		let missiles = self.interpolateds
			.iter()
			.filter(|&(id, _)|
				self.visuals.get(id) == &ShowAsMissile)
			.collect();

		(
			interpolate(&ships   , &mut self.bodies),
			interpolate(&missiles, &mut self.bodies))
	}

	pub fn update_camera(&self, camera: &mut Vec2) {
		let self_id = match self.self_id {
			Some(id) => id,
			None     => return
		};

		for (&id, interpolated) in self.interpolateds.iter() {
			if id == self_id && interpolated.current.is_some() {
				*camera = interpolated.current.unwrap().position;
			}
		}
	}
}


fn interpolate(interpolateds: &HashMap<&EntityId, &Interpolated>, c_bodies: &mut Components<Body>) -> Vec<Body> {
	let mut bodies = Vec::new();
	for (&&id, &interpolated) in interpolateds.iter() {
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

		c_bodies.insert(id, body);
	}

	bodies
}

use std::collections::HashMap;
use time;

use rustecs::{
	Components,
	EntityId
};

use common::ecs::{
	ClientWorld,
	Interpolated,
	ShowAsMissile,
	ShowAsShip,
};
use common::physics::{
	Body,
	Vec2
};

use network::Network;


pub struct State {
	self_id: Option<EntityId>,
	world  : ClientWorld,
}

impl State {
	pub fn new() -> State {
		State {
			self_id: None,
			world  : ClientWorld::new(),
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			for (_, interpolated) in self.world.interpolateds.mut_iter() {
				interpolated.previous_time = interpolated.current_time;

				interpolated.previous = interpolated.current;
				interpolated.current  = None;
			}

			let current_time = time::precise_time_ns();
			for entity in perception.updated.iter() {
				let interpolated = self.world.interpolateds.find_or_insert(
					entity.id,
					Interpolated::new(current_time, entity.body)
				);

				interpolated.current      = entity.body;
				interpolated.current_time = current_time;

				self.world.visuals.insert(entity.id, entity.visual.unwrap());
			}
		});
	}

	pub fn interpolate(&mut self) -> (Vec<Body>, Vec<Body>) {
		let ships = self.world.interpolateds
			.iter()
			.filter(|&(id, _)|
				self.world.visuals.get(id) == &ShowAsShip)
			.collect();
		let missiles = self.world.interpolateds
			.iter()
			.filter(|&(id, _)|
				self.world.visuals.get(id) == &ShowAsMissile)
			.collect();

		(
			interpolate(&ships   , &mut self.world.bodies),
			interpolate(&missiles, &mut self.world.bodies))
	}

	pub fn update_camera(&self, camera: &mut Vec2) {
		let self_id = match self.self_id {
			Some(id) => id,
			None     => return
		};

		for (&id, body) in self.world.bodies.iter() {
			if id == self_id {
				*camera = body.position;
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

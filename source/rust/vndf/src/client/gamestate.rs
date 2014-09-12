use time;

use client::network::Network;
use client::receiver::receive;
use game::ecs::ClientWorld;
use physics::Vec2;
use rustecs::EntityId;


pub struct State {
	pub self_id: Option<EntityId>,
	pub world  : ClientWorld,
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

			receive(&mut self.world, perception);
		});
	}

	pub fn interpolate(&mut self) {
		for (&id, interpolated) in self.world.interpolateds.iter() {
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

			self.world.bodies.insert(id, body);
		}
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

use time;

use game::ecs::ClientWorld;
use platform::Camera;
use rustecs::EntityId;

use super::network::Network;
use super::receiver::receive;


pub struct GameState {
	pub self_id: Option<EntityId>,
	pub world  : ClientWorld,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
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

	pub fn update_camera(&self, camera: &mut Camera) {
		let self_id = match self.self_id {
			Some(id) => id,
			None     => return
		};

		for (&id, body) in self.world.bodies.iter() {
			if id == self_id {
				camera.center = body.position.to_vector2_f64();
			}
		}
	}
}

use time;

use cgmath::{
	Vector,
	Vector3,
};

use platform::Camera;
use rustecs::EntityId;

use super::ecs::World;
use super::network::Network;
use super::receiver::receive;


pub struct GameState {
	pub self_id: Option<EntityId>,
	pub world  : World,
}

impl GameState {
	pub fn new() -> GameState {
		let mut world = World::new();
		world.create_planet(
			Vector3::zero(),
			2576.0,
			Vector3::new(0.8, 0.68, 0.27),
		);
		world.create_planet(
			Vector3::new(0.0, 5000.0, 0.0),
			480.0,
			Vector3::new(0.5, 0.7, 0.7),
		);

		GameState {
			self_id: None,
			world  : world,
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			for (_, interpolated) in self.world.interpolateds.iter_mut() {
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
				previous.position + (current.position - previous.position).mul_s(i);

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
				camera.center = body.position;
			}
		}
	}
}

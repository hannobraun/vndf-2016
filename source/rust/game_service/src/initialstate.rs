use serialize::json;
use std::io::File;

use ecs::World;
use game::ecs::Planet;


#[deriving(Decodable)]
pub struct InitialState {
	planets: Vec<Planet>,
}

impl InitialState {
	pub fn from_file(path: &str) -> InitialState {
		let initial_state_as_json =
			File::open(&Path::new(path))
				.read_to_string()
				.unwrap();

		json::decode(initial_state_as_json.as_slice()).unwrap()
	}

	pub fn apply_to_world(&self, world: &mut World) {
		for planet in self.planets.iter() {
			world.create_planet(
				planet.position,
				planet.radius,
				planet.color,
			);
		}
	}
}

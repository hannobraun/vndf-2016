use serialize::json;
use std::io::File;

use ecs::World;
use game::ecs::Planet;


#[deriving(Decodable)]
pub struct InitialState {
	planets: Vec<Planet>,
}

impl InitialState {
	pub fn from_file(world: &mut World, path: &str) {
		let initial_state_as_json =
			File::open(&Path::new(path))
				.read_to_string()
				.unwrap();

		let initial_state: InitialState =
			json::decode(initial_state_as_json.as_slice()).unwrap();

		for planet in initial_state.planets.iter()  {
			world.create_planet(
				planet.position,
				planet.radius,
				planet.color,
			);
		}
	}
}

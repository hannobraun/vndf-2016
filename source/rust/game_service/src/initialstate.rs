use serialize::json;
use std::io::File;

use ecs::World;
use game::ecs::Planet;


pub fn load(world: &mut World) {
	let initial_state_as_json =
		File::open(&Path::new("initial-state.json"))
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


#[deriving(Decodable)]
struct InitialState {
	planets: Vec<Planet>,
}

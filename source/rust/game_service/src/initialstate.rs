use serialize::json;

use ecs::World;
use game::ecs::Planet;


pub fn load(world: &mut World) {
	let initial_state_as_json = "
		{
			\"planets\": [
				{
					\"position\": {
						\"x\": 0.0,
						\"y\": 0.0,
						\"z\": 0.0
					},
					\"radius\": 2576.0,
					\"color\": {
						\"x\": 0.8,
						\"y\": 0.68,
						\"z\": 0.27
					}
				},
				{
					\"position\": {
						\"x\": 0.0,
						\"y\": 5000.0,
						\"z\": 0.0
					},
					\"radius\": 480.0,
					\"color\": {
						\"x\": 0.5,
						\"y\": 0.7,
						\"z\": 0.7
					}
				}
			]
		}
	";

	let initial_state: InitialState =
		json::decode(initial_state_as_json).unwrap();

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

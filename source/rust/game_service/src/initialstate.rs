use cgmath::Vector3;

use ecs::World;
use game::ecs::Planet;


pub fn load(world: &mut World) {
	let initial_state = InitialState {
		planets: vec![
			Planet {
				position: Vector3::zero(),
				radius  : 2576.0,
				color   : Vector3::new(0.8, 0.68, 0.27),
			},
			Planet {
				position: Vector3::new(0.0, 5000.0, 0.0),
				radius  : 480.0,
				color   : Vector3::new(0.5, 0.7, 0.7),
			},
		]
	};

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

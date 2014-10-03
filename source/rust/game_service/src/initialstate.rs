use cgmath::Vector3;

use ecs::World;


pub fn load(world: &mut World) {
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
}

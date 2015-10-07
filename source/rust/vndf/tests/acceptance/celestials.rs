use nalgebra::Vec2;

use vndf::server::game::data::Spawner;
use vndf::server::game::initial_state::{
	Celestial,
	InitialState,
};
use vndf::testing::rc;


#[test]
fn a_ship_colliding_with_a_celestial_body_should_be_removed() {
	let initial_state = InitialState::new()
		.with_celestial(Celestial {
			position: Vec2::new(0.0, 0.0),
			size    : 10.0,
		})
		.with_spawner(Spawner {
			position: Vec2::new(-15.0, 0.0),
			velocity: Vec2::new(1.0, 0.0),
		});

	let     server = rc::Server::start(initial_state);
	let mut client = rc::Client::start(server.port());

	client.wait_until(|frame|
		frame.ships.len() == 1
	);

	// Collision occurs in the meantime

	client.wait_until(|frame|
		frame.ships.len() == 0
	);
}

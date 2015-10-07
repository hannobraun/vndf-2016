use nalgebra::Vec2;

use vndf::client::interface::Frame;
use vndf::server::game::data::Spawner;
use vndf::server::game::initial_state::{
	Celestial,
	InitialState,
};
use vndf::shared::game::Body;
use vndf::testing::rc;


#[test]
fn celestial_bodies_should_be_visibile_to_the_player() {
	let initial_state = InitialState::new()
		.with_celestial(Celestial {
			position: Vec2::new(-100.0, 0.0),
			size    : 10.0,
		});

	let     server = rc::Server::start(initial_state);
	let mut client = rc::Client::start(server.port());

	client.wait_until(|frame|
		frame.planets.len() == 1
	);
}

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

#[test]
fn a_celestial_body_should_exert_gravitational_influence_on_ships() {
	let initial_state = InitialState::new()
		.with_celestial(Celestial {
			position: Vec2::new(0.0, 0.0),
			size    : 10.0,
		})
		.with_spawner(Spawner {
			position: Vec2::new(-15.0, 0.0),
			velocity: Vec2::new(0.0, 0.0),
		});

	let     server = rc::Server::start(initial_state);
	let mut client = rc::Client::start(server.port());

	let frame = client.wait_until(|frame|
		frame.ships.len() == 1 && first_ship(&frame).velocity.x > 0.0
	);

	let ship = first_ship(&frame);
	assert!(ship.velocity.x > 0.0);
	assert_eq!(ship.velocity.y, 0.0);
}


fn first_ship(frame: &Frame) -> Body {
	*frame.ships.iter().next().unwrap().1
}

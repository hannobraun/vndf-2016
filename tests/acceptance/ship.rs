use common::io::Input;
use common::physics::Radians;
use common::physics::util;

use common::testing::{
	Client,
	GameService
};


#[test]
fn it_should_follow_its_velocity_vector() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	while frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	while frame_1.ships.get(0).position == frame_2.ships.get(0).position {
		frame_2 = client.frame();
	}

	let pos_1    = frame_1.ships.get(0).position;
	let pos_2    = frame_2.ships.get(0).position;
	let velocity = frame_1.ships.get(0).velocity;

	assert!(util::is_on_line(
		(pos_1, pos_1 + velocity),
		pos_2,
		4));
}

#[test]
fn it_should_change_direction_according_to_input() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame = client.frame();

	while frame.ships.len() == 0 {
		frame = client.frame();
	}

	let velocity     = frame.ships.get(0).velocity;
	let new_velocity = velocity * -1.0;
	let new_attitude = Radians::from_vec(new_velocity);

	client.input(Input {
		exit    : false,
		attitude: new_attitude,
		send    : true
	});

	while frame.ships.get(0).velocity == velocity {
		frame = client.frame();
	}

	assert_eq!(
		new_velocity.round(16),
		frame.ships.get(0).velocity.round(16));
}

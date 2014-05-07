use common::io::Input;
use common::physics::Radians;

use control::{
	Client,
	GameService
};


#[test]
fn the_ship_should_follow_its_velocity_vector() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	while frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	while frame_1.ships[0].position == frame_2.ships[0].position {
		frame_2 = client.frame();
	}

	let movement = frame_2.ships[0].position - frame_1.ships[0].position;
	let velocity = frame_1.ships[0].velocity;

	assert_eq!(
		velocity.normalize().round(16),
		movement.normalize().round(16));
}

#[test]
fn the_ship_should_change_direction_according_to_input() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame = client.frame();

	while frame.ships.len() == 0 {
		frame = client.frame();
	}

	let velocity     = frame.ships[0].velocity;
	let new_velocity = velocity * -1.0;
	let new_attitude = Radians::from_vec(new_velocity);

	client.input(Input {
		exit    : false,
		attitude: new_attitude,
		send    : true
	});

	while frame.ships[0].velocity == velocity {
		frame = client.frame();
	}

	assert_eq!(
		new_velocity.round(16),
		frame.ships[0].velocity.round(16));
}

#[test]
fn the_camera_should_follow_the_ship() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	while frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	while frame_1.ships[0].position == frame_2.ships[0].position {
		frame_2 = client.frame();
	}

	assert_eq!(
		frame_1.ships[0].position,
		frame_1.camera);
	assert_eq!(
		frame_2.ships[0].position,
		frame_2.camera);
}

#[test]
fn it_should_render_all_connected_clients() {
	let     game_service = GameService::start();
	let mut client_1     = Client::start(game_service.port);

	let mut frame = client_1.frame();

	while frame.ships.len() == 0 {
		frame = client_1.frame();
	}

	assert_eq!(
		1,
		frame.ships.len());

	let mut client_2 = Client::start(game_service.port);

	while frame.ships.len() == 1 {
		frame = client_1.frame();
	}

	assert_eq!(
		2,
		frame.ships.len());

	client_2.stop();

	while frame.ships.len() == 2 {
		frame = client_1.frame();
	}

	assert_eq!(
		1,
		frame.ships.len());

	let mut client_3 = Client::start(game_service.port);

	while frame.ships.len() == 1 {
		frame = client_1.frame();
	}

	assert_eq!(
		2,
		frame.ships.len());

	client_3.stop();
}

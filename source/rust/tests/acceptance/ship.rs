use common::io::Input;
use common::physics::Radians;

use common::testing::{
	Client,
	GameService
};


#[test]
fn it_should_change_direction_according_to_input() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame = client.frame();

	wait_while!(frame.ships.len() == 0 {
		frame = client.frame();
	})

	let velocity     = frame.ships.get(0).velocity;
	let new_velocity = velocity * -1.0;
	let new_attitude = Radians::from_vec(new_velocity);

	let mut input  = Input::default();
	input.attitude = new_attitude;
	client.input(input);

	wait_while!(frame.ships.get(0).velocity == velocity && true {
		frame = client.frame();
	})

	assert_eq!(
		new_velocity.round(16),
		frame.ships.get(0).velocity.round(16));
}

#[test]
fn it_should_fire_a_missile() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame = client.frame();

	wait_while!(frame.ships.len() == 0 {
		frame = client.frame();
	})

	let mut input = Input::default();
	input.missile = 1;
	client.input(input);

	wait_while!(frame.missiles.len() == 0 {
		frame = client.frame();
	})

	let distance =
		(frame.ships.get(0).position - frame.missiles.get(0).position).mag();

	print!("distance: {}\n", distance);
	assert!(distance < 5.0);
}

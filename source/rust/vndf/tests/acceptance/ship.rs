use cgmath::{
	ApproxEq,
	EuclideanVector,
	Vector,
	Vector2,
};

use platform::Input;
use test_tools::{
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

	let velocity     = frame.ships[0].velocity;
	let new_velocity = velocity.mul_s(-1.0);
	let new_attitude = new_velocity.angle(&Vector2::new(1.0, 0.0));

	let mut input  = Input::default();
	input.attitude = new_attitude;
	client.input(input);

	wait_while!(frame.ships.get(0).velocity == velocity && true {
		frame = client.frame();
	})

	assert!(new_velocity.approx_eq_eps(&frame.ships.get(0).velocity, &1e-4));
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
		(frame.ships[0].position - frame.missiles[0].position).length();

	print!("distance: {}\n", distance);
	assert!(distance < 5.0);
}

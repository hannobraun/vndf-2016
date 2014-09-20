use cgmath::{
	EuclideanVector,
	Quaternion,
	Rad,
	Rotation,
	Rotation3,
	Vector3,
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

	let old_velocity = frame.ships[0].velocity;
	let old_attitude = frame.ships[0].attitude;
	let new_attitude = Quaternion::identity()
		.mul_q(&Rotation3::from_angle_z(Rad::turn_div_6()))
		.mul_q(&Rotation3::from_angle_y(Rad::turn_div_6()));

	// If we picked the old attitude as the new attitude by accident, this would
	// mess up the test.
	assert!(old_attitude != new_attitude);

	let mut input  = Input::default();
	input.attitude = new_attitude;
	client.input(input);

	wait_while!(frame.ships.get(0).attitude == old_attitude && true {
		frame = client.frame();
	})

	let new_velocity = frame.ships[0].velocity;
	let attitude_vec = new_attitude.rotate_vector(&Vector3::new(1.0, 0.0, 0.0));

	let old_angle = attitude_vec.angle(&old_velocity);
	let new_angle = attitude_vec.angle(&new_velocity);

	// Without being to specific about the thrust we produce and the integration
	// method used, we can certainly assume that the angle between velocity and
	// attitude should have been reduced.
	assert!(new_angle < old_angle);
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

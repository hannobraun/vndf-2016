use std::f32::consts::PI;

use nalgebra::{
	Dot,
	Norm,
	Vec2,
};

use client::interface::InputEvent;
use common::util::is_point_on_line;
use test_suite::rc;


#[test]
fn it_should_send_navigation_data() {
	let     game_service = rc::Server::start();
	let mut client       = rc::Client::start(game_service.port());

	let frame_1 = client.frame();

	let frame_1 = client.wait_until(|frame|
		frame.position != frame_1.position
	);
	let frame_2 = client.wait_until(|frame|
		frame.position != frame_1.position
	);

	assert!(is_point_on_line(
		frame_2.position,
		frame_1.position, frame_1.velocity,
	));
}

#[test]
fn it_should_schedule_maneuvers() {
	let     game_service = rc::Server::start();
	let mut client       = rc::Client::start(game_service.port());

	let frame_1 = client.wait_until(|frame| {
		frame.velocity != Vec2::new(0.0, 0.0)
	});

	let velocity_direction_rad = angle_between(
		Vec2::new(1.0, 0.0),
		frame_1.velocity,
	);
	let maneuver_direction_rad = velocity_direction_rad + PI;

	client.input(InputEvent::ScheduleManeuver(maneuver_direction_rad));

	let frame_2 = client.wait_until(|frame| {
		frame_1.velocity != frame.velocity
	});

	let new_velocity_direction_rad = angle_between(
		Vec2::new(1.0, 0.0),
		frame_2.velocity,
	);

	assert_eq!(maneuver_direction_rad, new_velocity_direction_rad);
}


fn angle_between(v1: Vec2<f32>, v2: Vec2<f32>) -> f32 {
	(v1.dot(&v2) / (v1.norm() * v2.norm())).acos()
}

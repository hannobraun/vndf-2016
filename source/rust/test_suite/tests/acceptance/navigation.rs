use std::f64::consts::PI;

use nalgebra::{
	Dot,
	Norm,
	Vec2,
};

use client::interface::InputEvent;
use shared::util::is_point_on_line;
use test_suite::rc;


#[test]
fn it_should_send_navigation_data() {
	let     server = rc::Server::start();
	let mut client = rc::Client::start(server.port());

	let frame_1 = client.frame();

	let frame_1 = client.wait_until(|frame|
		frame.ship.position != frame_1.ship.position
	);
	let frame_2 = client.wait_until(|frame|
		frame.ship.position != frame_1.ship.position
	);

	assert!(is_point_on_line(
		frame_2.ship.position,
		frame_1.ship.position, frame_1.ship.velocity,
	));
}

#[test]
fn it_should_schedule_maneuvers() {
	let     server = rc::Server::start();
	let mut client = rc::Client::start(server.port());

	let frame_1 = client.wait_until(|frame| {
		frame.ship.velocity != Vec2::new(0.0, 0.0)
	});

	let velocity_direction_rad = angle_between(
		Vec2::new(1.0, 0.0),
		frame_1.ship.velocity,
	);
	let maneuver_direction_rad = velocity_direction_rad + PI;

	client.input(InputEvent::ScheduleManeuver(maneuver_direction_rad));

	let frame_2 = client.wait_until(|frame| {
		frame_1.ship.velocity != frame.ship.velocity
	});

	let new_velocity_direction_rad = angle_between(
		Vec2::new(1.0, 0.0),
		frame_2.ship.velocity,
	);

	assert_eq!(maneuver_direction_rad, new_velocity_direction_rad);
}


fn angle_between(v1: Vec2<f64>, v2: Vec2<f64>) -> f64 {
	(v1.dot(&v2) / (v1.norm() * v2.norm())).acos()
}

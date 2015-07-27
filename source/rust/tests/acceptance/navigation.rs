use std::f64::consts::PI;

use vndf::client::interface::InputEvent;
use vndf::shared::util::{
	angle_of,
	is_point_on_line,
};
use vndf::testing::rc;


#[test]
fn it_should_send_navigation_data() {
	let     server = rc::Server::start();
	let mut client = rc::Client::start(server.port());

	let frame_1 = client.wait_until(|frame|
		frame.ship_id.is_some() && frame.ships.len() == 1
	);
	let ship_id = match frame_1.ship_id {
		Some(ship_id) => ship_id,
		None          => panic!("Expected ship id"),
	};

	let frame_1 = client.wait_until(|frame|
		frame.ships[&ship_id].position != frame_1.ships[&ship_id].position
	);
	let frame_2 = client.wait_until(|frame|
		frame.ships[&ship_id].position != frame_1.ships[&ship_id].position
	);

	assert!(is_point_on_line(
		frame_2.ships[&ship_id].position,
		frame_1.ships[&ship_id].position, frame_1.ships[&ship_id].velocity,
	));
}

#[test]
fn it_should_display_other_players_ships() {
	let     server   = rc::Server::start();
	let mut client_a = rc::Client::start(server.port());

	client_a.wait_until(|frame|
		frame.ships.len() == 1
	);

	let mut client_b = rc::Client::start(server.port());

	client_a.wait_until(|frame|
		frame.ships.len() == 2
	);
	client_b.wait_until(|frame|
		frame.ships.len() == 2
	);

	drop(client_b);

	client_a.wait_until(|frame|
		frame.ships.len() == 1
	);
}

#[test]
fn it_should_schedule_maneuvers() {
	let     server = rc::Server::start();
	let mut client = rc::Client::start(server.port());

	let frame = client.wait_until(|frame| {
		frame.ship_id.is_some() && frame.ships.len() == 1
	});
	let ship_id = match frame.ship_id {
		Some(ship_id) => ship_id,
		None          => panic!("Expected ship id"),
	};

	let velocity_direction_rad = angle_of(frame.ships[&ship_id].velocity);
	let maneuver_direction_rad = velocity_direction_rad + PI / 2.0;

	client.input(InputEvent::ScheduleManeuver(0.0, maneuver_direction_rad));

	client.wait_until(|frame| {
		maneuver_direction_rad == angle_of(frame.ships[&ship_id].velocity)
	});
}
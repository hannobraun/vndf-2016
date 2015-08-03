use std::f64::consts::PI;

use time::precise_time_s;

use vndf::client::interface::InputEvent;
use vndf::shared::game::ManeuverData;
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

	// TODO: This is highly error-prone, as the local time might be different
	//       from the server time. What we should use here is a kind of game
	//       time that the server provides us.
	let start_s = precise_time_s();

	let data = ManeuverData {
		start_s   : start_s,
		duration_s: 1.0,
		angle     : maneuver_direction_rad,
	};

	client.input(InputEvent::ScheduleManeuver(data));

	client.wait_until(|frame| {
		let new_velocity_direction_rad =
			angle_of(frame.ships[&ship_id].velocity);

		let old_difference =
			(maneuver_direction_rad - velocity_direction_rad).abs();
		let new_difference =
			(maneuver_direction_rad - new_velocity_direction_rad).abs();

		// This test is too high-level to really test any of the details, so we
		// just check whether the maneuver moved the velocity vector in the
		// right direction.
		new_difference < old_difference
	});
}

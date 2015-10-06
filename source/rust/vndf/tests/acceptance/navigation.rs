use std::f64::consts::PI;

use vndf::server::game::initial_state::InitialState;
use vndf::client::interface::InputEvent;
use vndf::shared::game::ManeuverData;
use vndf::shared::util::{
	angle_of,
	is_point_on_line,
};
use vndf::testing::rc;


#[test]
fn it_should_send_navigation_data() {
	let     server = rc::Server::start(InitialState::new());
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

	let p  = frame_2.ships[&ship_id].position;
	let l1 = frame_1.ships[&ship_id].position;
	let l2 = l1 + frame_1.ships[&ship_id].velocity;

	assert!(is_point_on_line(p, l1, l2));
}

#[test]
fn it_should_display_other_players_ships() {
	let     server   = rc::Server::start(InitialState::new());
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
	let     server = rc::Server::start(InitialState::new());
	let mut client = rc::Client::start(server.port());

	let frame = client.wait_until(|frame| {
		frame.game_time_s.is_some() &&
			frame.ship_id.is_some() &&
			frame.ships.len() == 1
	});
	let ship_id = match frame.ship_id {
		Some(ship_id) => ship_id,
		None          => panic!("Expected ship id"),
	};

	let velocity_direction_rad = angle_of(frame.ships[&ship_id].velocity);
	let maneuver_direction_rad = velocity_direction_rad + PI / 2.0;

	let data = ManeuverData {
		start_s   : frame.game_time_s.unwrap(),
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

#[test]
fn scheduled_maneuvers_should_be_visible() {
	let     server = rc::Server::start(InitialState::new());
	let mut client = rc::Client::start(server.port());

	let frame = client.wait_until(|frame| {
		frame.game_time_s.is_some()
	});

	let maneuver_data = ManeuverData {
		start_s   : frame.game_time_s.unwrap() + 1000.0,
		duration_s: 1.0,
		angle     : 0.0,
	};

	client.input(InputEvent::ScheduleManeuver(maneuver_data));

	client.wait_until(|frame| {
		frame.maneuvers.len() == 1 &&
			*frame.maneuvers.iter().next().unwrap().1 == maneuver_data
	});
}

#[test]
fn finished_maneuvers_should_be_removed() {
	let     server = rc::Server::start(InitialState::new());
	let mut client = rc::Client::start(server.port());

	let frame = client.wait_until(|frame| {
		frame.game_time_s.is_some()
	});

	let data = ManeuverData {
		start_s   : frame.game_time_s.unwrap() + 0.1,
		duration_s: 0.1,
		angle     : 0.0,
	};

	client.input(InputEvent::ScheduleManeuver(data));

	client.wait_until(|frame| {
		frame.maneuvers.len() == 1
	});
	client.wait_until(|frame| {
		frame.maneuvers.len() == 0
	});
}

#[test]
fn players_should_only_see_their_own_maneuvers() {
	let     server   = rc::Server::start(InitialState::new());
	let mut client_a = rc::Client::start(server.port());

	let frame = client_a.wait_until(|frame| {
		frame.game_time_s.is_some()
	});

	let data = ManeuverData {
		start_s   : frame.game_time_s.unwrap() + 1000.0,
		duration_s: 1.0,
		angle     : 0.0,
	};

	client_a.input(InputEvent::ScheduleManeuver(data));

	let mut client_b = rc::Client::start(server.port());

	let frame = client_b.wait_until(|frame| {
		frame.ships.len() == 2
	});

	assert_eq!(frame.maneuvers.len(), 0);
}

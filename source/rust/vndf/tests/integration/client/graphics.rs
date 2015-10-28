use nalgebra::{
	Norm,
	Vec2,
};

use vndf::shared::game::data::{
	Body,
	Ship,
};
use vndf::shared::protocol::{
	client,
	server,
};
use vndf::shared::protocol::server::Entity;
use vndf::shared::util::is_point_on_line;
use vndf::testing::{
	mock,
	rc,
};


#[test]
fn it_should_interpolate_between_snapshots_sent_by_server() {
	let mut server = mock::Server::start();
	let mut client = rc::Client::start(server.port());

	let event = server.wait_until(|event|
		if let &mut Some((_, ref event)) = event {
			*event == client::Event::Public(client::event::Public::Login)
		}
		else {
			false
		}
	);
	let address = event.unwrap().0;

	let position_1 = Vec2::new(1.0, 2.0);
	let position_2 = Vec2::new(2.0, 2.5);

	let mut ship = Entity {
		id: 0,

		body: Some(Body {
			position: position_1,
			velocity: Vec2::new(1.0, 0.5),
			force   : Vec2::new(0.0, 0.0),
			mass    : 0.0,
		}),
		ship: Some(Ship),

		broadcast: None,
		maneuver : None,
		planet   : None,
	};

	server.send(address, server::Event::Heartbeat(1.0));
	server.send(address, server::Event::UpdateEntity(ship.clone()));

	ship.body.as_mut().unwrap().position = position_2;

	server.send(address, server::Event::Heartbeat(2.0));
	server.send(address, server::Event::UpdateEntity(ship));

	let frame_1 = client.wait_until(|frame| {
		frame.ships.len() == 1
	});
	let frame_2 = client.frame();

	let ship_1 = frame_1.ships.iter().next().unwrap().1;
	let ship_2 = frame_2.ships.iter().next().unwrap().1;

	assert_interpolation(
		ship_1.position,
		ship_2.position,
		position_1,
		position_2,
	);
}

fn assert_interpolation(
	interpolated_1: Vec2<f64>,
	interpolated_2: Vec2<f64>,
	snapshot_1    : Vec2<f64>,
	snapshot_2    : Vec2<f64>,
) {
	// The interpolated positions must be somewhere between the two positions.
	assert!(is_point_on_line(
		interpolated_1,
		snapshot_1,
		snapshot_2,
	));
	assert!(is_point_on_line(
		interpolated_2,
		snapshot_1,
		snapshot_2,
	));

	let initial_to_1 = interpolated_1 - snapshot_1;
	let initial_to_2 = interpolated_2 - snapshot_1;

	// The second interpolated position must be closer to the final position.
	assert!(initial_to_1.norm() < initial_to_2.norm());
}

use cgmath::{
	Line,
	Point,
	Vector3,
	zero,
};

use game::ecs::{
	Entity,
	ShowAsShip,
};
use game::util;
use game::physics::Body;
use protocol::Perception;
use test_tools::{
	Client,
	MockGameService
};


#[test]
fn it_should_interpolate_between_perceptions() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port);

	game_service.accept_client();

	let pos_1 = zero();
	let pos_2 = Vector3::new(10.0, 0.0, 0.0);

	let mut entity = Entity {
		body: Some({
			let mut body = Body::new();
			body.position = pos_1;
			body.velocity = Vector3::new(10.0, 0.0, 0.0);
			body
		}),
		visual: Some(ShowAsShip),
		planet: None,
	};

	let perception_1 = Perception::new(
		|&(id, _)| id,
		Some(0),
		vec!(),
		vec!((0, entity))
	);
	entity.body.as_mut().unwrap().position = pos_2;
	let perception_2 = Perception::new(
		|&(id, _)| id,
		Some(0),
		vec!((0, entity)),
		vec!((0, entity))
	);

	game_service.send_perception(&perception_1);
	game_service.send_perception(&perception_2);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	wait_while!(frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	});

	wait_while!(frame_1.ships[0].position == pos_1 && true {
		frame_1 = frame_2;
		frame_2 = client.frame();
	});

	assert!(util::is_on_line(
		Line::new(
			Point::from_vec(&pos_1),
			Point::from_vec(&pos_2),
		),
		Point::from_vec(&frame_1.ships[0].position),
	));
	assert!(util::is_on_line(
		Line::new(
			Point::from_vec(&pos_1),
			Point::from_vec(&pos_2),
		),
		Point::from_vec(&frame_2.ships[0].position),
	));
	assert!(frame_2.ships[0].position != pos_2);
}

#[test]
fn the_camera_should_follow_the_ship() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port);

	game_service.accept_client();

	let pos_1 = zero();
	let pos_2 = Vector3::new(10.0, 0.0, 0.0);

	let mut entity = Entity {
		body: Some({
			let mut body = Body::new();
			body.position = pos_1;
			body.velocity = Vector3::new(10.0, 0.0, 0.0);
			body
		}),
		visual: Some(ShowAsShip),
		planet: None,
	};

	let perception_1 = Perception::new(
		|&(id, _)| id,
		Some(0),
		vec!(),
		vec!((0, entity))
	);
	entity.body.as_mut().unwrap().position = pos_2;
	let perception_2 = Perception::new(
		|&(id, _)| id,
		Some(0),
		vec!((0, entity)),
		vec!((0, entity))
	);

	game_service.send_perception(&perception_1);
	let mut frame_1 = client.frame();

	game_service.send_perception(&perception_2);
	let mut frame_2 = client.frame();

	wait_while!(frame_2.camera.center == pos_1 && true {
		frame_1 = frame_2;
		frame_2 = client.frame();
	});

	assert!(util::is_on_line(
		Line::new(
			Point::from_vec(&pos_1),
			Point::from_vec(&pos_2),
		),
		Point::from_vec(&frame_1.camera.center),
	));
	assert!(util::is_on_line(
		Line::new(
			Point::from_vec(&pos_1),
			Point::from_vec(&pos_2),
		),
		Point::from_vec(&frame_2.camera.center),
	));
}

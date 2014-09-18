use cgmath::Vector2;

use game::ecs::{
	SharedWorldEntity,
	ShowAsShip,
};
use physics::{
	Body,
	Radians,
	util,
	Vec2
};
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

	let pos_1 = Vector2::zero();
	let pos_2 = Vector2::new(10.0, 0.0);

	let mut entity = SharedWorldEntity {
		id: 0,
		body: Some(Body {
			position: Vec2(pos_1[0], pos_1[1]),
			velocity: Vec2(10.0, 0.0),
			attitude: Radians(0.0)
		}),
		visual: Some(ShowAsShip)
	};

	let perception_1 = Perception::new(
		|entity| entity.id,
		0u32,
		vec!(),
		vec!(entity)
	);
	entity.body.as_mut().unwrap().position = Vec2(pos_2[0], pos_2[1]);
	let perception_2 = Perception::new(
		|entity| entity.id,
		0u32,
		vec!(entity),
		vec!(entity)
	);

	game_service.send_perception(&perception_1);
	game_service.send_perception(&perception_2);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	wait_while!(frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	});

	wait_while!(frame_1.ships.get(0).position == Vec2(pos_1[0], pos_1[1]) && true {
		frame_1 = frame_2;
		frame_2 = client.frame();
	});

	assert!(util::is_on_line(
		(pos_1, pos_2),
		frame_1.ships.get(0).position.to_vector2_f64(),
		16));
	assert!(util::is_on_line(
		(pos_1, pos_2),
		frame_2.ships.get(0).position.to_vector2_f64(),
		16));
	assert!(frame_2.ships.get(0).position != Vec2(pos_2[0], pos_2[1]));
}

#[test]
fn the_camera_should_follow_the_ship() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port);

	game_service.accept_client();

	let pos_1 = Vector2::zero();
	let pos_2 = Vector2::new(10.0, 0.0);

	let mut entity = SharedWorldEntity {
		id: 0,
		body: Some(Body {
			position: Vec2(pos_1[0], pos_1[1]),
			velocity: Vec2(10.0, 0.0),
			attitude: Radians(0.0)
		}),
		visual: Some(ShowAsShip)
	};

	let perception_1 = Perception::new(
		|entity| entity.id,
		0u32,
		vec!(),
		vec!(entity)
	);
	entity.body.as_mut().unwrap().position = Vec2(pos_2[0], pos_2[1]);
	let perception_2 = Perception::new(
		|entity| entity.id,
		0u32,
		vec!(entity),
		vec!(entity)
	);

	game_service.send_perception(&perception_1);
	let mut frame_1 = client.frame();

	game_service.send_perception(&perception_2);
	let mut frame_2 = client.frame();

	wait_while!(frame_2.camera.center == pos_1 && true {
		frame_1 = frame_2;
		frame_2 = client.frame();
	});

	assert!(
		util::is_on_line(
			(pos_1, pos_2),
			frame_1.camera.center,
			16
		)
	);
	assert!(
		util::is_on_line(
			(pos_1, pos_2),
			frame_2.camera.center,
			16
		)
	);
}

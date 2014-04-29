use std::intrinsics::TypeId;

use common::physics::Vec2;
use common::protocol::{
	Create,
	Remove,
	SelfInfo
};
use common::physics::{Degrees, Radians};

use control::{
	Client,
	ClientCore,
	GameService
};


#[test]
fn the_ship_should_follow_its_velocity_vector() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port);

	let mut frame_1 = client.frame();
	let mut frame_2 = client.frame();

	while frame_1.ships.len() == 0 {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	// This is necessary because the ship is created due to a CREATE message,
	// which zeroes everything. After the network refactoring, this can be
	// cleaned up.
	while frame_1.ships[0].position == Vec2::zero() {
		frame_1 = frame_2;
		frame_2 = client.frame();
	}

	while frame_1.ships[0].position == frame_2.ships[0].position {
		frame_2 = client.frame();
	}

	let movement = frame_2.ships[0].position - frame_1.ships[0].position;
	let velocity = frame_1.ships[0].velocity;

	assert_eq!(
		velocity.normalize().round(16),
		movement.normalize().round(16));
}

#[test]
fn it_should_send_updates_for_connected_clients() {
	let     game_service = GameService::start();
	let mut client_a     = ClientCore::start(game_service.port);
	let mut client_b     = ClientCore::start(game_service.port);

	client_a.ignore(TypeId::of::<Create>());
	client_a.ignore(TypeId::of::<Remove>());
	client_b.ignore(TypeId::of::<Create>());

	let client_a_id = client_a.expect_self_info().id;
	let client_b_id = client_b.expect_self_info().id;

	let mut update_for_a = false;
	let mut update_for_b = false;
	for _ in range(0, 10) {
		let update = client_a.expect_update();

		if update.id == client_a_id {
			update_for_a = true;
		}
		if update.id == client_b_id {
			update_for_b = true;
		}
	}

	assert!(update_for_a);
	assert!(update_for_b);

	client_b.stop();

	for _ in range(0, 10) {
		client_a.expect_update();
	}

	update_for_a = false;
	update_for_b = false;

	for _ in range(0, 10) {
		let update = client_a.expect_update();

		if update.id == client_a_id {
			update_for_a = true;
		}
		if update.id == client_b_id {
			update_for_b = true;
		}
	}

	assert!(update_for_a);
	assert!(!update_for_b);
}

#[test]
fn the_ship_should_change_direction_according_to_input() {
	let     game_service = GameService::start();
	let mut client       = ClientCore::start(game_service.port);

	client.ignore(TypeId::of::<SelfInfo>());
	client.ignore(TypeId::of::<Create>());

	let attitude = Degrees(90.0).to_radians();

	client.send_attitude(attitude);
	client.expect_update();
	client.expect_update();
	let update = client.expect_update();

	assert_eq!(
		attitude.round(16),
		update.body.attitude.round(16));
	assert_eq!(
		attitude.round(16),
		Radians::from_vec(update.body.velocity).round(16));
}

use std::intrinsics::TypeId;

use common::physics::Radians;
use common::protocol::{Create, SelfInfo};

use control::{ClientCore, GameService};


#[test]
fn the_ship_should_move_along_its_velocity_vector() {
	let     game_service = GameService::start();
	let mut client       = ClientCore::start(game_service.port);

	client.ignore(TypeId::of::<SelfInfo>());
	client.ignore(TypeId::of::<Create>());

	let update_1 = client.expect_update();
	let update_2 = client.expect_update();

	let movement = update_2.body.position - update_1.body.position;
	let velocity = update_1.body.velocity;

	assert_eq!(
		movement.normalize().round(16),
		velocity.normalize().round(16));
}

#[test]
fn the_ships_attitude_should_match_its_velocity() {
	let     game_service = GameService::start();
	let mut client       = ClientCore::start(game_service.port);

	client.ignore(TypeId::of::<SelfInfo>());
	client.ignore(TypeId::of::<Create>());

	let update = client.expect_update();

	assert_eq!(
		Radians::from_vec(update.body.velocity).round(16),
		update.body.attitude.round(16));
}

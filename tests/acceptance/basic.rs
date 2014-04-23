use std::intrinsics::TypeId;

use common::protocol::{Create, SelfInfo};
use common::physics::{Degrees, Radians};

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

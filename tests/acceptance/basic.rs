use std::intrinsics::TypeId;

use common::protocol::{Create, SelfInfo, Update};

use control::{ClientCore, GameService};


#[test]
fn it_should_send_the_self_id_after_connecting() {
	let game_service = GameService::start();
	let mut client_a = ClientCore::start(game_service.port);
	let mut client_b = ClientCore::start(game_service.port);

	assert_eq!(0, client_a.expect_self_id());
	assert_eq!(1, client_b.expect_self_id());
}

#[test]
fn it_should_send_create_to_all_clients_on_connect() {
	let game_service    = GameService::start();
	let mut this_client = ClientCore::start(game_service.port);
	let other_client    = ClientCore::start(game_service.port);

	let _ = other_client;

	this_client.ignore(TypeId::of::<SelfInfo>());
	this_client.ignore(TypeId::of::<Update>());

	let create_a = this_client.expect_create();
	let create_b = this_client.expect_create();

	assert_eq!(0, create_a.id);
	assert_eq!(1, create_b.id);
}

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
		movement.normalize().round(32),
		velocity.normalize().round(32));
}

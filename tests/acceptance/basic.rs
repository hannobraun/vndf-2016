use std::intrinsics::TypeId;

use common::protocol::SelfInfo;

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
fn the_ship_should_move_on_a_straight_line() {
	let     game_service = GameService::start();
	let mut client       = ClientCore::start(game_service.port);

	client.ignore(TypeId::of::<SelfInfo>());

	let update1 = client.expect_update();
	let update2 = client.expect_update();
	let update3 = client.expect_update();

	let movement1 = update2.pos - update1.pos;
	let movement2 = update3.pos - update2.pos;

	assert_eq!(movement1.normalize(), movement2.normalize());
}

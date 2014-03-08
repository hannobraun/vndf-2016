use control::{ClientCore, GameService};


#[test]
fn it_should_connect_and_receive_updates() {
	#[allow(unused_variable)];
	let     game_service = GameService::start();
	let mut client_core  = ClientCore::start(game_service.port);

	client_core.expect_update();
}

#[test]
fn the_ship_should_move_on_a_straight_line() {
	#[allow(unused_variable)];
	let     game_service = GameService::start();
	let mut client_core  = ClientCore::start(game_service.port);

	let update1 = client_core.expect_update();
	let update2 = client_core.expect_update();
	let update3 = client_core.expect_update();

	let movement1 = update2.pos - update1.pos;
	let movement2 = update3.pos - update2.pos;

	assert_eq!(movement1.normalize(), movement2.normalize());
}

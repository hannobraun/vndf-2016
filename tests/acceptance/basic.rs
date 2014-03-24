use control::{ClientCore, GameService};

#[test]
fn the_ship_should_move_on_a_straight_line() {
	let     game_service = GameService::start();
	let mut client       = ClientCore::start(game_service.port);

	client.ignore_message(); // self id

	let update1 = client.expect_update();
	let update2 = client.expect_update();
	let update3 = client.expect_update();

	let movement1 = update2.pos - update1.pos;
	let movement2 = update3.pos - update2.pos;

	assert_eq!(movement1.normalize(), movement2.normalize());
}

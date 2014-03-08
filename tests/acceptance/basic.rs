use control::{ClientCore, GameService};


#[test]
fn it_should_connect_and_receive_updates() {
	#[allow(unused_variable)];
	let     game_service = GameService::start();
	let mut client_core  = ClientCore::start();

	client_core.expect_update();
}

use control::{ClientCore, GameService};
use util::Update;


#[test]
fn it_should_connect_and_receive_updates() {
	#[allow(unused_variable)];
	let     game_service = GameService::start();
	let mut client_core  = ClientCore::start();

	let update = Update::from_str(client_core.message());
	assert!(update != None);
}

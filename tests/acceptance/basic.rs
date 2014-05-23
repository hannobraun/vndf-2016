use common::testing::{
	Client,
	GameService
};


#[test]
fn it_should_render_all_connected_clients() {
	let     game_service = GameService::start();
	let mut client_1     = Client::start(game_service.port);

	let mut frame = client_1.frame();

	while frame.ships.len() == 0 {
		frame = client_1.frame();
	}

	assert_eq!(
		1,
		frame.ships.len());

	let mut client_2 = Client::start(game_service.port);

	while frame.ships.len() == 1 {
		frame = client_1.frame();
	}

	assert_eq!(
		2,
		frame.ships.len());

	client_2.stop();

	while frame.ships.len() == 2 {
		frame = client_1.frame();
	}

	assert_eq!(
		1,
		frame.ships.len());

	let mut client_3 = Client::start(game_service.port);

	while frame.ships.len() == 1 {
		frame = client_1.frame();
	}

	assert_eq!(
		2,
		frame.ships.len());

	client_3.stop();
}

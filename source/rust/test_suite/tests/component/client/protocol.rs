use common::protocol::{
	ClientEvent,
	ServerEvent,
};
use test_suite::{
	Client,
	MockGameService,
};


#[test]
fn it_should_display_an_error_if_connection_to_server_is_lost() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let event = game_service.wait_until(|event|
		if let &mut Some((_, ref event)) = event {
			event == &ClientEvent::Login
		}
		else {
			false
		}
	);

	let (address, _) = if let Some(event) = event {
		event
	}
	else {
		panic!("Expected event");
	};

	game_service.send(address, ServerEvent::Heartbeat);

	client.wait_until(|frame| frame.status.is_error());
}

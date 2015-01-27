use std::iter::repeat;

use common::protocol::ClientEvent;
use test_suite::{
	Client,
	MockGameService,
};


#[test]
fn it_should_reject_broadcasts_that_are_too_large_to_be_sent() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let invalid_broadcast: String =
		repeat('a').take(512 + 1).collect();
	let valid_broadcast = "This is a broadcast.".to_string();

	// It should show an error, if the broadcast is invalid.
	client.broadcast(invalid_broadcast.as_slice());
	client.wait_until(|frame| frame.status.is_error());

	// And it should still work afterwards.
	client.broadcast(valid_broadcast.as_slice());
	game_service.wait_until(|event| {
		print!("event: {:?}\n", event);
		event == &Some(ClientEvent::StartBroadcast(valid_broadcast.clone()))
	});
}

#[test]
fn it_should_reject_empty_broadcasts() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	client.broadcast("");
	client.wait_until(|frame| frame.status.is_error());
}

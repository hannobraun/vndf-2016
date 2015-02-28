use std::iter::repeat;

use common::protocol::ClientEvent;
use test_suite::{
	mock,
	rc,
};


#[test]
fn it_should_reject_broadcasts_that_are_too_large_to_be_sent() {
	let mut game_service = mock::GameService::start();
	let mut client       = rc::Client::start(game_service.port());

	let invalid_broadcast: String =
		repeat('a').take(512 + 1).collect();
	let valid_broadcast = "This is a broadcast.".to_string();

	// It should show an error, if the broadcast is invalid.
	client.start_broadcast(invalid_broadcast.as_slice());
	client.wait_until(|frame| frame.message.is_error());

	// And it should still work afterwards.
	client.start_broadcast(valid_broadcast.as_slice());
	game_service.wait_until(|event| {
		if let &mut Some((_, ref event)) = event {
			event == &ClientEvent::StartBroadcast(valid_broadcast.clone())
		}
		else {
			false
		}
	});
}

#[test]
fn it_should_reject_empty_broadcasts() {
	let     game_service = mock::GameService::start();
	let mut client       = rc::Client::start(game_service.port());

	client.start_broadcast("");
	client.wait_until(|frame| frame.message.is_error());
}

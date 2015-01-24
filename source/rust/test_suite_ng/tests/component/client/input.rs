use std::iter::repeat;

use acpe::MAX_PACKET_SIZE;

use common::protocol::Step;
use test_suite::{
	Client,
	MockGameService,
};


#[test]
fn it_should_reject_broadcasts_that_are_too_large_to_be_sent() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let invalid_broadcast: String =
		repeat('a').take(MAX_PACKET_SIZE + 1).collect();
	let valid_broadcast = "This is a broadcast.".to_string();

	// It should show an error, if the broadcast is invalid.
	client.broadcast(invalid_broadcast.as_slice());
	client.wait_until(|frame| frame.status.is_error());

	// And it should still work afterwards.
	client.broadcast(valid_broadcast.as_slice());
	game_service.wait_until(|action| {
		let action = action.as_mut().unwrap();
		action.update_items().any(|&(_, ref entity)|
			entity == &Step::Broadcast(valid_broadcast.clone())
		)
	});
}

#[test]
fn it_should_reject_empty_broadcasts() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	client.broadcast("");
	client.wait_until(|frame| frame.status.is_error());
}

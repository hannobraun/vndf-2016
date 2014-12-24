use acpe::MAX_PACKET_SIZE;

use common::protocol::Step;
use test_tools::{
	Client,
	MockGameService,
};


#[test]
fn it_should_resend_actions_until_confirmed() {
	let mut game_service = MockGameService::start();
	let     _client      = Client::start(game_service.port());

	// No need to send an explicit action, Login will do.
	game_service.expect_action().unwrap().ignore(); // first try
	game_service.expect_action().unwrap().confirm(); //second try

	// Assert either no action was sent or, if it was, that it was empty.
	match game_service.expect_action() {
		Some(action) =>
			assert!(action.inner.steps.len() == 0),
		None =>
			(),
	}
}

#[test]
fn it_should_keep_track_of_which_actions_are_confirmed() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let broadcast = "This is a broadcast.".to_string();

	{
		let mut login = game_service.expect_action().unwrap();
		login.confirm();
		client.broadcast(broadcast.as_slice());
		game_service.wait_until(|action| {
			let ref action = action.as_ref().unwrap().inner;
			action.steps.contains(&Step::Broadcast(broadcast.clone()))
		});
		login.confirm();
	}

	// Broadcast has never been confirmed. It should keep sending it.
	let action = game_service.expect_action().unwrap().inner;
	assert!(action.steps.contains(&Step::Broadcast(broadcast)));
}

#[test]
fn it_should_reject_broadcasts_that_are_too_large_to_be_sent() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	// TODO: Remove confirmation once ActionAssembler stops insisting on it.
	game_service.expect_action().unwrap().confirm(); // Confirm login

	let invalid_broadcast = String::from_char(MAX_PACKET_SIZE + 1, 'a');
	let valid_broadcast   = "This is a broadcast.".to_string();

	// It should show an error, if the broadcast is invalid.
	client.broadcast(invalid_broadcast.as_slice());
	client.wait_until(|frame| frame.status.is_error());

	// And it should still work afterwards.
	client.broadcast(valid_broadcast.as_slice());
	game_service.wait_until(|action| {
		let action = action.as_mut().unwrap();
		// TODO: Remove confirmation once ActionAssembler stops insisting on it.
		action.confirm();
		action.inner.steps.contains(
			&Step::Broadcast(valid_broadcast.clone())
		)
	});
}

#[test]
fn it_should_distribute_large_payloads_over_multiple_packets() {
	// TODO(83305336): The previous version of this test relied on the client
	//                 sending multiple broadcasts in a single action, which is
	//                 wasteful behavior which can't be relied on. Once
	//                 scheduling maneuvers has been implemented, it will be
	//                 possible to legitimately overflow an action. This test
	//                 should be re-introduced then.
}

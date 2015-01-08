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
			assert_eq!(action.inner.update_items().count(), 0),
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
			action.update_items().any(|&(_, ref entity)|
				entity == &Step::Broadcast(broadcast.clone())
			)
		});
		login.confirm();
	}

	// Broadcast has never been confirmed. It should keep sending it.
	let action = game_service.expect_action().unwrap().inner;
	assert!(action.update_items().any(|&(_, ref entity)|
		entity == &Step::Broadcast(broadcast.clone())
	));
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

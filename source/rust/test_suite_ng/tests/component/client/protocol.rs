use protocol_ng::Step;
use test_tools_ng::{
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
		// TODO(83305336): This is just the command argument, what's missing is
		//                 the command. Once we need more than one command, this
		//                 should be something like "broadcast ...", instead of
		//                 only "...".
		client.command(format!("{}", broadcast).as_slice());
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

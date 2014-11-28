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

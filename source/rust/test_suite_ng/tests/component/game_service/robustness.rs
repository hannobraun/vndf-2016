use test_suite::{
	GameService,
	MockClient,
};


#[test]
fn it_should_keep_working_if_client_doesnt_log_in_first() {
	let     game_service = GameService::start();
	let mut client_1     = MockClient::start(game_service.port());

	client_1.broadcast(0, "This is a broadcast.".to_string());

	let mut client_2 = MockClient::start(game_service.port());
	client_2.login(0);

	assert!(client_2.expect_perception().is_some());
}

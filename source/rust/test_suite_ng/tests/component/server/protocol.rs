use protocol_ng::{
	Action,
	Step,
};
use test_tools_ng::{
	GameService,
	MockClient,
};


#[test]
fn it_should_confirm_received_actions() {
	let     game_service = GameService::start();
	let mut client       = MockClient::start(game_service.port());

	let seq = 512;
	client.send_action(Action {
		seq  : seq,
		steps: vec![Step::Login],
	});

	let perception = client.expect_perception().unwrap();
	assert_eq!(seq, perception.last_action);
}

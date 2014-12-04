use test_tools::{
	Client,
	MockGameService,
};


#[test]
fn it_should_display_typed_input_to_the_user() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let input = "I'm typing, but not submitting yet, a command";
	client.input(input);

	client.wait_until(|frame| frame.input.as_slice() == input);
}

#[test]
fn it_should_ignore_control_characters() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let input = "abc\x11\x12\x13\x14";
	client.input(input);

	client.wait_until(|frame| frame.input.len() == 3);
}

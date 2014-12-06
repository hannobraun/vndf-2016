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

#[test]
fn it_should_remove_the_last_character_with_backspace() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	client.input("abc\x7f");
	client.wait_until(|frame| frame.input.as_slice() == "ab");

	client.input("\x7f\x7f\x7f\x7f");
	client.wait_until(|frame| {
		print!("{} {}\n", frame.input.as_slice() == "", frame.input.as_slice());
		frame.input.as_slice() == ""
	});
}

#[test]
fn it_should_display_an_error_when_entering_an_invalid_command() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	client.command("invalid-command");
	client.wait_until(|frame| frame.status.len() > 0);
}

#[test]
fn it_should_reset_the_error_after_a_successful_command() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	client.command("invalid-command");
	client.wait_until(|frame| frame.status.len() > 0);

	client.broadcast("This is a broadcast.");
	client.wait_until(|frame| frame.status.len() == 0);
}

#[test]
fn it_should_show_applicable_commands_depending_on_input() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let frame = client.frame();
	assert!(frame.commands.contains(&"broadcast".to_string()));
	assert!(frame.commands.contains(&"stop-broadcast".to_string()));

	client.input("bro");
	client.wait_until(|frame|
		!frame.commands.contains(&"stop-broadcast".to_string())
	);
	assert!(frame.commands.contains(&"broadcast".to_string()));
}

#[test]
fn it_should_display_help_messages() {
	let     game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	client.command("help");
	client.wait_until(|frame| frame.status.len() > 0);
}

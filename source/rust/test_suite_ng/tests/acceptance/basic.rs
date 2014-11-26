use test_tools_ng::Client;
use test_tools_ng::GameService;


#[test]
fn it_should_receive_broadcasts() {
	let     game_service = GameService::start();
	let mut client_1     = Client::start(game_service.port());
	let mut client_2     = Client::start(game_service.port());

	let message_1 = "This is a broadcast by client 1.".to_string();
	let message_2 = "This is a broadcast by client 2.".to_string();
	// TODO(83305336): This is just the command argument, what's missing is the
	//                 command. Once we need more than one command, this should
	//                 be something like "broadcast ...", instead of only "...".
	client_1.command(format!("{}", message_1).as_slice());
	client_2.command(format!("{}", message_2).as_slice());

	let frame_1 = client_1.wait_while(|frame| frame.broadcasts.len() < 2);
	let frame_2 = client_2.wait_while(|frame| frame.broadcasts.len() < 2);
	assert!(frame_1.broadcasts.contains(&message_1));
	assert!(frame_1.broadcasts.contains(&message_2));
	assert!(frame_2.broadcasts.contains(&message_1));
	assert!(frame_2.broadcasts.contains(&message_2));
}

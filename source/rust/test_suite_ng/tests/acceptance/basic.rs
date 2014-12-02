use test_tools::Client;
use test_tools::GameService;


#[test]
fn it_should_receive_broadcasts() {
	let     game_service = GameService::start();
	let mut client_1     = Client::start(game_service.port());
	let mut client_2     = Client::start(game_service.port());

	let message_1 = "This is a broadcast by client 1.".to_string();
	let message_2 = "This is a broadcast by client 2.".to_string();
	client_1.broadcast(message_1.as_slice());
	client_2.broadcast(message_2.as_slice());

	let frame_1 = client_1.wait_until(|frame| frame.broadcasts.len() >= 2);
	let frame_2 = client_2.wait_until(|frame| frame.broadcasts.len() >= 2);
	assert!(frame_1.broadcasts.contains(&message_1));
	assert!(frame_1.broadcasts.contains(&message_2));
	assert!(frame_2.broadcasts.contains(&message_1));
	assert!(frame_2.broadcasts.contains(&message_2));
}

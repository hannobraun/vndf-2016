use test_tools::rc::client_ng::Client;
use test_tools::rc::game_service_ng::GameService;


#[test]
fn it_should_receive_broadcasts() {
	let game_service = GameService::start();
	let client_1     = Client::start(game_service.port());
	let client_2     = Client::start(game_service.port());

	let message = "This is a broadcast.".to_string();
	client_1.command(format!("broadcast {}", message).as_slice());

	let frame_1 = client_1.wait_while(|frame| frame.broadcasts.len() == 0);
	let frame_2 = client_2.wait_while(|frame| frame.broadcasts.len() == 0);
	assert!(frame_1.broadcasts.contains(&message));
	assert!(frame_2.broadcasts.contains(&message));
}

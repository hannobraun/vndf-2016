use client::output::Broadcast;
use test_tools::{
	Client,
	GameService,
};


#[test]
fn it_should_receive_broadcasts() {
	let     game_service = GameService::start();
	let mut client_1     = Client::start(game_service.port());
	let mut client_2     = Client::start(game_service.port());

	let broadcast_1 = Broadcast {
		message: "This is a broadcast by client 1.".to_string()
	};
	let broadcast_2 = Broadcast {
		message: "This is a broadcast by client 2.".to_string()
	};
	client_1.broadcast(broadcast_1.message.as_slice());
	client_2.broadcast(broadcast_2.message.as_slice());

	let frame_1 = client_1.wait_until(|frame| frame.broadcasts.len() >= 2);
	let frame_2 = client_2.wait_until(|frame| frame.broadcasts.len() >= 2);
	assert!(frame_1.broadcasts.contains(&broadcast_1));
	assert!(frame_1.broadcasts.contains(&broadcast_2));
	assert!(frame_2.broadcasts.contains(&broadcast_1));
	assert!(frame_2.broadcasts.contains(&broadcast_2));
}

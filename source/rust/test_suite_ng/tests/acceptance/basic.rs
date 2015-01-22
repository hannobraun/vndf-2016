use client::platform::Frame;
use common::game::Broadcast;
use test_tools::{
	Client,
	GameService,
};


#[test]
fn it_should_send_broadcasts_to_all_clients() {
	fn contains(frame: &Frame, broadcast: (&String, &String)) -> bool {
		let (sender, message) = broadcast;

		frame.broadcasts.contains(&Broadcast {
			sender : sender.clone(),
			message: message.clone(),
		})
	}

	let     game_service = GameService::start();
	let mut client_1     = Client::start(game_service.port());
	let mut client_2     = Client::start(game_service.port());

	let message_1 = "This is a broadcast by client 1.".to_string();
	let message_2 = "This is a broadcast by client 2.".to_string();
	client_1.broadcast(message_1.as_slice());
	client_2.broadcast(message_2.as_slice());

	let frame_1 = client_1.wait_until(|frame| frame.broadcasts.len() == 2);
	let frame_2 = client_2.wait_until(|frame| frame.broadcasts.len() == 2);
	assert!(contains(&frame_1, (&frame_1.self_id, &message_1)));
	assert!(contains(&frame_1, (&frame_2.self_id, &message_2)));
	assert!(contains(&frame_2, (&frame_1.self_id, &message_1)));
	assert!(contains(&frame_2, (&frame_2.self_id, &message_2)));
}

#[test]
fn it_should_not_keep_sending_stopped_broadcasts() {
	let     game_service = GameService::start();
	let mut client       = Client::start(game_service.port());

	client.broadcast("This is a broadcast.");
	client.wait_until(|frame| frame.broadcasts.len() == 1);
	client.stop_broadcast();
	client.wait_until(|frame| frame.broadcasts.len() == 0);
}

#[test]
fn it_should_remove_clients_that_disconnected() {
	let     game_service = GameService::start();
	let mut client_a     = Client::start(game_service.port());
	let mut client_b     = Client::start(game_service.port());

	client_a.broadcast("Broadcast from A");
	client_b.broadcast("Broadcast from B");
	client_a.wait_until(|frame| frame.broadcasts.len() == 2);

	client_b.stop();
	client_a.wait_until(|frame| frame.broadcasts.len() == 1);
}

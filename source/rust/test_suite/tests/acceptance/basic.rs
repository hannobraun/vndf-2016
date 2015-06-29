use client::interface::Frame;
use shared::game::{
	Broadcast,
	EntityId,
};
use test_suite::rc;


#[test]
fn it_should_send_broadcasts_to_all_clients() {
	let     server   = rc::Server::start();
	let mut client_1 = rc::Client::start(server.port());
	let mut client_2 = rc::Client::start(server.port());

	let message_1 = "This is a broadcast by client 1.".to_string();
	let message_2 = "This is a broadcast by client 2.".to_string();
	client_1.start_broadcast(message_1.as_ref());
	client_2.start_broadcast(message_2.as_ref());

	let frame_1 = client_1.wait_until(|frame| frame.broadcasts.len() == 2);
	let frame_2 = client_2.wait_until(|frame| frame.broadcasts.len() == 2);
	assert!(contains(&frame_1, (frame_1.ship_id.as_ref().unwrap(), &message_1)));
	assert!(contains(&frame_1, (frame_2.ship_id.as_ref().unwrap(), &message_2)));
	assert!(contains(&frame_2, (frame_1.ship_id.as_ref().unwrap(), &message_1)));
	assert!(contains(&frame_2, (frame_2.ship_id.as_ref().unwrap(), &message_2)));


	fn contains(frame: &Frame, broadcast: (&EntityId, &String)) -> bool {
		let (sender, message) = broadcast;

		frame.broadcasts.contains(&Broadcast {
			sender : *sender,
			message: message.clone(),
		})
	}
}

#[test]
fn it_should_not_keep_sending_stopped_broadcasts() {
	let     server = rc::Server::start();
	let mut client = rc::Client::start(server.port());

	client.start_broadcast("This is a broadcast.");
	client.wait_until(|frame| frame.broadcasts.len() == 1);
	client.stop_broadcast();
	client.wait_until(|frame| frame.broadcasts.len() == 0);
}

#[test]
fn it_should_remove_clients_that_disconnected() {
	let     server   = rc::Server::start();
	let mut client_a = rc::Client::start(server.port());
	let mut client_b = rc::Client::start(server.port());

	client_a.start_broadcast("Broadcast from A");
	client_b.start_broadcast("Broadcast from B");
	client_a.wait_until(|frame| frame.broadcasts.len() == 2);

	client_b.stop();
	client_a.wait_until(|frame| frame.broadcasts.len() == 1);
}

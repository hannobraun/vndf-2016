use std::iter::repeat;

use shared::protocol::client;
use tests::{
	mock,
	rc,
};


#[test]
fn it_should_reject_broadcasts_that_are_too_large_to_be_sent() {
	let mut server = mock::Server::start();
	let mut client = rc::Client::start(server.port());

	let invalid_broadcast: String =
		repeat('a').take(512 + 1).collect();
	let valid_broadcast = "This is a broadcast.".to_string();

	// It should show an error, if the broadcast is invalid.
	client.start_broadcast(invalid_broadcast.as_ref());
	client.wait_until(|frame| frame.message.is_error());

	// And it should still work afterwards.
	client.start_broadcast(valid_broadcast.as_ref());
	server.wait_until(|event| {
		if let &mut Some((_, ref event)) = event {
			event == &client::Event::Privileged(client::event::Privileged::StartBroadcast(valid_broadcast.clone()))
		}
		else {
			false
		}
	});
}

#[test]
fn it_should_reject_empty_broadcasts() {
	let     server = mock::Server::start();
	let mut client = rc::Client::start(server.port());

	client.start_broadcast("");
	client.wait_until(|frame| frame.message.is_error());
}

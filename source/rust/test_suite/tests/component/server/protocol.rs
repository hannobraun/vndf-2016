use common::protocol::{
	client,
	server,
};
use test_suite::{
	mock,
	rc,
};


#[test]
fn it_should_ignore_clients_that_havent_logged_in() {
	let     server   = rc::Server::start();
	let mut client_1 = mock::Client::start(server.port());
	let mut client_2 = mock::Client::start(server.port());

	client_1.send(client::Event::StartBroadcast(
		"I haven't logged in, but am sending this anyway.".to_string(),
	));

	let message = "This is a broadcast.".to_string();
	client_2.send(client::Event::Login);
	client_2.send(client::Event::StartBroadcast(message.clone()));

	let mut received_message = String::new();
	client_2.wait_until(|event| {
		if let &Some(ref event) = event {
			if let &server::Event::StartBroadcast(ref broadcast) = event {
				received_message = broadcast.message.clone();
				true
			}
			else {
				false
			}
		}
		else {
			false
		}
	});

	assert_eq!(received_message, message)
}

#[test]
fn it_should_ignore_duplicate_logins() {
	let     server = rc::Server::start();
	let mut client = mock::Client::start(server.port());

	client.send(client::Event::Login);

	let mut self_id = String::new();
	client.wait_until(|event| {
		if let &Some(server::Event::SelfId(ref id)) = event {
			self_id = id.clone();
			true
		}
		else {
			false
		}
	});

	client.send(client::Event::Login);
	client.send(
		client::Event::StartBroadcast("This is a broadcast.".to_string())
	);

	let mut received_self_id   = None;
	client.wait_until(|event| {
		match *event {
			Some(ref event) => {
				match *event {
					server::Event::SelfId(ref self_id) => {
						received_self_id = Some(self_id.clone());
						true
					},
					server::Event::StartBroadcast(_) => {
						true
					},
					_ =>
						false,
				}
			},
			None =>
				false,
		}
	});

	if let Some(received_self_id) = received_self_id {
		assert_eq!(self_id, received_self_id);
	}
}

#[test]
fn it_should_send_regular_heartbeats() {
	let     server = rc::Server::start();
	let mut client = mock::Client::start(server.port());

	client.send(client::Event::Login);

	client.wait_until(|event| *event == Some(server::Event::Heartbeat));
}

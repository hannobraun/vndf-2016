use shared::protocol::{
	client,
	server,
};
use test_suite::{
	mock,
	rc,
};


#[test]
fn it_should_ignore_clients_that_havent_logged_in() {
	// TODO: This test seems somewhat dubious. The second client might well
	//       receive its own broadcast and then stop listening before the first
	//       client's broadcast arrives.

	let     server   = rc::Server::start();
	let mut client_1 = mock::Client::start(server.port());
	let mut client_2 = mock::Client::start(server.port());

	client_1.send(client::Event::Privileged(client::event::Privileged::StartBroadcast(
		"I haven't logged in, but am sending this anyway.".to_string(),
	)));

	let message = "This is a broadcast.".to_string();
	client_2.send(client::Event::Public(client::event::Public::Login));
	client_2.send(client::Event::Privileged(client::event::Privileged::StartBroadcast(message.clone())));

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
	// TODO: It's quite hard to understand what's going on here. This test
	//       should be cleaned up and fixed, if necessary.

	let     server = rc::Server::start();
	let mut client = mock::Client::start(server.port());

	client.send(client::Event::Public(client::event::Public::Login));

	let mut ship_id = None;
	client.wait_until(|event| {
		if let &Some(server::Event::ShipId(id)) = event {
			ship_id = Some(id);
			true
		}
		else {
			false
		}
	});

	client.send(client::Event::Public(client::event::Public::Login));
	client.send(
		client::Event::Privileged(client::event::Privileged::StartBroadcast("This is a broadcast.".to_string()))
	);

	let mut received_ship_id = None;
	client.wait_until(|event| {
		match *event {
			Some(ref event) => {
				match *event {
					server::Event::ShipId(ship_id) => {
						received_ship_id = Some(ship_id);
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

	if let Some(_) = received_ship_id {
		assert_eq!(ship_id, received_ship_id);
	}
}

#[test]
fn it_should_send_regular_heartbeats() {
	let     server = rc::Server::start();
	let mut client = mock::Client::start(server.port());

	client.send(client::Event::Public(client::event::Public::Login));

	client.wait_until(|event| *event == Some(server::Event::Heartbeat));
}

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
	let     server   = rc::Server::start();
	let mut client_1 = mock::Client::start(server.port());
	let mut client_2 = mock::Client::start(server.port());

	client_1.send(client::Event::Privileged(client::event::Privileged::StartBroadcast(
		"I haven't logged in, but am sending this anyway.".to_string(),
	)));

	// The process of sending a broadcast is too complicated to happen by
	// accident for a client that never has logged in. The server crashing in
	// that case is more realistic, and that's what this test is about.
	// Let's make sure it still works by logging in with a second client.

	client_2.send(client::Event::Public(client::event::Public::Login));
	client_2.wait_until(|event| {
		match event {
			&Some(server::Event::Heartbeat) => true,
			_                               => false,
		}
	});
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

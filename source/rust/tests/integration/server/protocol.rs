use vndf::shared::protocol::client::{
	login,
	start_broadcast,
};
use vndf::shared::protocol::server::Event::{
	Heartbeat,
	ShipId,
};
use vndf::testing::{
	mock,
	rc,
};


#[test]
fn it_should_ignore_clients_that_havent_logged_in() {
	let     server   = rc::Server::start();
	let mut client_1 = mock::Client::start(server.port());
	let mut client_2 = mock::Client::start(server.port());

	client_1.send(start_broadcast(
		"I haven't logged in, but am sending this anyway.".to_string(),
	));

	// The process of sending a broadcast is too complicated to happen by
	// accident for a client that never has logged in. The server crashing in
	// that case is more realistic, and that's what this test is about.
	// Let's make sure it still works by logging in with a second client.

	client_2.send(login());
	client_2.wait_until(|event| {
		match event {
			&Some(Heartbeat(_)) => true,
			_                   => false,
		}
	});
}

#[test]
fn it_should_ignore_duplicate_logins() {
	let     server = rc::Server::start();
	let mut client = mock::Client::start(server.port());

	client.send(login());

	let mut first_ship_id = None;
	client.wait_until(|event| {
		if let &Some(ShipId(id)) = event {
			first_ship_id = Some(id);
			true
		}
		else {
			false
		}
	});

	client.send(login());
	client.send(start_broadcast("This is a broadcast.".to_string()));

	let mut second_ship_id = None;
	client.wait_until(|event| {
		match *event {
			Some(ref event) => {
				match *event {
					ShipId(ship_id) => {
						second_ship_id = Some(ship_id);
						true
					},
					Heartbeat(_) => {
						// If we received a heartbeat, that means the server
						// ignored our second login and has moved on.
						true
					}
					_ =>
						false,
				}
			},
			None =>
				false,
		}
	});

	// If we received a second ship id, it should be the same as the first one.
	// If we didn't receive one, that's also ok.
	if let Some(_) = second_ship_id {
		assert_eq!(first_ship_id, second_ship_id);
	}
}

#[test]
fn it_should_send_regular_heartbeats_with_current_game_time() {
	let     server = rc::Server::start();
	let mut client = mock::Client::start(server.port());

	client.send(login());

	let game_time_1_s = receive_heartbeat(&mut client);
	let game_time_2_s = receive_heartbeat(&mut client);

	assert!(game_time_1_s < game_time_2_s);


	fn receive_heartbeat(client: &mut mock::Client) -> f64 {
		let mut game_time_s = None;

		client.wait_until(|event|
			if let &Some(Heartbeat(time_s)) = event {
				game_time_s = Some(time_s);
				true
			}
			else {
				false
			}
		);

		game_time_s.unwrap()
	}
}

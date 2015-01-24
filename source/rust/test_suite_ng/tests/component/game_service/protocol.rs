use common::game::Broadcast;
use common::protocol::Percept;
use test_suite::{
	GameService,
	MockClient,
};


// TODO: Rewrite using raw sockets. The mock client shouldn't be expected to
//       have this low-level access.
#[test]
fn it_should_disconnect_clients_sending_invalid_data() {
	let invalid_utf8    = [0x80u8];
	let invalid_message = "This is an invalid message.";

	fn test(invalid_data: &[u8]) {
		let     game_service = GameService::start();
		let mut client_1     = MockClient::start(game_service.port());

		client_1.login(0);
		assert!(client_1.expect_perception().is_some());
		client_1.send_data(invalid_data);
		client_1.wait_until(|perception| perception.is_none()); // flush queue

		// We should no longer receive any perceptions.
		assert!(client_1.expect_perception().is_none());

		// But the game service shouldn't have crashed either.
		let mut client_2 = MockClient::start(game_service.port());
		client_2.login(0);
		assert!(client_2.expect_perception().is_some());
	}

	test(&invalid_utf8);
	test(invalid_message.as_bytes());
}

#[test]
fn it_should_ignore_clients_that_havent_logged_in() {
	let     game_service = GameService::start();
	let mut client_1     = MockClient::start(game_service.port());
	let mut client_2     = MockClient::start(game_service.port());

	client_1.broadcast(
		0,
		"I haven't logged in, but send this anyway.".to_string()
	);

	let message = "This is a broadcast.".to_string();
	client_2.login(0);
	client_2.broadcast(1, message.clone());

	client_2.wait_until(|perception|
		if let &Some(ref perception) = perception {
			let sender = perception.header.self_id.as_ref().unwrap().clone();
			perception.update_items().any(|&(ref id, ref entity)| {
				let percept = Percept::Broadcast(Broadcast {
					sender : sender.clone(),
					message: message.clone(),
				});

				id == &sender && entity == &percept
			})
		}
		else {
			false
		}
	);
}

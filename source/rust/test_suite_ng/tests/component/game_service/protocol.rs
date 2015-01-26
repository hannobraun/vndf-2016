use common::game::Broadcast;
use common::protocol::Percept;
use test_suite::{
	GameService,
	MockClient,
};


#[test]
fn it_should_ignore_clients_that_havent_logged_in() {
	let     game_service = GameService::start();
	let mut client_1     = MockClient::start(game_service.port());
	let mut client_2     = MockClient::start(game_service.port());

	client_1.broadcast(
		0,
		"I haven't logged in, but am sending this anyway.".to_string(),
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

#[test]
fn it_should_ignore_duplicate_logins() {
	let     game_service = GameService::start();
	let mut client       = MockClient::start(game_service.port());

	let mut self_id = None;

	client.login(0);
	client.wait_until(|perception| {
		match *perception {
			Some(ref perception) => {
				self_id = perception.header.self_id.clone();
				true
			},
			None =>
				false,
		}
	});

	// Log in a second time, expect to keep the same id.
	client.login(1);
	client.wait_until(|perception| {
		match *perception {
			Some(ref perception) => {
				assert_eq!(perception.header.self_id, self_id);
				true
			},
			None =>
				false,
		}
	});
}

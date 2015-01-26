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
	client_2.login();
	client_2.broadcast(1, message.clone());

	let perception =
		client_2.wait_until(|perception| {
			match *perception {
				Some(ref perception) => perception.update_items().count() == 1,
				None                 => false,
			}
		})
		.unwrap();

	let percept = Percept::Broadcast(Broadcast {
		sender : perception.header.self_id.as_ref().unwrap().clone(),
		message: message.clone(),
	});
	let percepts: Vec<Percept> = perception
		.update_items()
		.map(|&(_, ref percept)| percept.clone())
		.collect();
	assert_eq!(percepts[0], percept);
}

#[test]
fn it_should_ignore_duplicate_logins() {
	let     game_service = GameService::start();
	let mut client       = MockClient::start(game_service.port());

	let mut self_id = None;

	client.login();
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
	client.login();
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

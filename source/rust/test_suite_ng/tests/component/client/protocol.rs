use std::collections::HashSet;

use acpe::MAX_PACKET_SIZE;
use time::precise_time_s;

use protocol_ng::Step;
use test_tools_ng::{
	Client,
	MockGameService,
};


#[test]
fn it_should_resend_actions_until_confirmed() {
	let mut game_service = MockGameService::start();
	let     _client      = Client::start(game_service.port());

	// No need to send an explicit action, Login will do.
	game_service.expect_action().unwrap().ignore(); // first try
	game_service.expect_action().unwrap().confirm(); //second try

	// Assert either no action was sent or, if it was, that it was empty.
	match game_service.expect_action() {
		Some(action) =>
			assert!(action.inner.steps.len() == 0),
		None =>
			(),
	}
}

#[test]
fn it_should_keep_track_of_which_actions_are_confirmed() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	let broadcast = "This is a broadcast.".to_string();

	{
		let mut login = game_service.expect_action().unwrap();
		login.confirm();
		client.broadcast(broadcast.as_slice());
		game_service.wait_until(|action| {
			let ref action = action.as_ref().unwrap().inner;
			action.steps.contains(&Step::Broadcast(broadcast.clone()))
		});
		login.confirm();
	}

	// Broadcast has never been confirmed. It should keep sending it.
	let action = game_service.expect_action().unwrap().inner;
	assert!(action.steps.contains(&Step::Broadcast(broadcast)));
}

#[test]
fn it_should_distribute_large_payloads_over_multiple_packets() {
	let mut game_service = MockGameService::start();
	let mut client       = Client::start(game_service.port());

	// Create enough clients to overflow the maximum packet size.
	let     broadcast_text = "This is broadcast number";
	let mut broadcasts     = HashSet::new();
	for i in range(0, MAX_PACKET_SIZE / broadcast_text.len() + 1) {
		let broadcast = format!("{} {}", broadcast_text, i);
		client.broadcast(broadcast.as_slice());
		broadcasts.insert(broadcast);
	}

	let first_action = game_service.wait_until(|action| action.is_some());
	first_action.unwrap().confirm();

	let start_s = precise_time_s();
	while broadcasts.len() > 0 {
		if precise_time_s() - start_s > 0.5 {
			panic!("Not all actions arrived.");
		}

		match game_service.expect_action() {
			Some(mut action) => {
				action.confirm();
				for step in action.inner.steps.into_iter() {
					match step {
						Step::Broadcast(broadcast) => {
							broadcasts.remove(&broadcast);
						},
						_ =>
							()
					}
				}
			},
			None =>
				(),
		}
	}
}

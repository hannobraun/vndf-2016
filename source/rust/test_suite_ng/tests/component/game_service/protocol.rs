use std::collections::HashSet;

use time::precise_time_s;

use protocol_ng::MAX_PACKET_SIZE;
use test_tools_ng::{
	GameService,
	MockClient,
};


#[test]
fn it_should_confirm_received_actions() {
	let     game_service = GameService::start();
	let mut client       = MockClient::start(game_service.port());

	let seq = 512;
	client.login(seq);

	let perception = client.expect_perception().unwrap();
	assert_eq!(seq, perception.last_action);
}


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
fn it_should_distribute_large_payloads_over_multiple_packets() {
	// TODO: This test is really slow. It needs to be optimized.

	let     game_service = GameService::start();
	let mut client       = MockClient::start(game_service.port());

	client.login(0);

	// 512 bytes should be the maximum size for UDP packets. 512 clients all
	// sending broadcasts are more than enough to overflow this.
	let mut other_clients = Vec::new();
	let mut broadcasts    = HashSet::new();
	for i in range(0u16, 512) {
		let mut client    = MockClient::start(game_service.port());
		let     broadcast = format!("Broadcast from client {}", i);

		client.login(0);
		client.broadcast(1, broadcast.clone());

		other_clients.push(client);
		broadcasts.insert(broadcast);
	}

	// Receive perceptions until all broadcasts have been seen.
	let mut perceptions = Vec::new();
	let     start_s     = precise_time_s();
	while broadcasts.len() > 0 {
		if precise_time_s() - start_s > 5.0 {
			panic!("Not all broadcasts arrived.");
		}

		match client.expect_perception() {
			Some(perception) => {
				for received in perception.broadcasts.iter() {
					broadcasts.remove(received);
				}
				perceptions.push(perception);
			},

			None => (),
		}
	}

	for perception in perceptions.iter() {
		assert!(perception.encode().len() <= MAX_PACKET_SIZE);
	}
}

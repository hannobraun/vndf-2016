#![feature(collections_drain)]


mod args;
mod clients;
mod event_handler;


extern crate env_logger;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate nalgebra;
extern crate rand;
extern crate time;

extern crate common;
extern crate server;


use std::collections::HashMap;
use std::env;
use std::thread::sleep_ms;

use time::precise_time_s;

use args::Args;

use common::protocol::server::Event as ServerEvent;
use event_handler::EventHandler;
use server::network::Network;


fn main() {
	env_logger::init().unwrap_or_else(|e|
		panic!("Error initializing logger: {}", e)
	);

	let args = Args::parse(env::args());

	let mut broadcasts = HashMap::new();
	let mut clients    = HashMap::new();
	let mut network    = Network::new(args.port);

	info!("Listening on port {}\n", args.port);

	let mut event_handler   = EventHandler::new();
	let mut outgoing_events = Vec::new();

	loop {
		trace!("Start server main loop iteration");

		let now_s = precise_time_s();

		event_handler.receive(network.receive());
		event_handler.handle(
			now_s,
			&mut clients,
			&mut broadcasts,
			&mut outgoing_events,
			&mut network,
		);

		let mut to_remove = Vec::new();
		for (&address, client) in clients.iter() {
			if client.last_active_s + args.client_timeout_s < now_s {
				to_remove.push((
					address,
					client.last_active_s,
					now_s,
				));
			}
		}
		for (address, last_active_s, now_s) in to_remove.drain(..) {
			info!(
				"Removing {} (last active: {}, time of removal: {})\n",
				address, last_active_s, now_s,
			);
			broadcasts.remove(&address);
			if let Some(client) = clients.remove(&address) {
				outgoing_events.push(ServerEvent::StopBroadcast(client.id));
			}
		}

		for (_, client) in &mut clients {
			// TODO(E7GyYwQy): Take passed time since last iteration into
			//                 account.
			client.position = client.position + client.velocity;
		}

		let recipients = clients
			.iter()
			.map(|(&address, _)|
				address
			);

		for (_, broadcast) in broadcasts.iter() {
			outgoing_events.push(
				ServerEvent::StartBroadcast(broadcast.clone())
			);
		}
		outgoing_events.push(ServerEvent::Heartbeat);

		network.send(recipients, outgoing_events.as_ref());
		outgoing_events.clear();

		for (&address, client) in &clients {
			let event = ServerEvent::UpdateEntity(
				client.position,
				client.velocity,
			);
			network.send(
				Some(address).into_iter(),
				&[event],
			);
		}

		// TODO(1oL33ljB): While physics will generally need to happen on a
		//                 fixed interval, there's not really a reason to delay
		//                 other kinds of logic by sleeping. For example,
		//                 broadcasts can be handled immediately.
		sleep_ms(args.sleep_ms);
	}
}

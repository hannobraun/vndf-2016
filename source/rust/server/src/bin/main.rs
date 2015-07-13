#![feature(drain)]


mod args;
mod clients;
mod game;
mod incoming_events;


extern crate env_logger;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate nalgebra;
extern crate rand;
extern crate time;

extern crate server;
extern crate shared;


use std::collections::HashMap;
use std::env;
use std::thread::sleep_ms;

use time::precise_time_s;

use args::Args;

use game::state::GameState;
use incoming_events::IncomingEvents;
use server::network::Network;
use shared::protocol::server::Event as ServerEvent;


fn main() {
	env_logger::init().unwrap_or_else(|e|
		panic!("Error initializing logger: {}", e)
	);

	let args = Args::parse(env::args());

	let mut game_state = GameState::new();
	let mut clients    = HashMap::new();
	let mut network    = Network::new(args.port);

	info!("Listening on port {}", args.port);

	let mut incoming_events = IncomingEvents::new();
	let mut outgoing_events = Vec::new();

	loop {
		trace!("Start server main loop iteration");

		let now_s = precise_time_s();

		incoming_events.receive(network.receive());
		incoming_events.handle(
			now_s,
			&mut clients,
			&mut game_state,
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
				"Removing {} (last active: {}, time of removal: {})",
				address, last_active_s, now_s,
			);

			if let Some(client) = clients.remove(&address) {
				outgoing_events.push(ServerEvent::RemoveEntity(client.ship_id));
				game_state.on_leave(&client.ship_id);
			}
		}

		game_state.on_update(now_s);

		let recipients = clients
			.iter()
			.map(|(&address, _)|
				address
			);

		outgoing_events.push(ServerEvent::Heartbeat);

		network.send(recipients, outgoing_events.as_ref());
		outgoing_events.clear();

		// TODO(AMy58bbh): Handle this via outgoing_events
		for (&address, _) in &clients {
			for (id, entity) in game_state.export_entities() {
				let event = ServerEvent::UpdateEntity(id, entity);

				network.send(
					Some(address).into_iter(),
					&[event],
				);
			}
		}

		// TODO(1oL33ljB): While physics will generally need to happen on a
		//                 fixed interval, there's not really a reason to delay
		//                 other kinds of logic by sleeping. For example,
		//                 broadcasts can be handled immediately.
		sleep_ms(args.sleep_ms);
	}
}

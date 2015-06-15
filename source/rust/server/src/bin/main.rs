#![feature(collections_drain)]


mod args;


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

use nalgebra::{
	Rot2,
	Rotate,
	Vec1,
	Vec2,
};
use rand::random;
use time::precise_time_s;

use args::Args;

use common::game::Broadcast;
use common::protocol::client;
use common::protocol::server::Event as ServerEvent;
use server::network::Network;


#[derive(Debug)]
struct Client {
	pub id           : String,
	pub last_active_s: f64,
	pub position     : Vec2<f64>,
	pub velocity     : Vec2<f64>,
}


fn main() {
	env_logger::init().unwrap_or_else(|e|
		panic!("Error initializing logger: {}", e)
	);

	let args = Args::parse(env::args());

	let mut broadcasts = HashMap::new();
	let mut clients    = HashMap::new();
	let mut network    = Network::new(args.port);

	info!("Listening on port {}\n", args.port);

	let mut incoming_events = Vec::new();
	let mut outgoing_events = Vec::new();

	loop {
		trace!("Start server main loop iteration");

		for (address, event) in network.receive() {
			incoming_events.push((address, event));
		}

		for (address, event) in incoming_events.drain(..) {
			let now = precise_time_s();

			let log_message = format!(
				"Event: {:?} (address: {}; time: {})\n",
				event, address, now,
			);
			if event.is_important() {
				info!("{}", log_message);
			}
			else {
				debug!("{}", log_message);
			}

			match event {
				client::Event::Public(event) => {
					match event {
						client::event::Public::Login => {
							if clients.contains_key(&address) {
								debug!("Ignoring Login: {}\n", address);
							}
							else {
								let client = Client {
									id           : generate_id(),
									last_active_s: precise_time_s(),
									position     : Vec2::new(0.0, 0.0),
									velocity     : Vec2::new(1.0, 0.0),
								};

								let login = ServerEvent::SelfId(client.id.clone());
								network.send(
									Some(address).into_iter(),
									&[login],
								);

								clients.insert(address, client);
							}
						}
					}
				},

				client::Event::Privileged(event) => {
					let client = match clients.get_mut(&address) {
						Some(client) =>
							client,
						None => {
							debug!(
								"Ignoring event: {:?} ({})\n",
								event, address,
							);
							continue;
						},
					};

					client.last_active_s = now;

					match event {
						client::event::Privileged::Heartbeat => {
							// Nothing to do here, really, as the the time of
							// last activity for the client has already been
							// updated.
						},
						client::event::Privileged::StartBroadcast(message) => {
							let broadcast = Broadcast {
								sender : client.id.clone(),
								message: message,
							};

							broadcasts.insert(address, broadcast);
						},
						client::event::Privileged::StopBroadcast => {
							broadcasts.remove(&address);
							outgoing_events.push(
								ServerEvent::StopBroadcast(client.id.clone())
							);
						},
						client::event::Privileged::ScheduleManeuver(angle) => {
							let rotation = Rot2::new(Vec1::new(angle as f64));
							let new_velocity = rotation.rotate(&Vec2::new(1.0, 0.0));

							client.velocity = new_velocity;
						},
					}
				}
			}
		}

		let     now_s     = precise_time_s();
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


fn generate_id() -> String {
	fn random_char(min: char, max: char) -> char {
		let min = min as u8;
		let max = max as u8;

		((random::<u8>() % (max + 1 - min)) + min) as char
	}
	fn random_letter() -> char {
		random_char('A', 'Z')
	}
	fn random_letter_or_number() -> char {
		if random() {
			random_letter()
		}
		else {
			random_char('0', '9')
		}
	}

	let mut id = String::new();

	for _ in (0u8 .. 3) {
		id.push(random_letter());
	}
	id.push('-');
	for _ in (0u8 .. 5) {
		id.push(random_letter_or_number());
	}

	id
}

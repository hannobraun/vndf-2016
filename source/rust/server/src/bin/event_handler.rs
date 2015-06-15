use std::collections::HashMap;
use std::net::SocketAddr;

use nalgebra::{
	Rot2,
	Rotate,
	Vec1,
	Vec2,
};
use rand::random;

use clients::{
	Client,
	Clients,
};
use common::game::Broadcast;
use common::protocol::{
	client,
	server,
};
use server::network::Network;


pub struct EventHandler {
	incoming: Vec<(SocketAddr, client::Event)>,
}

impl EventHandler {
	pub fn new() -> EventHandler {
		EventHandler {
			incoming: Vec::new(),
		}
	}

	pub fn receive<E>(&mut self, events: E)
		where E: Iterator<Item = (SocketAddr, client::Event)>
	{
		for (address, event) in events {
			self.incoming.push((address, event));
		}
	}

	pub fn handle(
		&mut self,
		now_s     : f64,
		clients   : &mut Clients,
		broadcasts: &mut HashMap<SocketAddr, Broadcast>,
		outgoing  : &mut Vec<server::Event>,
		network   : &mut Network,
	) {
		for (address, event) in self.incoming.drain(..) {
			handle_event(
				now_s,
				address,
				event,
				clients,
				broadcasts,
				outgoing,
				network,
			);
		}
	}
}


fn handle_event(
	now_s     : f64,
	address   : SocketAddr,
	event     : client::Event,
	clients   : &mut Clients,
	broadcasts: &mut HashMap<SocketAddr, Broadcast>,
	outgoing  : &mut Vec<server::Event>,
	network   : &mut Network,
) {
	let log_message = format!(
		"Event: {:?} (address: {}; time: {})\n",
		event, address, now_s,
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
							last_active_s: now_s,
							position     : Vec2::new(0.0, 0.0),
							velocity     : Vec2::new(1.0, 0.0),
						};

						let login = server::Event::SelfId(client.id.clone());
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
					return;
				},
			};

			client.last_active_s = now_s;

			match event {
				client::event::Privileged::Heartbeat => {
					// Nothing to do here, really, as the the time of
					// last activity for the client has already been
					// updated.
				},
				client::event::Privileged::StartBroadcast(message) => {
					broadcasts.insert(
						address,
						Broadcast {
							sender : client.id.clone(),
							message: message,
						}
					);
				},
				client::event::Privileged::StopBroadcast => {
					broadcasts.remove(&address);
					outgoing.push(
						server::Event::StopBroadcast(client.id.clone())
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

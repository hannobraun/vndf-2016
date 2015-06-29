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
use game_state::GameState;
use server::network::Network;
use shared::game::{
	Broadcast,
	Ship,
};
use shared::protocol::{
	client,
	server,
};


pub struct IncomingEvents {
	incoming: Vec<(SocketAddr, client::Event)>,
}

impl IncomingEvents {
	pub fn new() -> IncomingEvents {
		IncomingEvents {
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
		game_state: &mut GameState,
		outgoing  : &mut Vec<server::Event>,
		network   : &mut Network,
	) {
		for (address, event) in self.incoming.drain(..) {
			handle_event(
				now_s,
				address,
				event,
				clients,
				game_state,
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
	game_state: &mut GameState,
	outgoing  : &mut Vec<server::Event>,
	network   : &mut Network,
) {
	let log_message = format!(
		"Event: {:?} (address: {}; time: {})",
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
			handle_public_event(
				now_s,
				address,
				event,
				clients,
				game_state,
				network,
			);
		},

		client::Event::Privileged(event) => {
			let client = match clients.get_mut(&address) {
				Some(client) =>
					client,
				None => {
					debug!(
						"Ignoring event: {:?} ({})",
						event, address,
					);
					return;
				},
			};

			handle_privileged_event(
				now_s,
				address,
				event,
				client,
				game_state,
				outgoing,
			);
		},
	}
}

fn handle_public_event(
	now_s     : f64,
	address   : SocketAddr,
	event     : client::event::Public,
	clients   : &mut Clients,
	game_state: &mut GameState,
	network   : &mut Network,
) {
	match event {
		client::event::Public::Login => {
			if clients.contains_key(&address) {
				debug!("Ignoring Login: {}", address);
			}
			else {
				let client = Client {
					id           : generate_id(),
					last_active_s: now_s,
				};

				game_state.create_ship(address, Ship {
					position: Vec2::new(0.0, 0.0),
					velocity: Vec2::new(1.0, 0.0),
				});

				// TODO(AMy58bbh): This needs to be an outgoing event.
				//                 Currently, this won't work, as outgoing
				//                 events are broadcast to all clients, while
				//                 this event is only for a specific client.
				let login = server::Event::SelfId(client.id.clone());
				network.send(
					Some(address).into_iter(),
					&[login],
				);

				clients.insert(address, client);
			}
		}
	}
}

fn handle_privileged_event(
	now_s     : f64,
	address   : SocketAddr,
	event     : client::event::Privileged,
	client    : &mut Client,
	game_state: &mut GameState,
	outgoing  : &mut Vec<server::Event>,
) {
	client.last_active_s = now_s;

	match event {
		client::event::Privileged::Heartbeat => {
			// Nothing to do here, really, as the the time of
			// last activity for the client has already been
			// updated.
		},
		client::event::Privileged::StartBroadcast(message) => {
			game_state.add_broadcast(
				address,
				Broadcast {
					sender : client.id.clone(),
					message: message,
				}
			);
		},
		client::event::Privileged::StopBroadcast => {
			game_state.destroy_broadcast(&address);
			outgoing.push(
				server::Event::StopBroadcast(client.id.clone())
			);
		},
		client::event::Privileged::ScheduleManeuver(angle) => {
			let rotation = Rot2::new(Vec1::new(angle as f64));
			let new_velocity = rotation.rotate(&Vec2::new(1.0, 0.0));

			game_state.ship(&address).velocity = new_velocity;
		},
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

use std::net::SocketAddr;

use nalgebra::{
	Rot2,
	Rotate,
	Vec1,
	Vec2,
};

use clients::{
	Client,
	Clients,
};
use game::state::GameState;
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
		network   : &mut Network,
	) {
		for (address, event) in self.incoming.drain(..) {
			handle_event(
				now_s,
				address,
				event,
				clients,
				game_state,
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
						"Ignoring event from unknown client: {:?} ({})",
						event, address,
					);
					return;
				},
			};

			handle_privileged_event(
				now_s,
				event,
				client,
				game_state,
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
				debug!("Ignoring duplicate login: {}", address);
			}
			else {
				let ship_id = game_state.entities.new_entity(|entity|
					entity.add_ship(Ship {
						position: Vec2::new(0.0, 0.0),
						velocity: Vec2::new(1.0, 0.0),
					})
				);

				let client = Client {
					ship_id      : ship_id,
					last_active_s: now_s,
				};

				// TODO(AMy58bbh): This needs to be an outgoing event.
				//                 Currently, this won't work, as outgoing
				//                 events are broadcast to all clients, while
				//                 this event is only for a specific client.
				let login = server::Event::ShipId(client.ship_id);
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
	event     : client::event::Privileged,
	client    : &mut Client,
	game_state: &mut GameState,
) {
	client.last_active_s = now_s;

	match event {
		client::event::Privileged::Heartbeat => {
			// Nothing to do here, really, as the the time of
			// last activity for the client has already been
			// updated.
		},
		client::event::Privileged::StartBroadcast(message) => {
			game_state.entities.get_entity(client.ship_id, |entity|
				entity.add_broadcast(Broadcast {
					sender : client.ship_id,
					message: message,
				})
			);
		},
		client::event::Privileged::StopBroadcast => {
			game_state.entities.remove_broadcast(&client.ship_id);
		},
		client::event::Privileged::ScheduleManeuver(_, angle) => {
			let rotation = Rot2::new(Vec1::new(angle));
			let new_velocity = rotation.rotate(&Vec2::new(1.0, 0.0));

			game_state.entities.ship(&client.ship_id).velocity = new_velocity;
		},
	}
}

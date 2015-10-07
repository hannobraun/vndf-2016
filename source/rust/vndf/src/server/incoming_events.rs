use std::net::SocketAddr;

use server::clients::{
	Client,
	Clients,
};
use server::game::events;
use server::game::state::GameState;
use server::outgoing_events::{
	OutgoingEvents,
	Recipients,
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
		now_s          : f64,
		clients        : &mut Clients,
		game_state     : &mut GameState,
		outgoing_events: &mut OutgoingEvents,
	) {
		for (address, event) in self.incoming.drain(..) {
			handle_event(
				now_s,
				address,
				event,
				clients,
				game_state,
				outgoing_events,
			);
		}
	}
}


fn handle_event(
	now_s          : f64,
	address        : SocketAddr,
	event          : client::Event,
	clients        : &mut Clients,
	game_state     : &mut GameState,
	outgoing_events: &mut OutgoingEvents,
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
				outgoing_events,
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
	now_s          : f64,
	address        : SocketAddr,
	event          : client::event::Public,
	clients        : &mut Clients,
	game_state     : &mut GameState,
	outgoing_events: &mut OutgoingEvents,
) {
	match event {
		client::event::Public::Login => {
			// TODO: Move parts of this code into Client, as Client::login.
			if clients.clients.contains_key(&address) {
				debug!("Ignoring duplicate login: {}", address);
			}
			else {
				let ship_id = game_state.handle_event(events::Enter);

				let client = Client {
					ship_id      : ship_id,
					last_active_s: now_s,
				};

				outgoing_events.push(
					server::Event::ShipId(client.ship_id),
					Recipients::One(address),
				);

				clients.clients.insert(address, client);
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
			game_state.on_start_broadcast(client.ship_id, message)
		},
		client::event::Privileged::StopBroadcast => {
			game_state.on_stop_broadcast(client.ship_id)
		},
		client::event::Privileged::ScheduleManeuver(data) => {
			game_state.on_schedule_maneuver(client.ship_id, data)
		},
		client::event::Privileged::CancelManeuver(maneuver_id) => {
			game_state.on_cancel_maneuver(client.ship_id, maneuver_id)
		},
	}
}

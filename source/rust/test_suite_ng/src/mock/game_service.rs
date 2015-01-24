use std::io::net::ip::{
	Port,
	SocketAddr,
};
use time::precise_time_s;

use acceptance::random_port;
use acpe::protocol::{
	ActionHeader,
	Message,
};

use common::protocol::{
	Action,
	ClientEvent,
	Percept,
	ServerEvent,
	Step,
};
use game_service::network::Network;


pub struct GameService {
	port    : Port,
	network : Network,
	incoming: Vec<(SocketAddr, ClientEvent)>,
}

impl GameService {
	pub fn start() -> GameService {
		let port    = random_port(40000, 50000);
		let network = Network::new(port);

		GameService {
			port    : port,
			network : network,
			incoming: Vec::new(),
		}
	}

	pub fn port(&self) -> Port {
		self.port
	}

	pub fn send_perception(
		&mut self,
		address: SocketAddr,
		update : Vec<(String, Percept)>,
	) {
		let events = update
			.into_iter()
			.map(|(_, percept)| {
				let broadcast = match percept {
					Percept::Broadcast(broadcast) => broadcast,
				};

				ServerEvent::StartBroadcast(broadcast)
			});

		self.network.send(Some(address).into_iter(), events);
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn expect_action(&mut self) -> Option<Action> {
		let start_s = precise_time_s();

		while self.incoming.len() == 0 && precise_time_s() - start_s < 0.5 {
			self.incoming.extend(self.network.receive());
		}

		self.incoming = self.incoming
			.drain()
			.filter(|&(_, ref event)|
				event != &ClientEvent::Heartbeat
			)
			.collect();

		if self.incoming.len() > 0 {
			let (address, event) = self.incoming.remove(0);

			// This makes sure that confirmations are sent back to the client.
			// TODO: Remove
			self.network.send(
				Some(address).into_iter(),
				Some(ServerEvent::SelfId("".to_string())).into_iter(),
			);
			self.network.update();

			let step = match event {
				ClientEvent::Login =>
					Step::Login,
				ClientEvent::Heartbeat =>
					panic!("Unexpected event: Heartbeat"),
				ClientEvent::StartBroadcast(broadcast) =>
					Step::Broadcast(broadcast),
				ClientEvent::StopBroadcast =>
					Step::StopBroadcast,
			};

			let mut action = Message::new(ActionHeader { id: 0 });
			action.add_update(0, step);

			Some(action)
		}
		else {
			None
		}
	}

	// TODO(85118666): Make generic and move into a trait called Mock.
	pub fn wait_until<F>(&mut self, condition: F) -> Option<Action>
		where F: Fn(&mut Option<Action>) -> bool
	{
		let start_s = precise_time_s();

		let mut action = self.expect_action();

		while !condition(&mut action) {
			if precise_time_s() - start_s > 0.5 {
				panic!("Condition not satisfied after waiting");
			}

			action = self.expect_action();
		}

		action
	}
}

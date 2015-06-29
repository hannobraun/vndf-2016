use std::collections::HashMap;
use std::collections::hash_map::{
	IterMut,
	Values,
};
use std::net::SocketAddr;

use shared::game::{
	Broadcast,
	Ship,
};


#[derive(Debug)]
pub struct GameState {
	broadcasts: HashMap<SocketAddr, Broadcast>,
	ships     : HashMap<SocketAddr, Ship>,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			broadcasts: HashMap::new(),
			ships     : HashMap::new(),
		}
	}

	pub fn create_ship(&mut self, id: SocketAddr, ship: Ship) {
		self.ships.insert(id, ship);
	}

	pub fn ship(&mut self, id: &SocketAddr) -> &mut Ship {
		self.ships.get_mut(id)
			.unwrap_or_else(|| panic!("Ship not found: {}", id))
	}

	pub fn ships(&mut self) -> IterMut<SocketAddr, Ship> {
		self.ships.iter_mut()
	}

	pub fn add_broadcast(&mut self, id: SocketAddr, broadcast: Broadcast) {
		self.broadcasts.insert(id, broadcast);
	}

	pub fn destroy_broadcast(&mut self, id: &SocketAddr) {
		self.broadcasts.remove(id);
	}

	pub fn broadcasts(&self) -> Values<SocketAddr, Broadcast> {
		self.broadcasts.values()
	}

	pub fn update(&mut self) {
		for (_, ship) in &mut self.ships {
			// TODO(E7GyYwQy): Take passed time since last iteration into
			//                 account.
			ship.position = ship.position + ship.velocity;
		}
	}
}

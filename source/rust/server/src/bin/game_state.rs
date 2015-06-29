use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::net::SocketAddr;

use shared::game::{
	Broadcast,
	Ship,
};


pub type EntityId = SocketAddr;


#[derive(Debug)]
pub struct GameState {
	broadcasts: HashMap<SocketAddr, Broadcast>,
	ships     : HashMap<EntityId, Ship>,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			broadcasts: HashMap::new(),
			ships     : HashMap::new(),
		}
	}

	pub fn create_ship(&mut self, id: EntityId, ship: Ship) {
		self.ships.insert(id, ship);
	}

	pub fn ship(&mut self, id: &EntityId) -> &mut Ship {
		self.ships.get_mut(id)
			.unwrap_or_else(|| panic!("Ship not found: {}", id))
	}

	pub fn add_broadcast(&mut self, id: SocketAddr, broadcast: Broadcast) {
		self.broadcasts.insert(id, broadcast);
	}

	pub fn remove_broadcast(&mut self, id: &SocketAddr) {
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

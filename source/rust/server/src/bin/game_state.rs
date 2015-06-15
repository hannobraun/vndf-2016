use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::net::SocketAddr;

use shared::game::Broadcast;


pub struct GameState {
	pub broadcasts: HashMap<SocketAddr, Broadcast>,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			broadcasts: HashMap::new(),
		}
	}

	pub fn create_broadcast(&mut self, id: SocketAddr, broadcast: Broadcast) {
		self.broadcasts.insert(id, broadcast);
	}

	pub fn destroy_broadcast(&mut self, id: &SocketAddr) {
		self.broadcasts.remove(id);
	}

	pub fn broadcasts(&mut self) -> Values<SocketAddr, Broadcast> {
		self.broadcasts.values()
	}
}

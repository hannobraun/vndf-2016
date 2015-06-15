use std::collections::HashMap;
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
}

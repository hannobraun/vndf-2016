use std::collections::HashMap;
use std::net::SocketAddr;

use shared::game::Broadcast;


pub struct GameState {
	pub broadcasts: Broadcasts,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			broadcasts: HashMap::new(),
		}
	}
}


pub type Broadcasts = HashMap<SocketAddr, Broadcast>;

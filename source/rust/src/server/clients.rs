use std::collections::HashMap;
use std::net::SocketAddr;

use shared::game::EntityId;


pub struct Clients {
	pub clients: HashMap<SocketAddr, Client>,
}

impl Clients {
	pub fn new() -> Clients {
		Clients {
			clients: HashMap::new(),
		}
	}
}


#[derive(Debug)]
pub struct Client {
	pub ship_id      : EntityId,
	pub last_active_s: f64,
}

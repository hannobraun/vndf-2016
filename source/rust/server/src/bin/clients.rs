use std::collections::HashMap;
use std::net::SocketAddr;

use shared::game::EntityId;


pub type Clients = HashMap<SocketAddr, Client>;


#[derive(Debug)]
pub struct Client {
	pub id           : String,
	pub ship_id      : EntityId,
	pub last_active_s: f64,
}

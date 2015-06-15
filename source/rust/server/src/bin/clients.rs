use std::collections::HashMap;
use std::net::SocketAddr;

use shared::game::Ship;


pub type Clients = HashMap<SocketAddr, Client>;


#[derive(Debug)]
pub struct Client {
	pub id           : String,
	pub last_active_s: f64,
	// TODO(AMy58bbh): Move ship out of here.
	pub ship         : Ship,
}

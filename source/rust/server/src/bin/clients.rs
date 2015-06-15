use std::collections::HashMap;
use std::net::SocketAddr;


pub type Clients = HashMap<SocketAddr, Client>;


#[derive(Debug)]
pub struct Client {
	pub id           : String,
	pub last_active_s: f64,
}

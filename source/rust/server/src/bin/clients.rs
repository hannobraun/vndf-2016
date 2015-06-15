use std::collections::HashMap;
use std::net::SocketAddr;

use nalgebra::Vec2;


pub type Clients = HashMap<SocketAddr, Client>;


#[derive(Debug)]
pub struct Client {
	pub id           : String,
	pub last_active_s: f64,
	pub position     : Vec2<f64>,
	pub velocity     : Vec2<f64>,
}

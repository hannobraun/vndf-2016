use std::collections::HashMap;
use std::io::net::ip::SocketAddr;

use acpe::protocol::Seq;


pub use self::receiver::Receiver;
pub use self::sender::Sender;


mod receiver;
mod sender;


pub type Clients = HashMap<SocketAddr, Client>;


pub struct Client {
	pub id           : String,
	pub last_action  : Seq,
	pub last_active_s: f64,
	pub broadcast    : Option<String>,
}
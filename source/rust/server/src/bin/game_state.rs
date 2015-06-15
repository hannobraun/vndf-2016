use std::collections::HashMap;
use std::net::SocketAddr;

use shared::game::Broadcast;


pub type Broadcasts = HashMap<SocketAddr, Broadcast>;

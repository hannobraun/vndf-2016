use std::collections::HashMap;
use std::net::SocketAddr;

use common::game::Broadcast;


pub type Broadcasts = HashMap<SocketAddr, Broadcast>;

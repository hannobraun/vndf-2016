use acpe::protocol;
use rustc_serialize::Encodable;

use game::Broadcast;


// TODO: Remove Action and Perception
pub type Action     = protocol::Action<u64, Step>;
pub type Perception = protocol::Perception<String, Percept>;


#[derive(RustcDecodable, RustcEncodable, Show)]
pub enum ClientEvent {
	Login,
	Heartbeat,
	StartBroadcast(String),
	StopBroadcast,
}


#[derive(RustcDecodable, RustcEncodable, Show)]
pub enum ServerEvent {
	SelfId(String),
	StartBroadcast(Broadcast),
	StopBroadcast(String),
}


// TODO: Remove
#[derive(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Step {
	Login,
	Broadcast(String),
	StopBroadcast,
}


// TODO: Remove
#[derive(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Percept {
	Broadcast(Broadcast),
}

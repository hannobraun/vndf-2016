use acpe::protocol;
use rustc_serialize::Encodable;


// TODO: Remove Action and Perception
pub type Action     = protocol::Action<u64, Step>;
pub type Perception = protocol::Perception<String, Percept>;


pub enum ClientEvent {
	Login,
	Heartbeat,
	// TODO: Rename to StartBroadcast?
	Broadcast(String),
	StopBroadcast,
}


pub enum ServerEvent {
	SelfId(String),
	// TODO: Rename to StartBroadcast?
	Broadcast(Broadcast),
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


// TODO: Move to another module, something like common::game
#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}

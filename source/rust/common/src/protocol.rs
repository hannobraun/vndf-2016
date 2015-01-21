use acpe::protocol;
use rustc_serialize::Encodable;


pub type Action     = protocol::Action<u64, Step>;
pub type Perception = protocol::Perception<String, Percept>;


pub enum ClientEvent {
	Login,
	Heartbeat,
	Broadcast(String),
	StopBroadcast,
}


#[derive(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Step {
	Login,
	Broadcast(String),
	StopBroadcast,
}


#[derive(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Percept {
	Broadcast(Broadcast),
}


#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}

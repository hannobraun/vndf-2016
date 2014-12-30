use acpe::protocol;
use rustc_serialize::Encodable;


pub type Action     = protocol::Action<String, Step>;
pub type Perception = protocol::Perception<String, Percept>;


#[deriving(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Step {
	Login,
	Broadcast(String),
	StopBroadcast,
}


#[deriving(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Percept {
	Broadcast(Broadcast),
}


#[deriving(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}

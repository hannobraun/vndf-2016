use rustc_serialize::Encodable;

use game::Broadcast;


#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum ClientEvent {
	Login,
	Heartbeat,
	StartBroadcast(String),
	StopBroadcast,
}


#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum ServerEvent {
	SelfId(String),
	StartBroadcast(Broadcast),
	StopBroadcast(String),
}

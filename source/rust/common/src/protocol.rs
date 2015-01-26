use rustc_serialize::Encodable;

use game::Broadcast;


#[derive(Eq, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum ClientEvent {
	Login,
	Heartbeat,
	StartBroadcast(String),
	StopBroadcast,
}


#[derive(Eq, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum ServerEvent {
	SelfId(String),
	StartBroadcast(Broadcast),
	StopBroadcast(String),
}

use nalgebra::Vec2;
use rustc_serialize::Encodable;

use game::Broadcast;


#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum ClientEvent {
	Login,
	Heartbeat,
	StartBroadcast(String),
	StopBroadcast,
}


#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum ServerEvent {
	Heartbeat,
	SelfId(String),
	StartBroadcast(Broadcast),
	StopBroadcast(String),
	UpdateEntity(Vec2<f64>, Vec2<f64>),
}

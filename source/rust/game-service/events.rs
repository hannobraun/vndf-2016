use std::io::IoError;

use common::physics::{
	Radians,
	Vec2
};
use common::protocol::{
	Action,
	Perception
};

use network::ClientId;


#[deriving(PartialEq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(ClientId),
	Leave(ClientId),
	Action(ClientId, Action),
	MissileLaunch(Vec2, Radians)
}

#[deriving(PartialEq, Show)]
pub enum NetworkEvent {
	Message(Vec<ClientId>, Perception),
	Close(ClientId, IoError)
}

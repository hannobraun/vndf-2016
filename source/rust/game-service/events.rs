use std::io::IoError;

use common::physics::{
	Radians,
	Vec2
};
use common::protocol::{
	Action,
	Perception
};

use network::ConnId;


#[deriving(PartialEq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(ConnId),
	Leave(ConnId),
	Action(ConnId, Action),
	MissileLaunch(Vec2, Radians)
}

#[deriving(PartialEq, Show)]
pub enum NetworkEvent {
	Message(Vec<ConnId>, Perception),
	Close(ConnId, IoError)
}

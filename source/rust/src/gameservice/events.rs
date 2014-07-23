use std::io::IoError;

use rustecs::EntityId;

use common::ecs::SharedWorldEntity;
use common::net::ConnId;
use common::physics::{
	Radians,
	Vec2
};
use common::protocol::{
	Action,
	Perception
};


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
	Message(Vec<ConnId>, Perception<EntityId, SharedWorldEntity>),
	Close(ConnId, IoError)
}

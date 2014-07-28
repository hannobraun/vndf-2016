use std::io::IoError;

use common::ecs::SharedWorldEntity;
use common::protocol::{
	Action,
	Perception
};
use net::ConnId;
use physics::{
	Radians,
	Vec2
};
use rustecs::EntityId;


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

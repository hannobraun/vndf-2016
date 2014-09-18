use std::io::IoError;

use cgmath::Vector2;

use game::ecs::SharedWorldEntity;
use net::ConnId;
use physics::Radians;
use protocol::{
	Action,
	Perception
};
use rustecs::EntityId;


#[deriving(PartialEq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(ConnId),
	Leave(ConnId),
	Action(ConnId, Action),
	MissileLaunch(Vector2<f64>, Radians)
}

#[deriving(PartialEq, Show)]
pub enum NetworkEvent {
	Message(Vec<ConnId>, Perception<EntityId, SharedWorldEntity>),
	Close(ConnId, IoError)
}

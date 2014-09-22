use std::io::IoError;

use cgmath::{
	Quaternion,
	Vector3,
};

use game::ecs::SharedWorldEntity;
use net::ConnId;
use protocol;
use rustecs::EntityId;


#[deriving(PartialEq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(ConnId),
	Leave(ConnId),
	Action(ConnId, protocol::Action),
	MissileLaunch(Vector3<f64>, Quaternion<f64>)
}

#[deriving(PartialEq, Show)]
pub enum NetworkEvent {
	Message(Vec<ConnId>, protocol::Perception<EntityId, SharedWorldEntity>),
	Close(ConnId, IoError)
}

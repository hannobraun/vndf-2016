use std::io::IoError;

use cgmath::{
	Rad,
	Vector2,
};

use game::ecs::SharedWorldEntity;
use net::ConnId;
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
	MissileLaunch(Vector2<f64>, Rad<f64>)
}

#[deriving(PartialEq, Show)]
pub enum NetworkEvent {
	Message(Vec<ConnId>, Perception<EntityId, SharedWorldEntity>),
	Close(ConnId, IoError)
}

use std::io::IoError;

use common::ecs::EntityId;
use common::physics::{
	Radians,
	Vec2
};
use common::protocol::{
	Action,
	Perception
};

use network::ClientId;


#[deriving(Eq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(ClientId),
	Leave(ClientId),
	Action(ClientId, Action),
	MissileLaunch(EntityId, Vec2, Radians)
}

#[deriving(Eq, Show)]
pub enum NetworkEvent {
	Message(Vec<ClientId>, Perception),
	Close(ClientId, IoError)
}

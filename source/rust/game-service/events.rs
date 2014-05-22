use common::protocol::Action;

use network::ClientId;


#[deriving(Eq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(ClientId),
	Leave(ClientId),
	Action(ClientId, Action)
}

#[deriving(Eq, Show)]
pub enum NetworkEvent {
	Close(ClientId)
}

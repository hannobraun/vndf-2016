use common::protocol::Action;

#[deriving(Eq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(uint),
	Leave(uint),
	Action(uint, Action)
}

#[deriving(Eq, Show)]
pub enum NetworkEvent {
	Close(uint)
}

use libc::c_int;

use common::protocol::Action;


#[deriving(Eq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(uint),
	Leave(uint),
	Action(c_int, Action)
}

#[deriving(Eq, Show)]
pub enum NetworkEvent {
	Close(uint)
}

use libc::c_int;

use common::net::Connection;
use common::physics::Radians;


#[deriving(Eq, Show)]
pub enum GameEvent {
	Init,
	Update(f64),
	Enter(Connection),
	Leave(uint),
	Action(c_int, Radians)
}

#[deriving(Eq, Show)]
pub enum NetworkEvent {
	Close(uint)
}

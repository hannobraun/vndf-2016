use libc::c_int;

use common::net::Connection;
use common::physics::Radians;


#[deriving(Eq, Show)]
pub enum GameEvent {
	Connect(Connection),
	Disconnect(uint),
	DataReceived(c_int),
	CommandEvent(c_int, Radians),
	Init,
	Update(f64)
}

#[deriving(Eq, Show)]
pub enum NetworkEvent {
	Close(uint)
}

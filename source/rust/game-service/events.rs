use libc::c_int;

use common::net::Connection;
use common::physics::Radians;


#[deriving(Eq, Show)]
pub enum GameEvent {
	Connect(Connection),
	Disconnect(uint),
	DataReceived(c_int),
	CreateEvent(uint),
	CommandEvent(c_int, Radians),
	Update(f64)
}

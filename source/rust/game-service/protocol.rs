use std::libc;

use net;

pub fn send_remove(clientFD: libc::c_int, id: uint) -> libc::c_int {
	net::send_message(
		clientFD,
		format!("REMOVE {:u}", id))
}

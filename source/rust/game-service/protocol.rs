use std::libc;

use net;


pub fn send_update(
	clientFD: libc::c_int,
	id      : uint,
	xPos    : f64,
	yPos    : f64,
	zPos    : f64) -> libc::c_int {

	net::send_message(
		clientFD,
		format!(
			"UPDATE id: {:u}, pos: ({:f}, {:f} {:f})",
			id, xPos, yPos, zPos))
}

pub fn send_remove(clientFD: libc::c_int, id: uint) -> libc::c_int {
	net::send_message(
		clientFD,
		format!("REMOVE id: {:u}", id))
}

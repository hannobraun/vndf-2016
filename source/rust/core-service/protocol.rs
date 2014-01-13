use std::libc;


pub fn send_update(
	clientFD: libc::c_int,
	id      : libc::size_t,
	xPos    : libc::c_double,
	yPos    : libc::c_double) -> libc::c_int {

	::net::send_message(clientFD,
		format!("UPDATE id: {:u}, pos: ({:f}, {:f})", id, xPos, yPos))
}

pub fn send_remove(clientFD: libc::c_int, id: libc::size_t) -> libc::c_int {
	::net::send_message(clientFD, format!("REMOVE id: {:u}", id))
}

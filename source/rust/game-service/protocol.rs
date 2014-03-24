use std::libc;

use common::protocol::Update;

use net;


pub fn send_update(clientFD: libc::c_int, message: Update) -> libc::c_int {
	net::send_message(clientFD, message.to_str())
}

pub fn send_remove(clientFD: libc::c_int, id: uint) -> libc::c_int {
	net::send_message(
		clientFD,
		format!("REMOVE {:u}", id))
}

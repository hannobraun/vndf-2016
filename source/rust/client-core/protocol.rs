use libc;
use std::ptr;
use std::str;

use common::protocol::{Create, Message, Remove, SelfInfo, Update};

use net;


static BUFFER_SIZE : libc::c_int = 256;


pub struct Connection {
	socket_fd : libc::c_int,
	buffer    : [i8, ..BUFFER_SIZE],
	buffer_pos: uint
}

pub trait Handler {
	fn set_self_id(&self, message: SelfInfo);
	fn create_ship(&self, message: Create);
	fn update_ship(&self, message: Update);
	fn remove_ship(&self, message: Remove);
}


pub fn init(socket_fd: libc::c_int) -> Connection {
	Connection {
		socket_fd : socket_fd,
		buffer    : [0, ..BUFFER_SIZE],
		buffer_pos: 0 }
}

pub fn receive_positions(
	connection: &mut Connection,
	handler   : &mut Handler) {

	let bytes_received = net::receive(
		connection.socket_fd,
		connection.buffer.slice_from(connection.buffer_pos));

	connection.buffer_pos += bytes_received as uint;

	while connection.buffer_pos > 0 && connection.buffer[0] as uint <= connection.buffer_pos {
		let message_size = connection.buffer[0];
		assert!(message_size >= 0);

		let message = unsafe {
			str::raw::from_buf_len(
				(connection.buffer.as_ptr() as *u8).offset(1),
				(message_size - 1) as uint)
		};

		match Message::from_str(message) {
			SelfInfo(self_info) => handler.set_self_id(self_info),
			Create(create)      => handler.create_ship(create),
			Update(update)      => handler.update_ship(update),
			Remove(remove)      => handler.remove_ship(remove),

			_ =>
				fail!("invalid message ({})\n", message)
		}

		unsafe {
			ptr::copy_memory(
				connection.buffer.as_mut_ptr(),
				connection.buffer.as_ptr().offset(message_size as int),
				(BUFFER_SIZE - message_size as i32) as uint);
			connection.buffer_pos -= message_size as uint;
		}
	}
}

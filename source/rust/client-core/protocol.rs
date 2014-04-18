use libc;
use std::ptr;
use std::str;

use common::net::Connection;
use common::protocol::{Create, Message, Remove, SelfInfo, Update};

use net;


static BUFFER_SIZE : libc::c_int = 256;


pub struct Protocol {
	connection: Connection,
	buffer    : [u8, ..BUFFER_SIZE],
	buffer_pos: uint
}

pub trait Handler {
	fn set_self_id(&self, message: SelfInfo);
	fn create_ship(&self, message: Create);
	fn update_ship(&self, message: Update);
	fn remove_ship(&self, message: Remove);
}


pub fn init(connection: Connection) -> Protocol {
	Protocol {
		connection: connection,
		buffer    : [0, ..BUFFER_SIZE],
		buffer_pos: 0 }
}

pub fn receive_positions(protocol: &mut Protocol, handler : &mut Handler) {
	let bytes_received = net::receive(
		protocol.connection.fd,
		protocol.buffer.slice_from(protocol.buffer_pos));

	protocol.buffer_pos += bytes_received as uint;

	while protocol.buffer_pos > 0 && protocol.buffer[0] as uint <= protocol.buffer_pos {
		let message_size = protocol.buffer[0];

		let message = unsafe {
			str::raw::from_buf_len(
				(protocol.buffer.as_ptr() as *u8).offset(1),
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
				protocol.buffer.as_mut_ptr(),
				protocol.buffer.as_ptr().offset(message_size as int),
				(BUFFER_SIZE - message_size as i32) as uint);
			protocol.buffer_pos -= message_size as uint;
		}
	}
}

use std::ptr;
use std::str;

use common::net::Connection;
use common::protocol::{Create, Message, Remove, SelfInfo, Update};


pub struct Protocol {
	connection: Connection
}

pub trait Handler {
	fn set_self_id(&self, message: SelfInfo);
	fn create_ship(&self, message: Create);
	fn update_ship(&self, message: Update);
	fn remove_ship(&self, message: Remove);
}


pub fn init(connection: Connection) -> Protocol {
	Protocol {
		connection: connection
	}
}

pub fn receive_positions(protocol: &mut Protocol, handler : &mut Handler) {
	let bytes_received = protocol.connection.receive(
		protocol.connection.in_buffer.slice_from(protocol.connection.in_buffer_pos));

	protocol.connection.in_buffer_pos += bytes_received as uint;

	while protocol.connection.in_buffer_pos > 0 && protocol.connection.in_buffer[0] as uint <= protocol.connection.in_buffer_pos {
		let message_size = protocol.connection.in_buffer[0];

		let message = unsafe {
			str::raw::from_buf_len(
				(protocol.connection.in_buffer.as_ptr() as *u8).offset(1),
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
				protocol.connection.in_buffer.as_mut_ptr(),
				protocol.connection.in_buffer.as_ptr().offset(message_size as int),
				(protocol.connection.in_buffer.len() - message_size as uint) as uint);
			protocol.connection.in_buffer_pos -= message_size as uint;
		}
	}
}

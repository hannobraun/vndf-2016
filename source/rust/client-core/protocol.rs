use std::ptr;
use std::str;

use common::net::Connection;


pub struct Protocol {
	connection: Connection
}


pub fn init(connection: Connection) -> Protocol {
	Protocol {
		connection: connection
	}
}

pub fn receive_positions(protocol: &mut Protocol, handler: |~str|) {
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

		handler(message);

		unsafe {
			ptr::copy_memory(
				protocol.connection.in_buffer.as_mut_ptr(),
				protocol.connection.in_buffer.as_ptr().offset(message_size as int),
				(protocol.connection.in_buffer.len() - message_size as uint) as uint);
			protocol.connection.in_buffer_pos -= message_size as uint;
		}
	}
}

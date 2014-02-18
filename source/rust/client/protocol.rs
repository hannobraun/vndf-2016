use std::from_str;
use std::libc;
use std::ptr;
use std::str;

use common::vec::Vec2;

use display;
use net;


static BUFFER_SIZE : libc::c_int = 256;


pub struct Connection {
	socket_fd: libc::c_int,
	buffer   : [i8, ..BUFFER_SIZE],
	buffer_pos: uint
}


pub fn init(socket_fd: libc::c_int) -> Connection {
	Connection {
		socket_fd : socket_fd,
		buffer    : [0, ..BUFFER_SIZE],
		buffer_pos: 0 }
}

pub fn receive_positions(
	connection: &mut Connection,
	positions : &mut display::PosMap) {

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

		if message.starts_with("UPDATE") {
			let parts: ~[&str] = message.words().collect();

			let id_str = parts[2].trim_chars(&',');
			let x_str  = parts[4].trim_chars(&',').trim_chars(&'(');
			let y_str  = parts[5].trim_chars(&')');

			let id: int = from_str::from_str(id_str).unwrap_or_else(|| { fail!() });

			let x = from_str::from_str(x_str).unwrap_or_else(|| { fail!() });
			let y = from_str::from_str(y_str).unwrap_or_else(|| { fail!() });

			positions.insert(id, Vec2 { x: x, y: y });
		}
		else if message.starts_with("REMOVE") {
			let parts: ~[&str] = message.words().collect();

			let id_str = parts[2];

			let id: int = from_str::from_str(id_str).unwrap_or_else(|| { fail!() });

			positions.remove(&id);
		}
		else {
			print!("Unknown message type in message: {:s}\n", message);
			fail!();
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

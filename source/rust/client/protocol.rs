use std::from_str;
use std::libc;
use std::ptr;
use std::str;

use display;
use net;


pub static BUFFER_SIZE : libc::c_int = 256;


pub struct Connection {
	socket_fd: libc::c_int,
	buffer   : [libc::c_char, ..BUFFER_SIZE],
	bufferPos: libc::c_int
}


pub fn receive_positions(c: &mut Connection, positions: &mut display::PosMap) {
	unsafe {
		let bytesReceived = net::receive(
			c.socket_fd,
			c.buffer.slice_from(c.bufferPos as uint));

		c.bufferPos += bytesReceived as i32;

		while c.bufferPos > 0 && c.buffer[0] as i32 <= c.bufferPos {
			let messageSize = c.buffer[0];
			assert!(messageSize >= 0);

			let message = str::raw::from_buf_len(
				(c.buffer.as_ptr() as *u8).offset(1),
				(messageSize - 1) as uint);

			if message.starts_with("UPDATE") {
				let parts: ~[&str] = message.words().collect();

				let id_str = parts[2].trim_chars(&',');
				let x_str  = parts[4].trim_chars(&',').trim_chars(&'(');
				let y_str  = parts[5].trim_chars(&')');

				let id: int = from_str::from_str(id_str).unwrap_or_else(|| { fail!() });

				let x: f32 = from_str::from_str(x_str).unwrap_or_else(|| { fail!() });
				let y: f32 = from_str::from_str(y_str).unwrap_or_else(|| { fail!() });

				positions.insert(id, display::Position { x: x, y: y });
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

			ptr::copy_memory(
				c.buffer.as_mut_ptr(),
				c.buffer.as_ptr().offset(messageSize as int),
				(BUFFER_SIZE - messageSize as i32) as uint);
			c.bufferPos -= messageSize as i32;
		}
	}
}

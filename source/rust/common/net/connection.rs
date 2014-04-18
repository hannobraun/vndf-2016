use libc;
use libc::c_int;
use std::ptr;

use net::ffi;
use util::last_error;


#[deriving(Eq, Show)]
pub struct Connection {
	pub fd: c_int
}

impl Connection {
	pub fn from_fd(fd: c_int) -> Connection {
		Connection {
			fd: fd
		}
	}

	pub fn send_message(&self, message: &str) -> Result<(), ~str> {
		let mut buffer: [libc::c_char, ..256] = [0, ..256];

		unsafe {
			message.to_c_str().with_ref(|c_message| {
				let messageLength = libc::strlen(c_message);

				ptr::set_memory(
					buffer.as_mut_ptr(),
					(messageLength + 1) as u8,
					1);

				ptr::copy_memory(
					buffer.as_mut_ptr().offset(1),
					c_message,
					messageLength as uint);

				let buffer_length = messageLength + 1;

				let bytesSent = ffi::send(
					self.fd,
					buffer.as_ptr() as *mut libc::c_void,
					buffer_length,
					ffi::MSG_NOSIGNAL);

				if bytesSent < 0 {
					Err(format!("Error sending message: {}", last_error()))
				}
				else if bytesSent as u64 != buffer_length {
					Err(format!(
						"Only sent {:d} of {:u} bytes",
						bytesSent,
						buffer_length))
				}
				else {
					Ok(())
				}
			})
		}
	}
}

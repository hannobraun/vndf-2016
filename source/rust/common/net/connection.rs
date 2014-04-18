use libc;
use libc::c_int;
use std::ptr;

use net::ffi;


#[deriving(Eq, Show)]
pub struct Connection {
	pub fd: c_int
}

impl Connection {
	pub fn create(fd: c_int) -> Connection {
		Connection {
			fd: fd
		}
	}

	pub fn send_message(&self, message: &str) -> c_int {
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
					"Error sending message".to_c_str().with_ref(|c_str| {
						libc::perror(c_str);
					});

					-1
				}
				else if bytesSent as u64 != buffer_length {
					format!(
						"Only sent {:d} of {:u} bytes.\n",
						bytesSent,
						buffer_length);
					libc::exit(1)
				}
				else {
					0
				}
			})
		}
	}

}

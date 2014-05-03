use libc;
use libc::c_int;
use std::io::{
	IoError,
	IoResult
};
use std::os;
use std::ptr;
use std::str;

use net::ffi;
use util::last_error;


#[deriving(Eq, Show)]
pub struct Connection {
	pub fd: c_int,

	in_buffer    : ~[u8],
	in_buffer_pos: uint
}

impl Connection {
	pub fn from_fd(fd: c_int) -> Connection {
		Connection {
			fd           : fd,
			in_buffer    : ~[0, ..1024],
			in_buffer_pos: 0
		}
	}

	pub fn connect(hostname: &str, port: &str) -> IoResult<Connection> {
		let hints = ffi::addrinfo {
			ai_flags    : ffi::AI_PASSIVE,
			ai_family   : ffi::AF_UNSPEC,
			ai_socktype : ffi::SOCK_STREAM,
			ai_protocol : 0,
			ai_addrlen  : 0,
			ai_addr     : ptr::null(),
			ai_canonname: ptr::null(),
			ai_next     : ptr::null()
		};

		let servinfo: *ffi::addrinfo = ptr::null();

		unsafe {
			let mut status = hostname.to_c_str().with_ref(|c_hostname| {
				port.to_c_str().with_ref(|c_port| {
					ffi::getaddrinfo(
						c_hostname,
						c_port,
						&hints,
						&servinfo)
				})
			});

			if status != 0 {
				return Err(IoError::last_error())
			}

			let fd = ffi::socket(
				(*servinfo).ai_family,
				(*servinfo).ai_socktype,
				(*servinfo).ai_protocol);

			if fd == -1 {
				return Err(IoError::last_error())
			}

			status = ffi::connect(
				fd,
				(*servinfo).ai_addr,
				(*servinfo).ai_addrlen);

			if status != 0 {
				return Err(IoError::last_error())
			}

			ffi::freeaddrinfo(servinfo);

			Ok(Connection::from_fd(fd))
		}
	}

	pub fn send_message(&self, message: &str) -> Result<(), ~str> {
		let mut buffer: [libc::c_char, ..1024] = [0, ..1024];

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

				assert!(buffer_length <= buffer.len() as u64);

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

	pub fn receive_messages(&mut self, handler: |~str|) -> Result<(), &'static str> {
		let result = self.receive(
			self.in_buffer.slice_from(self.in_buffer_pos));

		let bytes_received = match result {
			Ok(bytes_received) => bytes_received,
			Err(error)         => return Err(error)
		};

		self.in_buffer_pos += bytes_received as uint;

		while self.in_buffer_pos > 0 && self.in_buffer[0] as uint <= self.in_buffer_pos {
			let message_size = self.in_buffer[0];

			let message = unsafe {
				str::raw::from_buf_len(
					(self.in_buffer.as_ptr() as *u8).offset(1),
					(message_size - 1) as uint)
			};

			handler(message);

			unsafe {
				ptr::copy_memory(
					self.in_buffer.as_mut_ptr(),
					self.in_buffer.as_ptr().offset(message_size as int),
					(self.in_buffer.len() - message_size as uint) as uint);
				self.in_buffer_pos -= message_size as uint;
			}
		}

		Ok(())
	}

	fn receive(&self, buffer: &[u8]) -> Result<libc::ssize_t, &'static str> {
		unsafe {
			let bytes_received = ffi::recv(
				self.fd,
				buffer.as_ptr() as *mut libc::c_void,
				buffer.len() as u64,
				ffi::MSG_DONTWAIT);

			if bytes_received == -1
				&& (
					os::errno() as i32 == ffi::EAGAIN ||
					os::errno() as i32 == ffi::EWOULDBLOCK) {

				return Ok(0);
			}
			if bytes_received == -1 {
				return Err(IoError::last_error().desc)
			}
			if bytes_received == 0 {
				return Err("Connection closed by server")
			}

			Ok(bytes_received)
		}
	}

	pub fn close(&self) {
		ffi::close(self.fd as int);
	}
}

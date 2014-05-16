use libc;
use libc::c_int;
use std::io::{
	Closed,
	IoError,
	IoResult,
	OtherIoError
};
use std;
use std::mem::size_of;
use std::os;
use std::ptr;
use std::str;

use net::ffi;


type MessageLength = u16;

static MAX_MSG_LENGTH: MessageLength = std::u16::MAX;


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

	pub fn send_message(&self, message: &str) -> IoResult<()> {
		let mut buffer: [libc::c_char, ..1024] = [0, ..1024];

		let size_of_length = size_of::<MessageLength>();
		let message_length = message.as_bytes().len() + size_of_length;

		assert!(message_length <= buffer.len());
		assert!(message_length <= MAX_MSG_LENGTH as uint);

		unsafe {
			ptr::copy_memory(
				buffer.as_mut_ptr(),
				&(message_length as MessageLength) as *MessageLength as *i8,
				size_of_length);

			ptr::copy_memory(
				buffer.as_mut_ptr().offset(size_of_length as int),
				message.as_ptr() as *i8,
				message_length - size_of_length);

			self.send(buffer, message_length)
		}
	}

	fn send(&self, buffer: &[i8], length: uint) -> IoResult<()> {
		let bytes_sent = unsafe {
			ffi::send(
				self.fd,
				buffer.as_ptr() as *mut libc::c_void,
				length as u64,
				ffi::MSG_NOSIGNAL)
		};

		if bytes_sent < 0 {
			Err(IoError::last_error())
		}
		else if bytes_sent as uint != length {
			Err(IoError {
				kind  : OtherIoError,
				desc  : "Could not send all bytes",
				detail: Some(format!(
					"Only sent {:d} of {:u} bytes",
					bytes_sent,
					length))
			})
		}
		else {
			Ok(())
		}
	}

	pub fn receive_messages(&mut self, handler: |~str|) -> IoResult<()> {
		let result = self.receive(
			self.in_buffer.slice_from(self.in_buffer_pos));

		let bytes_received = match result {
			Ok(bytes_received) => bytes_received,
			Err(error)         => return Err(error)
		};

		self.in_buffer_pos += bytes_received as uint;

		loop {
			let size_of_length = size_of::<MessageLength>();
			let mut message_length: MessageLength = 0;
			unsafe {
				ptr::copy_memory(
					&mut message_length as *mut MessageLength as *mut u8,
					self.in_buffer.as_ptr(),
					size_of_length);
			}

			if self.in_buffer_pos == 0 {
				break;
			}
			if message_length as uint > self.in_buffer_pos {
				break;
			};

			let message = unsafe {
				str::raw::from_buf_len(
					(self.in_buffer.as_ptr() as *u8).offset(size_of_length as int),
					(message_length - size_of_length as MessageLength) as uint)
			};

			handler(message);

			unsafe {
				ptr::copy_memory(
					self.in_buffer.as_mut_ptr(),
					self.in_buffer.as_ptr().offset(message_length as int),
					(self.in_buffer.len() - message_length as uint) as uint);
				self.in_buffer_pos -= message_length as uint;
			}
		}

		Ok(())
	}

	fn receive(&self, buffer: &[u8]) -> IoResult<libc::ssize_t> {
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
				return Err(IoError::last_error())
			}
			if bytes_received == 0 {
				return Err(IoError {
					kind  : Closed,
					desc  : "Connection closed by remote host",
					detail: None
				})
			}

			Ok(bytes_received)
		}
	}

	pub fn close(&self) {
		ffi::close(self.fd as int);
	}
}

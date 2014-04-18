use libc;
use libc::c_int;
use std::os;
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

	pub fn connect(hostname: ~str, port: ~str) -> Connection {
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
				fail!("Error getting address info: {}", last_error());
			}

			let fd = ffi::socket(
				(*servinfo).ai_family,
				(*servinfo).ai_socktype,
				(*servinfo).ai_protocol);

			if fd == -1 {
				fail!("Error creating socket: {}", last_error());
			}

			status = ffi::connect(
				fd,
				(*servinfo).ai_addr,
				(*servinfo).ai_addrlen);

			if status != 0 {
				fail!("Error connecting to server ({}:{}): {}",
					hostname,
					port,
					last_error());
			}

			ffi::freeaddrinfo(servinfo);

			Connection::from_fd(fd)
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

	pub fn receive(&self, buffer: &[u8]) -> libc::ssize_t {
		unsafe {
			let bytes_received = ffi::recv(
				self.fd,
				buffer.as_ptr() as *mut libc::c_void,
				buffer.len() as u64,
				ffi::MSG_DONTWAIT);

			if bytes_received == -1 && (os::errno() as i32 == ffi::EAGAIN || os::errno() as i32 == ffi::EWOULDBLOCK) {
				return 0;
			}
			if bytes_received == -1 {
				"Error receiving message".to_c_str().with_ref(|c_str| {
					libc::perror(c_str);
					libc::exit(1);
				})
			}
			if bytes_received == 0 {
				fail!("Connection closed while receiving");
			}

			bytes_received
		}
	}
}

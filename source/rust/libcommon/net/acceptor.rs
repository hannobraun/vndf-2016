use libc;
use libc::c_int;
use std::io::{
	IoError,
	IoResult
};
use std::mem;
use std::ptr;

use net::Connection;
use net::ffi;


pub struct Acceptor {
	pub fd: c_int
}

impl Acceptor {
	pub fn create(port: &str) -> IoResult<Acceptor> {
		match init_socket(port) {
			Ok(fd) =>
				Ok(Acceptor {
					fd: fd
				}),

			Err(error) =>
				Err(error)
		}
	}

	pub fn accept(&self) -> IoResult<Connection> {
		let fd = unsafe {
			ffi::accept(
				self.fd,
				ptr::mut_null(),
				ptr::mut_null())
		};

		if fd >= 0 {
			Ok(Connection::from_fd(fd))
		}
		else {
			Err(IoError::last_error())
		}
	}
}


fn init_socket(port: &str) -> IoResult<c_int> {
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
		let status = port.to_c_str().with_ref(|c_message| {
			ffi::getaddrinfo(
				ptr::null(),
				c_message,
				&hints,
				&servinfo)
		});

		if status != 0 {
			return Err(IoError::last_error());
		}

		let socket_fd = ffi::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);

		if socket_fd == -1 {
			return Err(IoError::last_error());
		}

		let yes = 1;
		let status = ffi::setsockopt(
			socket_fd,
			ffi::SOL_SOCKET,
			ffi::SO_REUSEADDR,
			&yes as *int as *libc::c_void,
			mem::size_of::<c_int>() as u32);

		if status == -1 {
			return Err(IoError::last_error());
		}

		let status = ffi::bind(
			socket_fd,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			return Err(IoError::last_error());
		}

		let status = ffi::listen(
			socket_fd,
			1024);
		if status != 0 {
			return Err(IoError::last_error());
		}

		ffi::freeaddrinfo(servinfo);

		Ok(socket_fd)
	}
}

use std::libc;
use std::libc::funcs::bsd43;
use std::libc::types::os::common::bsd44;
use std::ptr;

use common::net;


extern {
	fn __errno_location() -> *libc::c_int;
}

fn errno() -> libc::c_int {
	unsafe {
		*__errno_location()
	}
}


static MSG_DONTWAIT: i32 = 0x40;
static EAGAIN      : i32 = 11;
static EWOULDBLOCK : i32 = 140;


pub fn connect(hostname: ~str, port: ~str) -> libc::c_int {
	let hints = bsd44::addrinfo {
		ai_flags    : net::AI_PASSIVE,
		ai_family   : net::AF_UNSPEC,
		ai_socktype : net::SOCK_STREAM,
		ai_protocol : 0,
		ai_addrlen  : 0,
		ai_addr     : ptr::null(),
		ai_canonname: ptr::null(),
		ai_next     : ptr::null() };

	let servinfo = ptr::null::<bsd44::addrinfo>();

	unsafe {
		let mut status = hostname.to_c_str().with_ref(|c_hostname| {
			port.to_c_str().with_ref(|c_port| {
				net::getaddrinfo(
					c_hostname,
					c_port,
					&hints,
					&servinfo)
			})
		});

		if status != 0 {
			"Error getting address info".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let socketFD = bsd43::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);
		if socketFD == -1 {
			"Error creating socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		status = bsd43::connect(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);
		if status != 0 {
			(format!("Error connecting to server ({}:{})", hostname, port)).to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		net::freeaddrinfo(servinfo);

		socketFD
	}
}

pub fn receive(socketFD: libc::c_int, buffer: &[i8]) -> libc::ssize_t {
	unsafe {
		let bytesReceived = bsd43::recv(
			socketFD,
			buffer.as_ptr() as *mut libc::c_void,
			buffer.len() as u64,
			MSG_DONTWAIT);

		if bytesReceived == -1 && (errno() == EAGAIN || errno() == EWOULDBLOCK) {
			return 0;
		}
		if bytesReceived == -1 {
			"Error receiving message".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
				libc::exit(1);
			})
		}
		if bytesReceived == 0 {
			print!("Connection closed while receiving.\n");
			libc::exit(1);
		}

		bytesReceived
	}
}

use libc;
use std::os::errno;
use std::ptr;

use common::net;


pub fn connect(hostname: ~str, port: ~str) -> libc::c_int {
	let hints = net::ffi::addrinfo {
		ai_flags    : net::ffi::AI_PASSIVE,
		ai_family   : net::ffi::AF_UNSPEC,
		ai_socktype : net::ffi::SOCK_STREAM,
		ai_protocol : 0,
		ai_addrlen  : 0,
		ai_addr     : ptr::null(),
		ai_canonname: ptr::null(),
		ai_next     : ptr::null() };

	let servinfo = ptr::null::<net::ffi::addrinfo>();

	unsafe {
		let mut status = hostname.to_c_str().with_ref(|c_hostname| {
			port.to_c_str().with_ref(|c_port| {
				net::ffi::getaddrinfo(
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

		let socketFD = net::ffi::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);
		if socketFD == -1 {
			"Error creating socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		status = net::ffi::connect(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);
		if status != 0 {
			(format!("Error connecting to server ({}:{})", hostname, port)).to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		net::ffi::freeaddrinfo(servinfo);

		socketFD
	}
}

pub fn receive(socketFD: libc::c_int, buffer: &[u8]) -> libc::ssize_t {
	unsafe {
		let bytesReceived = net::ffi::recv(
			socketFD,
			buffer.as_ptr() as *mut libc::c_void,
			buffer.len() as u64,
			net::ffi::MSG_DONTWAIT);

		if bytesReceived == -1 && (errno() as i32 == net::ffi::EAGAIN || errno() as i32 == net::ffi::EWOULDBLOCK) {
			return 0;
		}
		if bytesReceived == -1 {
			"Error receiving message".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
				libc::exit(1);
			})
		}
		if bytesReceived == 0 {
			fail!("Connection closed while receiving");
		}

		bytesReceived
	}
}

use std::libc;
use std::ptr;

use common::net;


extern {
	fn __errno_location() -> *libc::c_int;
	fn recv(sockfd: libc::c_int, buf: *libc::c_void, len: libc::size_t, flags: libc::c_int) -> libc::ssize_t;
	fn connect(sockfd: libc::c_int, addr: *net::SockAddr, addrlen: libc::c_uint) -> libc::c_int;
}

fn errno() -> libc::c_int {
	unsafe {
		*__errno_location()
	}
}


pub fn net_connect(hostname: *libc::c_char, port: *libc::c_char) -> libc::c_int {
	let hints = net::AddrInfo {
		ai_flags    : net::AI_PASSIVE,
		ai_family   : net::AF_UNSPEC,
		ai_socktype : net::SOCK_STREAM,
		ai_protocol : 0,
		ai_addrlen  : 0,
		ai_addr     : ptr::null(),
		ai_canonname: ptr::null(),
		ai_next     : ptr::null() };

	let servinfo = ptr::null::<net::AddrInfo>();

	unsafe {
		let mut status = net::getaddrinfo(
			hostname,
			port,
			&hints,
			&servinfo);

		if status != 0 {
			"Error getting address info".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let socketFD = net::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);
		if socketFD == -1 {
			"Error creating socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		status = connect(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);
		if status != 0 {
			"Error connecting to server".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		net::freeaddrinfo(servinfo);

		socketFD
	}
}

#[no_mangle]
pub extern fn net_receive(socketFD: libc::c_int, buffer: *libc::c_char, bufferSize: libc::size_t) -> libc::ssize_t {
	let MSG_DONTWAIT = 0x40;
	let EAGAIN       = 11;
	let EWOULDBLOCK  = 140;

	unsafe {
		let bytesReceived = recv(
			socketFD,
			buffer as *libc::c_void,
			bufferSize,
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

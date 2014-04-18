use libc;
use libc::c_int;
use std::c_str::CString;
use std::mem;
use std::os;
use std::ptr;

use net::ffi;


pub fn init_socket(port: &str) -> c_int {
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
			fail!("Error getting address info: {}", last_error());
		}

		let socket_fd = ffi::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);

		if socket_fd == -1 {
			fail!("Error creating socket: {}", last_error());
		}

		let yes = 1;
		let status = ffi::setsockopt(
			socket_fd,
			ffi::SOL_SOCKET,
			ffi::SO_REUSEADDR,
			&yes as *int as *libc::c_void,
			mem::size_of::<c_int>() as u32);

		if status == -1 {
			fail!("Error setting socket option: {}", last_error());
		}

		let status = ffi::bind(
			socket_fd,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			fail!("Error binding socket: {}", last_error());
		}

		let status = ffi::listen(
			socket_fd,
			1024);
		if status != 0 {
			fail!("Error listening on socket: {}", last_error());
		}

		ffi::freeaddrinfo(servinfo);

		socket_fd
	}
}

pub fn init_poller() -> c_int {
	let poller_fd = unsafe {
		ffi::epoll_create(1)
	};

	if poller_fd < 0 {
		fail!("Error initiating epoll: {}", last_error());
	}

	poller_fd
}

pub fn register_accept(poller_fd: c_int, server_fd: c_int) {
	let event = ffi::epoll_event {
		events: ffi::EPOLLIN,
		data  : 0 };

	let status = unsafe {
		ffi::epoll_ctl(
			poller_fd,
			ffi::EPOLL_CTL_ADD,
			server_fd,
			&event)
	};

	if status != 0 {
		fail!("Error registering server socket with epoll: {}", last_error());
	}
}

fn last_error() -> ~str {
	unsafe {
		let c_error = libc::strerror(os::errno() as i32);
		CString::new(c_error, false)
			.as_str()
			.expect("failed to convert C error message into Rust string")
			.to_owned()
	}
}

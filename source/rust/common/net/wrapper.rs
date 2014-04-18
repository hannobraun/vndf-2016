use libc;
use libc::c_int;
use std::c_str::CString;
use std::os;
use std::ptr;

use net::ffi;


pub fn init_socket(port: &str) -> libc::c_int {
	let hints = ffi::addrinfo {
		ai_flags    : ffi::AI_PASSIVE,
		ai_family   : ffi::AF_UNSPEC,
		ai_socktype : ffi::SOCK_STREAM,
		ai_protocol : 0,
		ai_addrlen  : 0,
		ai_addr     : ptr::null(),
		ai_canonname: ptr::null(),
		ai_next     : ptr::null() };

	let servinfo = ptr::null::<ffi::addrinfo>();

	unsafe {
		let status = port.to_c_str().with_ref(|c_message| {
			ffi::getaddrinfo(
				ptr::null(),
				c_message,
				&hints,
				&servinfo)
		});

		if status != 0 {
			"Error getting address info".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let socketFD = ffi::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);

		if socketFD == -1 {
			"Error creating socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let yes= 1;
		let status = ffi::setsockopt(
			socketFD,
			ffi::SOL_SOCKET,
			ffi::SO_REUSEADDR,
			&yes as *int as *libc::c_void,
			::std::mem::size_of::<libc::c_int>() as u32);

		if status == -1 {
			"Error setting socket option".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let status = ffi::bind(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			"Error binding socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let status = ffi::listen(
			socketFD,
			1024);
		if status != 0 {
			"Error listening on socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		ffi::freeaddrinfo(servinfo);

		socketFD
	}
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

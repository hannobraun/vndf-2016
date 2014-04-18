use libc;
use std::ptr;

use common::net;


pub struct Net {
	pub pollerFD: libc::c_int,
	pub serverFD: libc::c_int
}


pub fn init(port: &str) -> Net {
	let serverFD = init_socket(port);
	let pollerFD = init_poller();

	register_accept(pollerFD, serverFD);

	print!("Listening on port {}\n", port);

	Net {
		pollerFD: pollerFD,
		serverFD: serverFD }
}


fn init_socket(port: &str) -> libc::c_int {
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
		let status = port.to_c_str().with_ref(|c_message| {
			net::ffi::getaddrinfo(
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

		let yes= 1;
		let status = net::ffi::setsockopt(
			socketFD,
			net::ffi::SOL_SOCKET,
			net::ffi::SO_REUSEADDR,
			&yes as *int as *libc::c_void,
			::std::mem::size_of::<libc::c_int>() as u32);

		if status == -1 {
			"Error setting socket option".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let status = net::ffi::bind(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			"Error binding socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let status = net::ffi::listen(
			socketFD,
			1024);
		if status != 0 {
			"Error listening on socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		net::ffi::freeaddrinfo(servinfo);

		socketFD
	}
}

fn init_poller() -> libc::c_int {
	unsafe {
		let pollerFD = net::ffi::epoll_create(1);
		if pollerFD < 0 {
			"Error initiating epoll".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		pollerFD
	}
}

fn register_accept(pollerFD: libc::c_int, serverFD: libc::c_int) {
	let event = net::ffi::epoll_event { events: net::ffi::EPOLLIN, data: 0 };

	unsafe {
		let status = net::ffi::epoll_ctl(
			pollerFD,
			net::ffi::EPOLL_CTL_ADD,
			serverFD,
			&event);

		if status != 0 {
			"Error registering server socket with epoll".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}
	}
}

pub fn number_of_events(net: &Net, frameTimeInMs: i32) -> i32 {
	let emptyEvent = net::ffi::epoll_event {
		events: 0,
		data  : 0 };
	let pollEvents: [net::ffi::epoll_event, ..1024] = [emptyEvent, ..1024];

	unsafe {
		let numberOfEvents = net::ffi::epoll_wait(
			net.pollerFD,
			pollEvents.as_ptr(),
			1024,
			frameTimeInMs);

		assert!(numberOfEvents != -1);

		numberOfEvents
	}
}

pub fn accept_client(serverFD: libc::c_int) -> libc::c_int {
	unsafe {
		net::ffi::accept(
			serverFD,
			ptr::mut_null(),
			ptr::mut_null())
	}
}

pub fn send_message(clientFD: libc::c_int, message: &str) -> libc::c_int {
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

			let bytesSent = net::ffi::send(
				clientFD,
				buffer.as_ptr() as *mut libc::c_void,
				buffer_length,
				net::ffi::MSG_NOSIGNAL);

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

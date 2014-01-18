use std::libc;
use std::ptr;

use common::net;


extern {
	fn epoll_create(size: libc::c_int) -> libc::c_int;

	fn epoll_ctl(
		epfd : libc::c_int,
		op   : libc::c_int,
		fd   : libc::c_int,
		event: *EpollEvent) -> libc::c_int;

	fn epoll_wait(
		epfd     : libc::c_int,
		events   : *EpollEvent,
		maxevents: libc::c_int,
		timeout  : libc::c_int) -> libc::c_int;

	fn setsockopt(
		sockfd : libc::c_int,
		level  : libc::c_int,
		optname: libc::c_int,
		optval : *libc::c_void,
		optlen : libc::c_uint) -> libc::c_int;

	fn bind(
		sockfd : libc::c_int,
		addr   : *net::SockAddr,
		addrlen: libc::c_uint) -> libc::c_int;

	fn listen(
		sockfd : libc::c_int,
		backlog: libc::c_int) -> libc::c_int;

	fn accept(
		sockfd : libc::c_int,
		addr   : *net::SockAddr,
		addrlen: *libc::c_uint) -> libc::c_int;

	fn send(
		sockfd: libc::c_int,
		buf   : *libc::c_void,
		len   : libc::size_t,
		flags : libc::c_int) -> libc::ssize_t;
}


struct Net {
	pollerFD: libc::c_int,
	serverFD: libc::c_int
}

struct EpollEvent {
	events: u32,
	data  : u64
}


pub fn init(port: &str) -> Net {
	let serverFD = init_socket(port);
	let pollerFD = init_poller();

	register_accept(pollerFD, serverFD);

	Net {
		pollerFD: pollerFD,
		serverFD: serverFD }
}


fn init_socket(port: &str) -> libc::c_int {
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
		let status = port.to_c_str().with_ref(|c_message| {
			net::getaddrinfo(
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

		let socketFD = net::socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);

		if (socketFD == -1) {
			"Error creating socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let SOL_SOCKET   = 1;
		let SO_REUSEADDR = 2;

		let yes = 1;
		let status = setsockopt(
			socketFD,
			SOL_SOCKET,
			SO_REUSEADDR,
			ptr::to_unsafe_ptr(&yes) as *libc::c_void,
			::std::mem::size_of::<libc::c_int>() as u32);

		if status == -1 {
			"Error setting socket option".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let status = bind(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			"Error binding socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		let status = listen(
			socketFD,
			1024);
		if status != 0 {
			"Error listening on socket".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}

		net::freeaddrinfo(servinfo);

		socketFD
	}
}

fn init_poller() -> libc::c_int {
	unsafe {
		let pollerFD = epoll_create(1);
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
	let EPOLLIN = 1;
	let EPOLL_CTL_ADD = 1;

	let event = EpollEvent { events: EPOLLIN, data: 0 };

	unsafe {
		let status = epoll_ctl(
			pollerFD,
			EPOLL_CTL_ADD,
			serverFD,
			ptr::to_unsafe_ptr(&event));

		if status != 0 {
			"Error registering server socket with epoll".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			libc::exit(1);
		}
	}
}

pub fn number_of_events(net: &Net, frameTimeInMs: i32) -> i32 {
	let emptyEvent = EpollEvent {
		events: 0,
		data  : 0 };
	let pollEvents: [EpollEvent, ..1024] = [emptyEvent, ..1024];

	unsafe {
		let numberOfEvents = epoll_wait(
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
		accept(
			serverFD,
			ptr::null(),
			ptr::null())
	}
}

pub fn send_message(clientFD: libc::c_int, message: &str) -> libc::c_int {

	let MSG_NOSIGNAL = 0x4000;

	let mut buffer: [libc::c_char, ..256] = [0, ..256];

	unsafe {
		message.to_c_str().with_ref(|c_message| {
			let messageLength = libc::strlen(c_message);

			ptr::set_memory(
				buffer.as_mut_ptr(),
				(messageLength + 1) as u8,
				1);

			ptr::copy_memory(
				ptr::mut_offset(buffer.as_mut_ptr(), 1),
				c_message,
				messageLength as uint);

			let buffer_length = messageLength + 1;

			let bytesSent = send(
				clientFD,
				buffer.as_ptr() as *libc::c_void,
				buffer_length,
				MSG_NOSIGNAL);

			if bytesSent < 0 {
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

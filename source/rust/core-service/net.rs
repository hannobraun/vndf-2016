use std::libc;

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

	fn getaddrinfo(
		name   : *libc::c_char,
		service: *libc::c_char,
		req    : *AddrInfo,
		pai    : **AddrInfo) -> libc::c_int;

	fn socket(
		domain  : libc::c_int,
		theType : libc::c_int,
		protocol: libc::c_int) -> libc::c_int;

	fn setsockopt(
		sockfd : libc::c_int,
		level  : libc::c_int,
		optname: libc::c_int,
		optval : *libc::c_void,
		optlen : libc::c_uint) -> libc::c_int;

	fn bind(
		sockfd : libc::c_int,
		addr   : *SockAddr,
		addrlen: libc::c_uint) -> libc::c_int;

	fn listen(
		sockfd : libc::c_int,
		backlog: libc::c_int) -> libc::c_int;

	fn freeaddrinfo(res: *AddrInfo);

	fn accept(
		sockfd : libc::c_int,
		addr   : *SockAddr,
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

struct AddrInfo {
	ai_flags    : libc::c_int,
	ai_family   : libc::c_int,
	ai_socktype : libc::c_int,
	ai_protocol : libc::c_int,
	ai_addrlen  : u32,
	ai_addr     : *SockAddr,
	ai_canonname: *libc::c_char,
	ai_next     : *AddrInfo
}

struct SockAddr {
	sa_family: libc::c_ushort,
	sa_data  : [libc::c_char, ..14]
}

struct EpollEvent {
	events: u32,
	data  : u64
}


pub fn init(port: &str) -> Net {
	let serverFD = port.to_c_str().with_ref(|c_str| {
		init_socket(c_str)
	});
	let pollerFD = init_poller();

	register_accept(pollerFD, serverFD);

	Net {
		pollerFD: pollerFD,
		serverFD: serverFD }
}


fn init_socket(port: *libc::c_char) -> libc::c_int {
	let AI_PASSIVE  = 1;
	let AF_UNSPEC   = 0;
	let SOCK_STREAM = 1;

	let hints = AddrInfo {
		ai_flags    : AI_PASSIVE,
		ai_family   : AF_UNSPEC,
		ai_socktype : SOCK_STREAM,
		ai_protocol : 0,
		ai_addrlen  : 0,
		ai_addr     : ::std::ptr::null(),
		ai_canonname: ::std::ptr::null(),
		ai_next     : ::std::ptr::null() };

	let servinfo = ::std::ptr::null::<AddrInfo>();

	unsafe {
		let status = getaddrinfo(
			::std::ptr::null(),
			port,
			&hints,
			&servinfo);

		if status != 0 {
			"Error getting address info".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
			});
			libc::exit(1);
		}

	};

	let socketFD = unsafe{
		let socketFD = socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);

		if (socketFD == -1) {
			"Error creating socket".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
			});
			libc::exit(1);
		}

		socketFD };

	let SOL_SOCKET   = 1;
	let SO_REUSEADDR = 2;

	unsafe {
		let yes = 1;
		let status = setsockopt(
			socketFD,
			SOL_SOCKET,
			SO_REUSEADDR,
			::std::ptr::to_unsafe_ptr(&yes) as *libc::c_void,
			::std::mem::size_of::<libc::c_int>() as u32);

		if status == -1 {
			"Error setting socket option".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
			});
			libc::exit(1);
		}
	}

	unsafe {
		let status = bind(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			"Error binding socket".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
			});
			libc::exit(1);
		}
	}

	unsafe {
		let status = listen(
			socketFD,
			1024);
		if status != 0 {
			"Error listening on socket".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
			});
			libc::exit(1);
		}
	}

	unsafe {
		freeaddrinfo(servinfo);
	}

	socketFD
}

fn init_poller() -> libc::c_int {
	unsafe {
		let pollerFD = epoll_create(1);
		if pollerFD < 0 {
			"Error initiating epoll".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
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
		let status = epoll_ctl(pollerFD, EPOLL_CTL_ADD, serverFD, ::std::ptr::to_unsafe_ptr(&event));
		if status != 0 {
			"Error registering server socket with epoll".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
			});
			libc::exit(1);
		}
	}
}

#[no_mangle]
pub extern fn net_number_of_events(net: &Net, frameTimeInMs: libc::c_int) -> libc::c_int {
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
			::std::ptr::null(),
			::std::ptr::null())
	}
}

pub fn send_message(clientFD: libc::c_int, message: *libc::c_char, messageLength: libc::size_t) -> libc::c_int {
	let MSG_NOSIGNAL = 0x4000;

	unsafe {
		let bytesSent = send(
			clientFD,
			message as *libc::c_void,
			messageLength,
			MSG_NOSIGNAL);

		if bytesSent < 0 {
			-1
		}
		else if bytesSent as u64 != messageLength {
			format!(
				"Only sent {:d} of {:u} bytes.\n",
				bytesSent,
				messageLength);
			libc::exit(1)

		}
		else {
			0
		}
	}
}

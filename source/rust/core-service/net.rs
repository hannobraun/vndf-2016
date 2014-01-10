extern {
	fn epoll_create(size: ::std::libc::c_int) -> ::std::libc::c_int;
	fn epoll_ctl(epfd: ::std::libc::c_int, op: ::std::libc::c_int, fd: ::std::libc::c_int, event: *EpollEvent) -> ::std::libc::c_int;
	fn getaddrinfo(name: *::std::libc::c_char, service: *::std::libc::c_char, req: *AddrInfo, pai: **AddrInfo) -> ::std::libc::c_int;
	fn socket(domain: ::std::libc::c_int, theType: ::std::libc::c_int, protocol: ::std::libc::c_int) -> ::std::libc::c_int;
	fn setsockopt(sockfd: ::std::libc::c_int, level: ::std::libc::c_int, optname: ::std::libc::c_int, optval: *::std::libc::c_void, optlen: ::std::libc::c_uint) -> ::std::libc::c_int;
	fn bind(sockfd: ::std::libc::c_int, addr: *SockAddr, addrlen: ::std::libc::c_uint) -> ::std::libc::c_int;
	fn listen(sockfd: ::std::libc::c_int, backlog: ::std::libc::c_int) -> ::std::libc::c_int;
	fn freeaddrinfo(res: *AddrInfo);
	fn accept(sockfd: ::std::libc::c_int, addr: *SockAddr, addrlen: *::std::libc::c_uint) -> ::std::libc::c_int;
	fn send(sockfd: ::std::libc::c_int, buf: *::std::libc::c_void, len: ::std::libc::size_t, flags: ::std::libc::c_int) -> ::std::libc::ssize_t;
}


struct Net {
	pollerFD: ::std::libc::c_int,
	serverFD: ::std::libc::c_int
}

struct AddrInfo {
	ai_flags    : ::std::libc::c_int,
	ai_family   : ::std::libc::c_int,
	ai_socktype : ::std::libc::c_int,
	ai_protocol : ::std::libc::c_int,
	ai_addrlen  : u32,
	ai_addr     : *SockAddr,
	ai_canonname: *::std::libc::c_char,
	ai_next     : *AddrInfo
}

struct SockAddr {
	sa_family: ::std::libc::c_ushort,
	sa_data  : [::std::libc::c_char, ..14]
}

struct EpollEvent {
	events: u32,
	data  : u64
}


#[no_mangle]
pub extern fn net_init(port: *::std::libc::c_char) -> Net {
	let serverFD = net_initSocket(port);
	let pollerFD = initPoller();

	registerAccept(pollerFD, serverFD);

	Net {
		pollerFD: pollerFD,
		serverFD: serverFD }
}


pub fn net_initSocket(port: *::std::libc::c_char) -> ::std::libc::c_int {
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
				::std::libc::perror(c_str);
			});
			::std::libc::exit(1);
		}

	};

	let socketFD = unsafe{
		let socketFD = socket(
			(*servinfo).ai_family,
			(*servinfo).ai_socktype,
			(*servinfo).ai_protocol);

		if (socketFD == -1) {
			"Error creating socket".to_c_str().with_ref(|c_str| {
				::std::libc::perror(c_str);
			});
			::std::libc::exit(1);
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
			::std::ptr::to_unsafe_ptr(&yes) as *::std::libc::c_void,
			::std::mem::size_of::<::std::libc::c_int>() as u32);

		if status == -1 {
			"Error setting socket option".to_c_str().with_ref(|c_str| {
				::std::libc::perror(c_str);
			});
			::std::libc::exit(1);
		}
	}

	unsafe {
		let status = bind(
			socketFD,
			(*servinfo).ai_addr,
			(*servinfo).ai_addrlen);

		if status != 0 {
			"Error binding socket".to_c_str().with_ref(|c_str| {
				::std::libc::perror(c_str);
			});
			::std::libc::exit(1);
		}
	}

	unsafe {
		let status = listen(
			socketFD,
			1024);
		if status != 0 {
			"Error listening on socket".to_c_str().with_ref(|c_str| {
				::std::libc::perror(c_str);
			});
			::std::libc::exit(1);
		}
	}

	unsafe {
		freeaddrinfo(servinfo);
	}

	socketFD
}

fn initPoller() -> ::std::libc::c_int {
	unsafe {
		let pollerFD = epoll_create(1);
		if pollerFD < 0 {
			"Error initiating epoll".to_c_str().with_ref(|c_str| {
				::std::libc::perror(c_str);
			});
			::std::libc::exit(1);
		}

		pollerFD
	}
}

fn registerAccept(pollerFD: ::std::libc::c_int, serverFD: ::std::libc::c_int) {
	let EPOLLIN = 1;
	let EPOLL_CTL_ADD = 1;

	let event = EpollEvent { events: EPOLLIN, data: 0 };

	unsafe {
		let status = epoll_ctl(pollerFD, EPOLL_CTL_ADD, serverFD, ::std::ptr::to_unsafe_ptr(&event));
		if status != 0 {
			"Error registering server socket with epoll".to_c_str().with_ref(|c_str| {
				::std::libc::perror(c_str);
			});
			::std::libc::exit(1);
		}
	}
}

#[no_mangle]
pub extern fn net_acceptClient(serverFD: ::std::libc::c_int) -> ::std::libc::c_int {
	unsafe {
		accept(
			serverFD,
			::std::ptr::null(),
			::std::ptr::null())
	}
}

pub fn net_send(clientFD: ::std::libc::c_int, message: *::std::libc::c_char, messageLength: ::std::libc::size_t) -> ::std::libc::c_int {
	let MSG_NOSIGNAL = 0x4000;

	unsafe {
		let bytesSent = send(
			clientFD,
			message as *::std::libc::c_void,
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
			::std::libc::exit(1)

		}
		else {
			0
		}
	}
}

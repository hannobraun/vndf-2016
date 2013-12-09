#[crate_type = "rlib"];
#[link(name = "net", package_id = "net", vers = "0.0")];


extern {
	fn epoll_create(size: std::libc::c_int) -> std::libc::c_int;
	fn epoll_ctl(epfd: std::libc::c_int, op: std::libc::c_int, fd: std::libc::c_int, event: *EpollEvent) -> std::libc::c_int;
}


struct Net {
	pollerFD: std::libc::c_int,
	serverFD: std::libc::c_int
}

struct EpollEvent {
	events: u32,
	data  : u64
}


#[no_mangle]
pub extern fn initPoller() -> std::libc::c_int {
	unsafe {
		let pollerFD = epoll_create(1);
		if pollerFD < 0 {
			"Error initiating epoll".to_c_str().with_ref(|c_str| {
				std::libc::perror(c_str);
			});
			std::libc::exit(1);
		}

		pollerFD
	}
}

#[no_mangle]
pub extern fn registerAccept(pollerFD: std::libc::c_int, serverFD: std::libc::c_int) {
	let EPOLLIN = 1;
	let EPOLL_CTL_ADD = 1;

	let event = EpollEvent { events: EPOLLIN, data: 0 };

	unsafe {
		let status = epoll_ctl(pollerFD, EPOLL_CTL_ADD, serverFD, std::ptr::to_unsafe_ptr(&event));
		if status != 0 {
			"Error registering server socket with epoll".to_c_str().with_ref(|c_str| {
				std::libc::perror(c_str);
				std::libc::exit(1);
			})
		}
	}
}

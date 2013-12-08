#[crate_type = "rlib"];
#[link(name = "net", package_id = "net", vers = "0.0")];


extern {
	fn epoll_create(size: std::libc::c_int) -> std::libc::c_int;
}


struct Net {
	pollerFD: std::libc::c_int,
	serverFD: std::libc::c_int
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

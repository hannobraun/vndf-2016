use libc;
use libc::c_int;
use std::c_str::CString;
use std::os;

use net::ffi;


pub fn register_accept(poller_fd: c_int, serverFD: c_int) {
	let event = ffi::epoll_event {
		events: ffi::EPOLLIN,
		data  : 0 };

	let status = unsafe {
		ffi::epoll_ctl(
			poller_fd,
			ffi::EPOLL_CTL_ADD,
			serverFD,
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

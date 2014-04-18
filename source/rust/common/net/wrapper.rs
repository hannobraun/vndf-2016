use libc;
use libc::c_int;

use net::ffi;


pub fn register_accept(pollerFD: c_int, serverFD: c_int) {
	let event = ffi::epoll_event {
		events: ffi::EPOLLIN,
		data  : 0 };

	unsafe {
		let status = ffi::epoll_ctl(
			pollerFD,
			ffi::EPOLL_CTL_ADD,
			serverFD,
			&event);

		if status != 0 {
			"Error registering server socket with epoll".to_c_str().with_ref(|c_message| {
				libc::perror(c_message);
			});
			fail!();
		}
	}
}

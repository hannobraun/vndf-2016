use libc;
use libc::c_int;
use std::c_str::CString;
use std::os;

use net::epoll::ffi;


pub struct EPoll {
	pub epfd: c_int
}

impl EPoll {
	pub fn create() -> Result<EPoll, ~str> {
		let epfd = unsafe {
			ffi::epoll_create(1)
		};

		if epfd >= 0 {
			Ok(EPoll {
				epfd: epfd
			})
		}
		else {
			Err(last_error())
		}
	}

	pub fn add(&self, fd: c_int, events: u32) -> Result<(), ~str> {
		let event = ffi::epoll_event {
			events: events,
			data  : 0
		};

		let status = unsafe {
			ffi::epoll_ctl(
				self.epfd,
				ffi::EPOLL_CTL_ADD,
				fd,
				&event)
		};

		if status == 0 {
			Ok(())
		}
		else {
			Err(last_error())
		}
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

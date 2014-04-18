use libc::c_int;

use net::epoll::ffi;
use util::last_error;


pub struct EPoll {
	epfd        : c_int,
	event_buffer: [ffi::epoll_event, ..1024]
}

impl EPoll {
	pub fn create() -> Result<EPoll, ~str> {
		let epfd = unsafe {
			ffi::epoll_create(1)
		};

		if epfd >= 0 {
			let emptyEvent = ffi::epoll_event {
				events: 0,
				data  : 0 };

			Ok(EPoll {
				epfd        : epfd,
				event_buffer: [emptyEvent, ..1024]
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

	pub fn wait(&self, timeout_in_ms: u32) -> Result<u32, ~str> {
		let number_of_events = unsafe {
			ffi::epoll_wait(
				self.epfd,
				self.event_buffer.as_ptr(),
				self.event_buffer.len() as i32,
				timeout_in_ms as i32)
		};

		if number_of_events >= 0 {
			Ok(number_of_events as u32)
		}
		else {
			Err(last_error())
		}
	}
}

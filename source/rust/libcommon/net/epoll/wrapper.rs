use libc::c_int;
use std::io::{
	IoError,
	IoResult
};

use net::epoll::ffi;


pub struct EPoll {
	pub epfd: c_int,

	event_buffer  : [ffi::epoll_event, ..1024],
	results_buffer: Vec<c_int>
}

impl EPoll {
	pub fn create() -> IoResult<EPoll> {
		let epfd = unsafe {
			ffi::epoll_create(1)
		};

		if epfd >= 0 {
			let emptyEvent = ffi::epoll_event {
				events: 0,
				data  : 0 };

			Ok(EPoll {
				epfd          : epfd,
				event_buffer  : [emptyEvent, ..1024],
				results_buffer: Vec::new()
			})
		}
		else {
			Err(IoError::last_error())
		}
	}

	pub fn add(&self, fd: c_int, events: u32) -> IoResult<()> {
		let mut event = ffi::epoll_event {
			events: events,
			data  : fd as u64
		};

		let status = unsafe {
			ffi::epoll_ctl(
				self.epfd,
				ffi::EPOLL_CTL_ADD,
				fd,
				&mut event)
		};

		if status == 0 {
			Ok(())
		}
		else {
			Err(IoError::last_error())
		}
	}

	pub fn wait<'a>(&'a mut self, timeout_in_ms: u32) -> IoResult<&'a Vec<c_int>> {
		let number_of_events = unsafe {
			ffi::epoll_wait(
				self.epfd,
				self.event_buffer.as_mut_ptr(),
				self.event_buffer.len() as i32,
				timeout_in_ms as i32)
		};

		if number_of_events >= 0 {
			self.results_buffer.clear();

			for i in range(0, number_of_events) {
				self.results_buffer.push(
					self.event_buffer[i as uint].data as c_int);
			}

			Ok(&self.results_buffer)
		}
		else {
			Err(IoError::last_error())
		}
	}
}

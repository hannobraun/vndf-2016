use libc::c_int;


pub static EPOLLIN      : u32   = 1;
pub static EPOLL_CTL_ADD: i32   = 1;


#[allow(non_camel_case_types)]
pub struct epoll_event {
	pub events: u32,
	pub data  : u64
}


extern {
	pub fn epoll_create(size: c_int) -> c_int;

	pub fn epoll_ctl(
		epfd : c_int,
		op   : c_int,
		fd   : c_int,
		event: *epoll_event) -> c_int;

	pub fn epoll_wait(
		epfd     : c_int,
		events   : *epoll_event,
		maxevents: c_int,
		timeout  : c_int) -> c_int;
}

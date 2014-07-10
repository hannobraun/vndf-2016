use libc::c_int;


pub use libc::close;


// epoll_create1 flags
pub static EPOLL_CLOEXEC: c_int = 0o2000000;

// epoll_ctl ops
pub static EPOLL_CTL_ADD: c_int = 1;
pub static EPOLL_CTL_DEL: c_int = 2;
pub static EPOLL_CTL_MOD: c_int = 3;

// event flags
pub static EPOLLIN     : u32 = 0x001;
pub static EPOLLOUT    : u32 = 0x004;
pub static EPOLLRDHUP  : u32 = 0x2000;
pub static EPOLLPRI    : u32 = 0x002;
pub static EPOLLERR    : u32 = 0x008;
pub static EPOLLHUP    : u32 = 0x010;
pub static EPOLLET     : u32 = 1 << 31;
pub static EPOLLONESHOT: u32 = 1 << 30;



#[allow(non_camel_case_types)]
pub struct epoll_event {
	pub events: u32,
	pub data  : epoll_data
}

#[allow(non_camel_case_types)]
pub type epoll_data = u64;


extern {
	pub fn epoll_create(size: c_int) -> c_int;

	pub fn epoll_create1(flags: c_int) -> c_int;

	pub fn epoll_ctl(
		epfd : c_int,
		op   : c_int,
		fd   : c_int,
		event: *mut epoll_event) -> c_int;

	pub fn epoll_wait(
		epfd     : c_int,
		events   : *mut epoll_event,
		maxevents: c_int,
		timeout  : c_int) -> c_int;
}

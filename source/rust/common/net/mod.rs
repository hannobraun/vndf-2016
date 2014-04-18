use libc;
use libc::types::os::common::bsd44;


pub use libc::consts::os::bsd44::{
	SOCK_STREAM,
	SOL_SOCKET};
pub use libc::consts::os::bsd44::SO_REUSEADDR;

pub static AI_PASSIVE: libc::c_int = 1;
pub static AF_UNSPEC : libc::c_int = 0;


extern {
	pub fn getaddrinfo(
		name   : *libc::c_char,
		service: *libc::c_char,
		req    : *bsd44::addrinfo,
		pai    : **bsd44::addrinfo) -> libc::c_int;

	pub fn freeaddrinfo(res: *bsd44::addrinfo);
}

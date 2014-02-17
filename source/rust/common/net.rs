use std::libc;
use std::libc::types::os::common::bsd44;


pub static AI_PASSIVE : libc::c_int = 1;
pub static AF_UNSPEC  : libc::c_int = 0;
pub static SOCK_STREAM: libc::c_int = 1;


extern {
	pub fn getaddrinfo(
		name   : *libc::c_char,
		service: *libc::c_char,
		req    : *bsd44::addrinfo,
		pai    : **bsd44::addrinfo) -> libc::c_int;

	pub fn freeaddrinfo(res: *bsd44::addrinfo);
}

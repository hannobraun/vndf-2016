use std::libc;
use std::libc::types::os::common::bsd44;


pub static AI_PASSIVE : libc::c_int = 1;
pub static AF_UNSPEC  : libc::c_int = 0;
pub static SOCK_STREAM: libc::c_int = 1;


pub struct AddrInfo {
	ai_flags    : libc::c_int,
	ai_family   : libc::c_int,
	ai_socktype : libc::c_int,
	ai_protocol : libc::c_int,
	ai_addrlen  : u32,
	ai_addr     : *bsd44::sockaddr,
	ai_canonname: *libc::c_char,
	ai_next     : *AddrInfo
}


extern {
	pub fn getaddrinfo(
		name   : *libc::c_char,
		service: *libc::c_char,
		req    : *AddrInfo,
		pai    : **AddrInfo) -> libc::c_int;

	pub fn freeaddrinfo(res: *AddrInfo);

	pub fn socket(
		domain  : libc::c_int,
		theType : libc::c_int,
		protocol: libc::c_int) -> libc::c_int;
}

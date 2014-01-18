use std::libc;


pub static AI_PASSIVE : libc::c_int = 1;
pub static AF_UNSPEC  : libc::c_int = 0;
pub static SOCK_STREAM: libc::c_int = 1;


pub struct AddrInfo {
	ai_flags    : libc::c_int,
	ai_family   : libc::c_int,
	ai_socktype : libc::c_int,
	ai_protocol : libc::c_int,
	ai_addrlen  : u32,
	ai_addr     : *SockAddr,
	ai_canonname: *libc::c_char,
	ai_next     : *AddrInfo
}

pub struct SockAddr {
	sa_family: libc::c_ushort,
	sa_data  : [libc::c_char, ..14]
}


extern {
	pub fn getaddrinfo(
		name   : *libc::c_char,
		service: *libc::c_char,
		req    : *AddrInfo,
		pai    : **AddrInfo) -> libc::c_int;
}

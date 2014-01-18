use std::libc;


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

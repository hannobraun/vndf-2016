use libc::{
	c_char,
	c_int};
use libc::types::os::common::bsd44;


pub use libc::consts::os::bsd44::{
	SOCK_STREAM,
	SOL_SOCKET};
pub use libc::consts::os::bsd44::SO_REUSEADDR;
pub use libc::consts::os::posix88::{
	EAGAIN,
	EWOULDBLOCK};
pub use libc::funcs::bsd43::{
	accept,
	bind,
	connect,
	listen,
	recv,
	send,
	setsockopt,
	socket};
pub use libc::types::os::common::bsd44::{
	addrinfo,
	sockaddr};


pub static AF_UNSPEC    : c_int = 0;
pub static AI_PASSIVE   : c_int = 1;
pub static MSG_DONTWAIT : i32   = 0x40;
pub static MSG_NOSIGNAL : i32   = 0x4000;


extern {
	pub fn getaddrinfo(
		name   : *c_char,
		service: *c_char,
		req    : *bsd44::addrinfo,
		pai    : **bsd44::addrinfo) -> c_int;

	pub fn freeaddrinfo(res: *bsd44::addrinfo);
}

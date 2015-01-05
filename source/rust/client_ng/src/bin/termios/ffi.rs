#![allow(non_camel_case_types)]

use libc::{
	c_int,
	c_uchar,
	c_uint,
};


pub type cc_t     = c_uchar;
pub type speed_t  = c_uint;
pub type tcflag_t = c_uint;


extern {
	pub fn tcgetattr(fd: c_int, termios: *mut termios) -> c_int;
	pub fn tcsetattr(fd: c_int, actions: c_int, termios: *const termios) -> c_int;
}


pub const ECHO   : tcflag_t = 0x08;
pub const FAILURE: c_int    = -1;
pub const ICANON : tcflag_t = 0x02;
pub const NCCS   : uint     = 32;
pub const SUCCESS: c_int    = 0;
pub const TCSANOW: c_int    = 0;


#[repr(C)]
pub struct termios {
	pub c_iflag : tcflag_t,
	pub c_oflag : tcflag_t,
	pub c_cflag : tcflag_t,
	pub c_lflag : tcflag_t,
	pub c_line  : cc_t,
	pub c_cc    : [cc_t; NCCS],
	pub c_ispeed: speed_t,
	pub c_ospeed: speed_t,
}

use libc::c_int;

use super::ffi::{
	FAILURE,
	NCCS,
	SUCCESS,
	termios,
	tcgetattr,
};


pub struct Termios {
	pub termios: termios,
}

impl Termios {
	pub fn get(fd: c_int) -> Termios {
		let mut termios = termios {
			c_iflag : 0,
			c_oflag : 0,
			c_cflag : 0,
			c_lflag : 0,
			c_line  : 0,
			c_cc    : [0, ..NCCS],
			c_ispeed: 0,
			c_ospeed: 0,
		};

		match unsafe { tcgetattr(fd, &mut termios) } {
			FAILURE => panic!("Error getting term attributes"),
			SUCCESS => (),
			_       => unreachable!(),
		}

		Termios {
			termios: termios,
		}
	}
}

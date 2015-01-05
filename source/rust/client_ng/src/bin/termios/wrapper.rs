use libc::c_int;

use super::ffi::{
	ECHO,
	FAILURE,
	ICANON,
	NCCS,
	SUCCESS,
	TCSANOW,
	tcflag_t,
	termios,
	tcgetattr,
	tcsetattr,
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
			c_cc    : [0; NCCS],
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

	pub fn set(&self, fd: c_int) {
		match unsafe { tcsetattr(fd, TCSANOW, &self.termios) } {
			FAILURE => panic!("Error setting term attributes"),
			SUCCESS => (),
			_       => unreachable!(),
		}
	}

	pub fn echo(&mut self, set: bool) {
		apply(&mut self.termios.c_lflag, ECHO, set);
	}

	pub fn canonical_input(&mut self, set: bool) {
		apply(&mut self.termios.c_lflag, ICANON, set);
	}
}


fn apply(flags: &mut tcflag_t, flag: tcflag_t, set: bool) {
	match set {
		true  => *flags &= flag,
		false => *flags &= !flag,
	}
}

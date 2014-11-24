extern crate libc;


use std::io::timer::sleep;
use std::time::Duration;

use termios::ffi::{
	mod,
	ECHO,
	FAILURE,
	ICANON,
	NCCS,
	SUCCESS,
	TCSANOW,
	tcgetattr,
	tcsetattr,
};


mod termios;


fn main() {
	let mut termios = ffi::termios {
		c_iflag : 0,
		c_oflag : 0,
		c_cflag : 0,
		c_lflag : 0,
		c_line  : 0,
		c_cc    : [0, ..NCCS],
		c_ispeed: 0,
		c_ospeed: 0,
	};

	match unsafe { tcgetattr(libc::STDIN_FILENO, &mut termios) } {
		FAILURE => panic!("Error getting term attributes"),
		SUCCESS => (),
		_       => unreachable!(),
	}

	termios.c_lflag &= !ICANON & !ECHO; // disable line buffering and echo
	match unsafe { tcsetattr(libc::STDIN_FILENO, TCSANOW, &termios) } {
		FAILURE => panic!("Error setting term attributes"),
		SUCCESS => (),
		_       => unreachable!(),
	}

	let mut i = 0u8;

	print!("\n");
	loop {
		print!("\x1b[1A{}\n", i);

		i += 1;
		sleep(Duration::milliseconds(200));
	}
}

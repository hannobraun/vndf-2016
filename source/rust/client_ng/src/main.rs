extern crate libc;


use std::io::timer::sleep;
use std::time::Duration;

use termios::Termios;
use termios::ffi::{
	ECHO,
	FAILURE,
	ICANON,
	SUCCESS,
	TCSANOW,
	tcsetattr,
};


mod termios;


fn main() {
	let mut termios = Termios::get(libc::STDIN_FILENO);

	termios.termios.c_lflag &= !ICANON & !ECHO; // disable line buffering and echo
	match unsafe { tcsetattr(libc::STDIN_FILENO, TCSANOW, &termios.termios) } {
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

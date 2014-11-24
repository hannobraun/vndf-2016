extern crate libc;


use std::io::timer::sleep;
use std::time::Duration;

use termios::Termios;


mod termios;


fn main() {
	let mut termios = Termios::get(libc::STDIN_FILENO);
	termios.echo(false);
	termios.canonical_input(false);
	termios.set(libc::STDIN_FILENO);

	let mut i = 0u8;

	print!("\n");
	loop {
		print!("\x1b[1A{}\n", i);

		i += 1;
		sleep(Duration::milliseconds(200));
	}
}

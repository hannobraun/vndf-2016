use libc;
use std::io::{
	stdout,
	IoResult,
	LineBufferedWriter,
};
use std::io::stdio::StdWriter;

use termios::Termios;


pub struct Screen {
	stdout: LineBufferedWriter<StdWriter>,
}

impl Screen {
	pub fn new() -> Screen {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		Screen {
			stdout: stdout(),
		}
	}

	pub fn clear(&mut self) -> IoResult<()> {
		try!(write!(&mut self.stdout, "\x1b[2J\x1b[H"));
		Ok(())
	}

	pub fn buffer(&mut self) -> &mut LineBufferedWriter<StdWriter> {
		&mut self.stdout
	}

	pub fn submit(&mut self) -> IoResult<()> {
		try!(self.stdout.flush());
		Ok(())
	}
}

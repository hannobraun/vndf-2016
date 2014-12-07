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
	pub fn new(_width: u16, _height: u16) -> Screen {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		Screen {
			stdout: stdout(),
		}
	}

	pub fn clear(&mut self) -> IoResult<()> {
		try!(write!(&mut self.stdout, "\x1b[2J")); // clear screen
		Ok(())
	}

	/// Origin is in upper-left corner.
	pub fn buffer(
		&mut self,
		x: u16,
		y: u16,
	) -> &mut LineBufferedWriter<StdWriter> {
		// TODO: Improve error handling
		(write!(
			&mut self.stdout,
			"\x1b[{};{}H",
			y + 1, x + 1
		)).unwrap(); // set cursor

		&mut self.stdout
	}

	pub fn submit(&mut self) -> IoResult<()> {
		try!(self.stdout.flush());
		Ok(())
	}
}

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
		try!(write!(&mut self.stdout, "\x1b[2J")); // clear screen
		try!(write!(&mut self.stdout, "\x1b[H")); // reset cursor
		Ok(())
	}

	/// Origin is in upper-left corner.
	pub fn buffer_at(
		&mut self,
		x: u16,
		y: u16,
	) -> &mut LineBufferedWriter<StdWriter> {
		// TODO: Improve error handling
		(write!(&mut self.stdout, "\x1b[H")).unwrap(); // reset cursor
		(write!(&mut self.stdout, "\x1b[{}C", x + 1)).unwrap(); // cursor right
		(write!(&mut self.stdout, "\x1b[{}B", y + 1)).unwrap(); // cursor down

		&mut self.stdout
	}

	pub fn submit(&mut self) -> IoResult<()> {
		try!(self.stdout.flush());
		Ok(())
	}
}

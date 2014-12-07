use libc;
use std::io::{
	stdout,
	IoError,
	IoErrorKind,
	IoResult,
	LineBufferedWriter,
};
use std::io::stdio::StdWriter;
use std::str::from_utf8;

use termios::Termios;


pub struct Screen {
	stdout: LineBufferedWriter<StdWriter>,
	buffer: ScreenBuffer,
	cursor: (u16, u16),
}

impl Screen {
	pub fn new(width: u16, height: u16) -> Screen {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		let width  = width  as uint;
		let height = height as uint;

		let buffer = Vec::from_fn(height, |_| Vec::from_elem(width, ' '));

		Screen {
			stdout: stdout(),
			buffer: buffer,
			cursor: (0, 0),
		}
	}

	pub fn clear(&mut self) -> IoResult<()> {
		for line in self.buffer.iter_mut() {
			for c in line.iter_mut() {
				*c = ' ';
			}
		}

		Ok(())
	}

	/// Origin is in upper-left corner.
	pub fn buffer(&mut self, x: u16, y: u16) -> BufferWriter {
		BufferWriter {
			buffer: &mut self.buffer,
			x     : x,
			y     : y,
		}
	}

	pub fn set_cursor(&mut self, x: u16, y: u16) {
		self.cursor = (x, y);
	}

	pub fn submit(&mut self) -> IoResult<()> {
		try!(write!(&mut self.stdout, "\x1b[2J")); // clear screen
		try!(write!(&mut self.stdout, "\x1b[H")); // reset cursor

		for line in self.buffer.iter() {
			for &c in line.iter() {
				try!(self.stdout.write_char(c));
			}
			try!(self.stdout.write_char('\n'));
		}

		let (x, y) = self.cursor;
		try!(write!(
			&mut self.stdout,
			"\x1b[{};{}H",
			y + 1, x + 1
		)); // set cursor

		try!(self.stdout.flush());
		Ok(())
	}
}


type ScreenBuffer = Vec<Vec<char>>;


struct BufferWriter<'r> {
	buffer: &'r mut ScreenBuffer,
	x     : u16,
	y     : u16,
}

impl<'r> Writer for BufferWriter<'r> {
	fn write(&mut self, buf: &[u8]) -> IoResult<()> {
		let s = match from_utf8(buf) {
			Some(s) =>
				s,
			None =>
				return Err(IoError {
					kind  : IoErrorKind::OtherIoError,
					desc  : "Tried to write invalid UTF-8",
					detail: None,
				})

		};

		for c in s.chars() {
			// TODO: Check bounds
			let x = self.x as uint;
			let y = self.y as uint;
			self.buffer[y][x] = c;

			self.x += 1;
		}

		Ok(())
	}
}

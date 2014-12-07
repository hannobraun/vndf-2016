use libc;
use std::io::{
	stdout,
	IoError,
	IoErrorKind,
	IoResult,
	LineBufferedWriter,
};
use std::io::stdio::StdWriter;
use std::mem::swap;
use std::str::from_utf8;

use termios::Termios;


pub struct Screen {
	stdout  : LineBufferedWriter<StdWriter>,
	buffer_a: ScreenBuffer,
	buffer_b: ScreenBuffer,
	cursor  : (u16, u16),
}

impl Screen {
	pub fn new(width: u16, height: u16) -> IoResult<Screen> {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		let mut stdout = stdout();
		match write!(&mut stdout, "\x1b[2J") { // clear screen
			Ok(())     => (),
			Err(error) => return Err(error),
		}

		let width  = width  as uint;
		let height = height as uint;

		let buffer_a = Vec::from_fn(height, |_| Vec::from_elem(width, ' '));
		let buffer_b = buffer_a.clone();

		Ok(Screen {
			stdout  : stdout,
			buffer_a: buffer_a,
			buffer_b: buffer_b,
			cursor  : (0, 0),
		})
	}

	/// Origin is in upper-left corner.
	pub fn buffer(&mut self, x: u16, y: u16) -> BufferWriter {
		BufferWriter {
			buffer: &mut self.buffer_a,
			x     : x,
			y     : y,
		}
	}

	pub fn set_cursor(&mut self, x: u16, y: u16) {
		self.cursor = (x, y);
	}

	pub fn submit(&mut self) -> IoResult<()> {
		for (y, line) in self.buffer_a.iter().enumerate() {
			for (x, &c) in line.iter().enumerate() {
				if c != self.buffer_b[y][x] {
					try!(write!(
						&mut self.stdout,
						"\x1b[{};{}H", // move cursor
						y + 1, x + 1
					));
					try!(self.stdout.write_char(c));
				}
			}
		}

		swap(&mut self.buffer_a, &mut self.buffer_b);

		for line in self.buffer_a.iter_mut() {
			for c in line.iter_mut() {
				*c = ' ';
			}
		}

		let (x, y) = self.cursor;
		try!(write!(
			&mut self.stdout,
			"\x1b[{};{}H", // set cursor
			y + 1, x + 1
		));

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

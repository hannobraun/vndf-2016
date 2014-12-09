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

use super::{
	C,
	Color,
	Pos,
};


pub struct Screen {
	stdout  : LineBufferedWriter<StdWriter>,
	buffer_a: ScreenBuffer,
	buffer_b: ScreenBuffer,
	cursor  : (Pos, Pos),
	bold    : bool,
	color   : Color,
}

impl Screen {
	pub fn new(width: Pos, height: Pos) -> IoResult<Screen> {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		let mut stdout = stdout();
		match write!(&mut stdout, "\x1b[2J") { // clear screen
			Ok(())     => (),
			Err(error) => return Err(error),
		}

		let buffer_a = ScreenBuffer::new(width, height);
		let buffer_b = buffer_a.clone();

		Ok(Screen {
			stdout  : stdout,
			buffer_a: buffer_a,
			buffer_b: buffer_b,
			cursor  : (0, 0),
			bold    : false,
			color   : Color::default(),
		})
	}

	pub fn width(&self) -> Pos {
		self.buffer_a.width()
	}

	/// Origin is in upper-left corner.
	pub fn buffer(&mut self, x: Pos, y: Pos, limit: Pos) -> BufferWriter {
		BufferWriter {
			buffer: &mut self.buffer_a,
			x     : x,
			y     : y,
			limit : limit,
			bold  : self.bold,
			color : self.color,
		}
	}

	pub fn set_bold(&mut self, bold: bool) -> bool {
		let previous_value = self.bold;
		self.bold = bold;
		previous_value
	}

	pub fn set_color(&mut self, color: Color) -> Color {
		let previous_value = self.color;
		self.color = color;
		previous_value
	}

	pub fn set_cursor(&mut self, x: Pos, y: Pos) {
		self.cursor = (x, y);
	}

	pub fn submit(&mut self) -> IoResult<()> {
		{
			let mut iter = self.buffer_a.iter().zip(self.buffer_b.iter());
			for ((x, y, c_a), (_, _, c_b)) in iter {
				if c_a != c_b {
					try!(write!(
						&mut self.stdout,
						"\x1b[{};{}H", // move cursor
						y + 1, x + 1,
					));
					if c_a.bold {
						try!(write!(
							&mut self.stdout,
							"\x1b[1m", // set bold
						));
					}
					try!(write!(
						&mut self.stdout,
						"\x1b[{}m", // set foreground color
						c_a.color.foreground_code(),
					));
					try!(self.stdout.write_char(c_a.c));
					if c_a.bold {
						try!(write!(
							&mut self.stdout,
							"\x1b[0m", // reset attributes
						));
					}
				}
			}
		}

		swap(&mut self.buffer_a, &mut self.buffer_b);
		self.buffer_a.clear();

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


#[deriving(Clone)]
struct ScreenBuffer {
	buffer: Vec<Vec<C>>,
}

impl ScreenBuffer {
	pub fn new(width: Pos, height: Pos) -> ScreenBuffer {
		let width  = width  as uint;
		let height = height as uint;

		ScreenBuffer {
			buffer: Vec::from_fn(height, |_| Vec::from_elem(width, C::new()))
		}
	}

	pub fn width(&self) -> Pos {
		self.buffer[0].len() as Pos
	}

	pub fn height(&self) -> Pos {
		self.buffer.len() as Pos
	}

	pub fn iter(&self) -> BufferIterator {
		BufferIterator {
			buffer: &self.buffer,
			x     : 0,
			y     : 0,
		}
	}

	pub fn clear(&mut self) {
		for line in self.buffer.iter_mut() {
			for c in line.iter_mut() {
				c.c = ' ';
			}
		}
	}
}


struct BufferIterator<'r> {
	buffer: &'r Vec<Vec<C>>,
	x     : uint,
	y     : uint,
}

impl<'r> Iterator<(Pos, Pos, C)> for BufferIterator<'r> {
	fn next(&mut self) -> Option<(Pos, Pos, C)> {
		if self.x >= self.buffer[0].len() {
			self.x  = 0;
			self.y += 1;
		}

		if self.y >= self.buffer.len() {
			return None;
		}

		let result =
			Some((self.x as Pos, self.y as Pos, self.buffer[self.y][self.x]));

		self.x += 1;

		result
	}
}


struct BufferWriter<'r> {
	buffer: &'r mut ScreenBuffer,
	x     : Pos,
	y     : Pos,
	limit : Pos,
	bold  : bool,
	color : Color,
}

impl<'r> Writer for BufferWriter<'r> {
	fn write(&mut self, buf: &[u8]) -> IoResult<()> {
		if self.y >= self.buffer.height() {
			return Err(IoError {
				kind  : IoErrorKind::OtherIoError,
				desc  : "y coordinate is out of bounds",
				detail: None,
			})
		}

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
			if self.x >= self.limit || self.x >= self.buffer.width() {
				// Truncate everything beyond the limit
				break;
			}

			let x = self.x as uint;
			let y = self.y as uint;
			self.buffer.buffer[y][x] = C {
				c    : c,
				bold : self.bold,
				color: self.color,
			};

			self.x += 1;
		}

		Ok(())
	}
}

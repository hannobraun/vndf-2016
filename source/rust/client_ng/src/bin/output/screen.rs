use libc;
use std::io::{
	stdout,
	IoResult,
	LineBufferedWriter,
};
use std::io::stdio::StdWriter;
use std::mem::swap;

use termios::Termios;

use super::{
	Color,
	Pos,
};
use super::buffer::{
	BufferWriter,
	ScreenBuffer,
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

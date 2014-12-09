use std::io::{
	IoError,
	IoErrorKind,
	IoResult,
};
use std::str::from_utf8;

use super::{
	Color,
	Pos,
};


#[deriving(Clone, Eq, PartialEq)]
pub struct C {
	pub c    : char,
	pub bold : bool,
	pub color: Color,
}

impl C {
	fn new() -> C {
		C {
			c    : ' ',
			bold : false,
			color: Color::default(),
		}
	}
}


#[deriving(Clone)]
pub struct ScreenBuffer {
	buffer: Vec<Vec<C>>,

	pub bold: bool,
}

impl ScreenBuffer {
	pub fn new(width: Pos, height: Pos) -> ScreenBuffer {
		let width  = width  as uint;
		let height = height as uint;

		ScreenBuffer {
			buffer: Vec::from_fn(height, |_| Vec::from_elem(width, C::new())),
			bold  : false,
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


pub struct BufferIterator<'r> {
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


pub struct BufferWriter<'r> {
	pub buffer: &'r mut ScreenBuffer,
	pub x     : Pos,
	pub y     : Pos,
	pub limit : Pos,
	pub bold  : bool,
	pub color : Color,
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

use std::old_io::IoResult;

use super::{
	Pos,
	Screen,
};
use super::buffer::{
	C,
	ScreenBuffer,
};


pub struct Section {
	pub buffer: ScreenBuffer,
	pub width : Pos,
	pub height: Pos,
}

impl Section {
	pub fn new(width: Pos, height: Pos) -> Section {
		let inner_width  = width  - 2;
		let inner_height = height - 2;

		Section {
			buffer: ScreenBuffer::new(inner_width, inner_height),
			width : width,
			height: height,
		}
	}

	pub fn write(&self, x: Pos, y: Pos, screen: &mut Screen) -> IoResult<()> {
		for (buffer_x, buffer_y, c) in self.buffer.iter() {
			try!(screen.buffer().set(x + buffer_x + 1, y + buffer_y + 1, c));
		}

		if let Some(cursor) = self.buffer.cursor {
			screen.cursor(Some(cursor));
		}

		let mut c = C::new();

		c.c = '┏';
		try!(screen.buffer().set(x +              0, y +               0, c));
		c.c = '┓';
		try!(screen.buffer().set(x + self.width - 1, y +               0, c));
		c.c = '┗';
		try!(screen.buffer().set(x +              0, y + self.height - 1, c));
		c.c = '┛';
		try!(screen.buffer().set(x + self.width - 1, y + self.height - 1, c));

		c.c = '━';
		for rel_x in range(1, self.width - 1) {
			for &rel_y in [0, self.height - 1].iter() {
				try!(screen.buffer().set(x + rel_x, y + rel_y, c));
			}
		}

		c.c = '┃';
		for &rel_x in [0, self.width - 1].iter() {
			for rel_y in range(1, self.height - 1) {
				try!(screen.buffer().set(x + rel_x, y + rel_y, c));
			}
		}

		Ok(())
	}
}

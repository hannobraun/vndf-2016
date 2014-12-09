use std::io::IoResult;

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
		let inner_width  = width  - 1;
		let inner_height = height - 1;

		Section {
			buffer: ScreenBuffer::new(inner_width, inner_height),
			width : width,
			height: height,
		}
	}

	pub fn write(&self, screen: &mut Screen) -> IoResult<()> {
		for (x, y, c) in self.buffer.iter() {
			// TODO: Take position into account
			try!(screen.buffer().set(x + 1, y + 1, c));
		}

		let mut c = C::new();

		c.c = '┏';
		try!(screen.buffer().set(             0,               0, c));
		c.c = '┓';
		try!(screen.buffer().set(self.width - 1,               0, c));
		c.c = '┗';
		try!(screen.buffer().set(             0, self.height - 1, c));
		c.c = '┛';
		try!(screen.buffer().set(self.width - 1, self.height - 1, c));

		c.c = '━';
		for x in range(1, self.width - 1) {
			for &y in [0, self.height - 1].iter() {
				try!(screen.buffer().set(x, y, c));
			}
		}

		c.c = '┃';
		for &x in [0, self.width - 1].iter() {
			for y in range(1, self.height - 1) {
				try!(screen.buffer().set(x, y, c));
			}
		}

		Ok(())
	}
}

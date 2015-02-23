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

		try!(draw_border(screen.buffer(), x, y, self.width, self.height));

		Ok(())
	}
}


pub fn draw_border(
	buffer: &mut ScreenBuffer,
	x     : Pos,
	y     : Pos,
	width : Pos,
	height: Pos
) -> IoResult<()> {
	let mut c = C::new();

	c.c = '┏';
	try!(buffer.set(x +         0, y +          0, c));
	c.c = '┓';
	try!(buffer.set(x + width - 1, y +          0, c));
	c.c = '┗';
	try!(buffer.set(x +         0, y + height - 1, c));
	c.c = '┛';
	try!(buffer.set(x + width - 1, y + height - 1, c));

	c.c = '━';
	for rel_x in range(1, width - 1) {
		for &rel_y in [0, height - 1].iter() {
			try!(buffer.set(x + rel_x, y + rel_y, c));
		}
	}

	c.c = '┃';
	for &rel_x in [0, width - 1].iter() {
		for rel_y in range(1, height - 1) {
			try!(buffer.set(x + rel_x, y + rel_y, c));
		}
	}

	Ok(())
}

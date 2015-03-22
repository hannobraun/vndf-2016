use std::io;

use super::Pos;
use super::buffer::{
	C,
	ScreenBuffer,
};


pub fn draw_border(
	buffer: &mut ScreenBuffer,
	x     : Pos,
	y     : Pos,
	width : Pos,
	height: Pos
) -> io::Result<()> {
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
	for rel_x in (1 .. width - 1) {
		for &rel_y in [0, height - 1].iter() {
			try!(buffer.set(x + rel_x, y + rel_y, c));
		}
	}

	c.c = '┃';
	for &rel_x in [0, width - 1].iter() {
		for rel_y in (1 .. height - 1) {
			try!(buffer.set(x + rel_x, y + rel_y, c));
		}
	}

	Ok(())
}

use std::io::IoResult;

use super::{
	Pos,
	Screen,
};
use super::buffer::ScreenBuffer;


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
		// TODO: Write border

		Ok(())
	}
}

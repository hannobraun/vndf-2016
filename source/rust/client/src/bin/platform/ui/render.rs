use std::io::IoResult;

use render::ScreenBuffer;


pub trait Render<E, D> {
	fn render(&mut self, b: &mut ScreenBuffer, element: &E, data: &D)
		-> IoResult<()>;
}

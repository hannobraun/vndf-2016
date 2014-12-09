use std::io::IoResult;

use client::output::Frame;


pub use self::color::Color;
pub use self::headless::HeadlessRenderer;
pub use self::renderer::Renderer;
pub use self::screen::Screen;


mod color;
mod headless;
mod renderer;
mod screen;


pub trait Render {
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}


pub type Pos = u16;


#[deriving(Clone, Eq, PartialEq)]
struct C {
	c    : char,
	bold : bool,
	color: Color,
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

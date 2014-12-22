use std::io::IoResult;

use client::platform::Frame;


pub use self::buffer::ScreenBuffer;
pub use self::color::Color;
pub use self::renderer::Renderer;
pub use self::screen::Screen;


mod buffer;
mod color;
mod renderer;
mod screen;
mod util;


pub trait Render {
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}


pub type Pos = u16;

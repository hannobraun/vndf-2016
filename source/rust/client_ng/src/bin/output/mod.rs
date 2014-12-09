use std::io::IoResult;

use client::output::Frame;


pub use self::headless::HeadlessRenderer;
pub use self::renderer::Renderer;


mod color;
mod headless;
mod renderer;
mod screen;


pub trait Render {
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}

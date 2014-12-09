use std::io::IoResult;

use client::output::Frame;


pub use self::renderer::Renderer;


mod color;
mod renderer;
mod screen;


pub trait Output {
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}


pub struct HeadlessRenderer;

impl HeadlessRenderer {
	pub fn new() -> IoResult<HeadlessRenderer> {
		Ok(HeadlessRenderer)
	}
}

impl Output for HeadlessRenderer {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		print!("{}\n", frame.to_json());
		Ok(())
	}
}
